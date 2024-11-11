use crate::routes::bookings::bookings_model::{
    Booking, BookingStatus, GetBookingsQuery, RequestBooking,
};
use crate::routes::pricing::pricing_model::CalculatePriceRequest;
use crate::routes::pricing::pricing_service::calculate_price;
use crate::routes::transactions::transactions_model::TransactionType;
use crate::shared::types::{PaginatedResponse, PaginationMeta};
use crate::utilities::database::db_executor::DbExecutor;
use anyhow::Context;
use sqlx::postgres::PgRow;
use sqlx::{QueryBuilder, Row};
use time::OffsetDateTime;
use uuid::Uuid;

#[tracing::instrument(name = "Create booking in database", skip(executor))]
pub async fn create_booking_in_database<'e>(
    request: RequestBooking,
    executor: &mut DbExecutor<'e>,
) -> Result<Uuid, anyhow::Error> {
    let booking_id = Uuid::new_v4();
    let transaction_id = request.transaction_id.expect("Transaction id is required");

    let booking_status = match request.transaction_type {
        TransactionType::External => BookingStatus::Confirmed,
        _ => BookingStatus::Requested,
    };

    let total = match request.transaction_type {
        TransactionType::External => request.total.expect("Total is required"),
        _ => {
            let pricing = calculate_price(
                CalculatePriceRequest {
                    rental_id: request.rental_id,
                    vendor_id: request.vendor_id,
                    pricing_id: request.pricing_id.expect("Pricing id is required"),
                    start_date: request.start_date,
                    end_date: request.end_date,
                    quantity: request.quantity,
                },
                executor,
            )
            .await?;
            pricing.total
        }
    };

    let query = sqlx::query!(
        r#"
        INSERT INTO bookings (
            booking_id,
            transaction_id,
            rental_id,
            vendor_id,
            pricing_id,
            quantity,
            start_date,
            end_date,
            booking_status,
            total
        )
        VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7,
            $8,
            $9,
            $10
        )
        "#,
        booking_id,
        transaction_id,
        request.rental_id,
        request.vendor_id,
        request.pricing_id,
        request.quantity,
        request.start_date,
        request.end_date,
        booking_status as BookingStatus,
        total
    );

    match executor {
        DbExecutor::Transaction(transaction) => query.execute(&mut **transaction).await,
        DbExecutor::Pool(pool) => query.execute(*pool).await,
    }
    .context("Failed to create new booking in the database.")?;

    Ok(booking_id)
}

#[tracing::instrument(name = "Get all bookings from database by query", skip(executor))]
pub async fn get_bookings_from_database_by_query<'e>(
    query_params: &GetBookingsQuery,
    executor: &mut DbExecutor<'e>,
) -> Result<PaginatedResponse<Booking>, anyhow::Error> {
    let sql = r#"
            SELECT
                booking_id,
                created_at,
                updated_at,
                transaction_id,
                rental_id,
                vendor_id,
                pricing_id,
                quantity,
                start_date,
                end_date,
                booking_status,
                total,
                COUNT(*) OVER() AS total_count
            FROM bookings
            WHERE 1 = 1
    "#;

    let mut query = QueryBuilder::new(sql);

    if let Some(transaction_ids) = &query_params.transaction_ids {
        if !transaction_ids.is_empty() {
            query.push(" AND transaction_id = ANY(");
            query.push_bind(transaction_ids);
            query.push(")");
        }
    }

    if let Some(rental_id) = &query_params.rental_id {
        query.push(" AND rental_id = ");
        query.push_bind(rental_id);
    }

    if let Some(vendor_id) = &query_params.vendor_id {
        query.push(" AND vendor_id = ");
        query.push_bind(vendor_id);
    }

    if let Some(booking_status) = &query_params.booking_status {
        query.push(" AND booking_status = ");
        query.push_bind(booking_status);
    }

    if let Some(start_date) = &query_params.start_date {
        query.push(" AND end_date >= ");
        query.push_bind(start_date);
    }

    if let Some(end_date) = &query_params.end_date {
        query.push(" AND start_date <= ");
        query.push_bind(end_date);
    }

    query.push(" ORDER BY start_date DESC"); // TODO: Replace with the SORT_BY heuristic

    let page = query_params.page.unwrap_or(1);
    let per_page = query_params.per_page.unwrap_or(20);
    query.push(" LIMIT ");
    query.push_bind(per_page);
    query.push(" OFFSET ");
    query.push_bind((page - 1) * per_page);

    let query = query.build();

    let rows = match executor {
        DbExecutor::Transaction(transaction) => query.fetch_all(&mut **transaction).await,
        DbExecutor::Pool(pool) => query.fetch_all(*pool).await,
    }
    .context("Failed to perform a query to get all bookings based on query parameters")?;

    let total_count = if let Some(row) = rows.first() {
        row.get::<i64, _>("total_count")
    } else {
        0
    };

    let bookings: Vec<Booking> = rows
        .into_iter()
        .map(|row: PgRow| Booking {
            booking_id: row.get("booking_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            transaction_id: row.get("transaction_id"),
            rental_id: row.get("rental_id"),
            vendor_id: row.get("vendor_id"),
            pricing_id: row.get("pricing_id"),
            quantity: row.get("quantity"),
            start_date: row.get("start_date"),
            end_date: row.get("end_date"),
            booking_status: row.get("booking_status"),
            total: row.get("total"),
            rental: None,
            available: None,
        })
        .collect();

    Ok(PaginatedResponse {
        data: bookings,
        meta: PaginationMeta {
            total_count,
            page,
            per_page,
        },
    })
}

#[tracing::instrument(name = "Get booking from database by booking id", skip(executor))]
pub async fn get_booking_from_database_by_booking_id<'e>(
    booking_id: &Uuid,
    executor: &mut DbExecutor<'e>,
) -> Result<Option<Booking>, anyhow::Error> {
    let query = sqlx::query!(
        r#"
        SELECT
            booking_id,
            created_at,
            updated_at,
            transaction_id,
            rental_id,
            vendor_id,
            pricing_id,
            quantity,
            start_date,
            end_date,
            booking_status as "booking_status: BookingStatus",
            total
        FROM bookings
        WHERE booking_id = $1
        "#,
        booking_id,
    );

    let booking: Option<Booking> = match executor {
        DbExecutor::Transaction(transaction) => query.fetch_optional(&mut **transaction).await,
        DbExecutor::Pool(pool) => query.fetch_optional(*pool).await,
    }
    .context("Failed to perform a query to get booking by booking id.")?
    .map(|row| Booking {
        booking_id: row.booking_id,
        created_at: row.created_at,
        updated_at: row.updated_at,
        transaction_id: row.transaction_id,
        rental_id: row.rental_id,
        vendor_id: row.vendor_id,
        pricing_id: row.pricing_id,
        quantity: row.quantity,
        start_date: row.start_date,
        end_date: row.end_date,
        booking_status: row.booking_status,
        total: row.total,
        rental: None,
        available: None,
    });

    Ok(booking)
}

#[tracing::instrument(
    name = "Update booking status in database by booking id",
    skip(executor)
)]
pub async fn update_booking_status_in_database_by_booking_id<'e>(
    booking_id: &Uuid,
    booking_status: &BookingStatus,
    executor: &mut DbExecutor<'e>,
) -> Result<(), anyhow::Error> {
    let query = sqlx::query!(
        r#"
        UPDATE bookings
        SET
            booking_status = $2
        WHERE booking_id = $1
        "#,
        booking_id,
        booking_status as &BookingStatus
    );

    match executor {
        DbExecutor::Transaction(transaction) => query.execute(&mut **transaction).await,
        DbExecutor::Pool(pool) => query.execute(*pool).await,
    }
    .context("Failed to perform a query to update booking status by booking id.")?;

    Ok(())
}

#[tracing::instrument(name = "Get booked quantity by rental id", skip(executor))]
pub async fn get_booked_quantity_by_rental_id<'e>(
    rental_id: &Uuid,
    booking_id: &Option<Uuid>,
    start_date: &OffsetDateTime,
    end_date: &OffsetDateTime,
    executor: &mut DbExecutor<'e>,
) -> Result<Vec<(OffsetDateTime, i32)>, anyhow::Error> {
    let base_sql = r#"
        WITH dates AS (
            SELECT generate_series(
        "#;

    let mut query = QueryBuilder::new(base_sql);

    query.push_bind(start_date);
    query.push("::timestamptz, ");
    query.push_bind(end_date);
    query.push("::timestamptz, '1 day'::interval) AS date), relevant_bookings AS (SELECT b.start_date, b.end_date, b.quantity FROM bookings b WHERE b.rental_id = ");
    query.push_bind(rental_id);
    query.push(
        " AND b.booking_status IN ('requested', 'accepted', 'confirmed', 'completed', 'disputed')",
    );

    if let Some(exclude_id) = booking_id {
        query.push(" AND b.booking_id IS DISTINCT FROM ");
        query.push_bind(exclude_id);
    }

    query.push(
        r#"
        )
        SELECT date, COALESCE(SUM(rb.quantity)::INTEGER, 0) as booked_quantity  -- Cast SUM result to INTEGER
        FROM dates d
        LEFT JOIN relevant_bookings rb ON d.date BETWEEN rb.start_date AND rb.end_date
        GROUP BY date
        ORDER BY date
    "#,
    );

    let query = query.build();

    let availability_data: Vec<(OffsetDateTime, i32)> = match executor {
        DbExecutor::Transaction(transaction) => query.fetch_all(&mut **transaction).await,
        DbExecutor::Pool(pool) => query.fetch_all(*pool).await,
    }
    .context("Failed to perform a query to get availability info")?
    .into_iter()
    .map(|record| {
        let date: OffsetDateTime = record.get("date");
        let booked_quantity: i32 = record.get("booked_quantity"); // Ensured as i32
        (date, booked_quantity)
    })
    .collect::<Vec<(OffsetDateTime, i32)>>();

    Ok(availability_data)
}
