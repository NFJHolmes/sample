use crate::routes::booking_holds::booking_holds_model::{BookingHoldStatus, GetBookingHoldsQuery};
use crate::routes::booking_holds::booking_holds_service::get_booking_holds_by_query;
use crate::routes::bookings::bookings_model::{
    Availabilities, Availability, Booking, BookingStatus, GetAvailabilitiesQuery,
    GetAvailabilityQuery, GetBookingsQuery, RequestBooking,
};
use crate::routes::bookings::bookings_repo::{
    create_booking_in_database, get_booked_quantity_by_rental_id,
    get_booking_from_database_by_booking_id, get_bookings_from_database_by_query,
    update_booking_status_in_database_by_booking_id,
};
use crate::routes::bookings::bookings_utils::{
    build_booking_details, calculate_availability_from_merged_bookings,
    merge_booked_quantities_and_holds,
};
use crate::routes::rentals::rentals_service::get_rental_by_rental_id;
use crate::routes::transactions::transactions_model::TransactionType;
use crate::routes::transactions::transactions_service::{
    get_transaction_by_transaction_id, handle_transaction_accept_decline,
    handle_transaction_cancel_booking, handle_transaction_complete,
};
use crate::shared::types::PaginatedResponse;
use crate::startup::AppState;
use crate::utilities::database::db_executor::DbExecutor;
use crate::utilities::errors::AppError;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[tracing::instrument(name = "Request booking", skip(executor))]
pub async fn request_booking<'e>(
    request: RequestBooking,
    executor: &mut DbExecutor<'e>,
) -> Result<Booking, AppError> {
    match request.transaction_type {
        TransactionType::External => {
            if request.pricing_id.is_some() {
                return Err(AppError::ValidationError(String::from(
                    "External bookings cannot have a pricing id",
                )));
            }
            if request.total.is_none() {
                return Err(AppError::ValidationError(String::from(
                    "External bookings must have a total",
                )));
            }
        }
        _ => {
            if request.pricing_id.is_none() {
                return Err(AppError::ValidationError(String::from(
                    "Non-external bookings must have a pricing id",
                )));
            }
            if request.total.is_some() {
                return Err(AppError::ValidationError(String::from(
                    "Non-external bookings cannot have a total",
                )));
            }
        }
    }

    let availability_query: GetAvailabilityQuery = GetAvailabilityQuery {
        rental_id: request.rental_id,
        start_date: request.start_date,
        end_date: request.end_date,
        exclude_transaction_id: request.transaction_id,
        exclude_booking_id: None,
        // Don't consider pending booking holds, only blocked
        booking_hold_status: Some(BookingHoldStatus::Blocked),
    };
    check_availability(request.quantity, availability_query, executor).await?;

    let booking_id = create_booking_in_database(request, executor).await?;
    let booking = get_booking_by_booking_id(&booking_id, executor).await?;

    Ok(booking)
}

#[tracing::instrument(name = "Check availability", skip(executor))]
pub async fn check_availability<'e>(
    quantity: i32,
    query_params: GetAvailabilityQuery,
    executor: &mut DbExecutor<'e>,
) -> Result<Vec<Availability>, AppError> {
    let availability = get_availability(query_params, executor).await?;

    // Check if the requested quantity is available
    let is_available = availability
        .iter()
        .all(|a| a.available_quantity >= quantity);

    if !is_available {
        return Err(AppError::ValidationError(String::from(
            "Requested quantity exceeds available quantity.",
        )));
    }

    Ok(availability)
}

#[tracing::instrument(name = "Get availability", skip(executor))]
pub async fn get_availability<'e>(
    query_params: GetAvailabilityQuery,
    executor: &mut DbExecutor<'e>,
) -> Result<Vec<Availability>, AppError> {
    // Fetch total quantity available for the rental item
    let rental = get_rental_by_rental_id(&query_params.rental_id, executor).await?;
    let total_quantity = rental.quantity;

    // Fetch booked quantities for each day within the date range
    let booked_quantities = get_booked_quantity_by_rental_id(
        &query_params.rental_id,
        &query_params.exclude_booking_id,
        &query_params.start_date,
        &query_params.end_date,
        executor,
    )
    .await?;

    // Fetch booking holds for each day within the date range
    let booking_holds = get_booking_holds_by_query(
        &GetBookingHoldsQuery {
            rental_id: Some(query_params.rental_id),
            start_date: Some(query_params.start_date),
            end_date: Some(query_params.end_date),
            exclude_transaction_id: query_params.exclude_transaction_id,
            booking_hold_status: query_params.booking_hold_status,
            per_page: Some(10000),
            ..Default::default()
        },
        executor,
    )
    .await?
    .data;

    // Merge booked quantities and booking holds
    let merged_bookings = merge_booked_quantities_and_holds(booked_quantities, booking_holds);

    // Calculate availability from merged data
    let availability = calculate_availability_from_merged_bookings(merged_bookings, total_quantity);

    Ok(availability)
}

#[tracing::instrument(name = "Get availabilities", skip(executor))]
pub async fn get_availabilities<'e>(
    query_params: GetAvailabilitiesQuery,
    executor: &mut DbExecutor<'e>,
) -> Result<Availabilities, AppError> {
    let mut availabilities: HashMap<Uuid, Vec<Availability>> = HashMap::new();

    for rental_id in query_params.rental_ids.iter() {
        let availability_query = GetAvailabilityQuery {
            rental_id: *rental_id,
            start_date: query_params.start_date,
            end_date: query_params.end_date,
            exclude_transaction_id: query_params.exclude_transaction_id,
            exclude_booking_id: None,
            booking_hold_status: query_params.booking_hold_status,
        };

        let availability = get_availability(availability_query, executor).await?;
        availabilities.insert(*rental_id, availability);
    }

    Ok(Availabilities { availabilities })
}

#[tracing::instrument(name = "Accept booking", skip(state, executor))]
pub async fn accept_booking<'e>(
    booking_id: &Uuid,
    state: Arc<AppState>,
    executor: &mut DbExecutor<'e>,
) -> Result<Booking, AppError> {
    let booking =
        update_booking_status_by_booking_id(booking_id, &BookingStatus::Accepted, executor).await?;

    // TODO: Ideally this should be done outside of the "accept" functionality.
    //  Start a cron job that checks for eligible confirm/decline/partial transactions
    //  same for decline_booking which also calls this fn
    let transaction = get_transaction_by_transaction_id(&booking.transaction_id, executor).await?;
    handle_transaction_accept_decline(&transaction, state, executor).await?;

    Ok(booking)
}

#[tracing::instrument(name = "Decline booking", skip(state, executor))]
pub async fn decline_booking<'e>(
    booking_id: &Uuid,
    state: Arc<AppState>,
    executor: &mut DbExecutor<'e>,
) -> Result<Booking, AppError> {
    let booking =
        update_booking_status_by_booking_id(booking_id, &BookingStatus::Declined, executor).await?;

    let transaction = get_transaction_by_transaction_id(&booking.transaction_id, executor).await?;
    handle_transaction_accept_decline(&transaction, state, executor).await?;

    Ok(booking)
}

// TODO: Something to consider here is if its a user or a vendor cancelation
// If its user, we should refund but keep the deposit
// If its vendor, we refund everything
#[tracing::instrument(name = "Cancel booking", skip(state, executor))]
pub async fn cancel_booking<'e>(
    booking: Booking,
    state: Arc<AppState>,
    executor: &mut DbExecutor<'e>,
) -> Result<Booking, AppError> {
    let previous_status = booking.booking_status;

    let booking = update_booking_status_by_booking_id(
        &booking.booking_id,
        &BookingStatus::Canceled,
        executor,
    )
    .await?;

    let transaction = get_transaction_by_transaction_id(&booking.transaction_id, executor).await?;
    if transaction.transaction_type == TransactionType::External {
        return Ok(booking); // Early return for external bookings because there is no refund or notification necessary
    }

    handle_transaction_cancel_booking(&transaction, &booking, &previous_status, state, executor)
        .await?;

    Ok(booking)
}

#[tracing::instrument(name = "Confirm booking", skip(bookings, executor))]
pub async fn confirm_bookings<'e>(
    bookings: &[Booking],
    executor: &mut DbExecutor<'e>,
) -> Result<Vec<Booking>, AppError> {
    let mut updated_bookings = Vec::new();

    for booking in bookings {
        let updated_booking = confirm_booking(booking, executor).await?;
        updated_bookings.push(updated_booking);
    }

    Ok(updated_bookings)
}

#[tracing::instrument(name = "Confirm booking", skip(booking, executor))]
pub async fn confirm_booking<'e>(
    booking: &Booking,
    executor: &mut DbExecutor<'e>,
) -> Result<Booking, AppError> {
    let booking = update_booking_status_by_booking_id(
        &booking.booking_id,
        &BookingStatus::Confirmed,
        executor,
    )
    .await?;

    Ok(booking)
}

#[tracing::instrument(name = "Complete booking", skip(state, executor))]
pub async fn complete_booking<'e>(
    booking: Booking,
    state: Arc<AppState>,
    executor: &mut DbExecutor<'e>,
) -> Result<Booking, AppError> {
    let booking = update_booking_status_by_booking_id(
        &booking.booking_id,
        &BookingStatus::Completed,
        executor,
    )
    .await?;

    let transaction = get_transaction_by_transaction_id(&booking.transaction_id, executor).await?;
    if transaction.transaction_type == TransactionType::External {
        return Ok(booking); // Early return for external bookings because there is no payout necessary
    }

    handle_transaction_complete(
        &booking.transaction_id,
        &booking.vendor_id,
        state.clone(),
        executor,
    )
    .await?;

    Ok(booking)
}

#[tracing::instrument(name = "Get all bookings by query", skip(executor))]
pub async fn get_bookings_by_query<'e>(
    query_params: &GetBookingsQuery,
    executor: &mut DbExecutor<'e>,
) -> Result<PaginatedResponse<Booking>, AppError> {
    let mut bookings_response = get_bookings_from_database_by_query(query_params, executor).await?;

    let include_rental = query_params.include_rental == Some(true);
    let include_availability = query_params.check_availability == Some(true);
    bookings_response.data = build_booking_details(
        bookings_response.data,
        include_rental,
        include_availability,
        executor,
    )
    .await?;

    Ok(bookings_response)
}

#[tracing::instrument(name = "Get booking by booking id", skip(executor))]
pub async fn get_booking_by_booking_id<'e>(
    booking_id: &Uuid,
    executor: &mut DbExecutor<'e>,
) -> Result<Booking, AppError> {
    let booking_option = match get_booking_from_database_by_booking_id(booking_id, executor).await {
        Err(e) => {
            tracing::error!("Failed to get booking by booking id: {}", e);
            return Err(AppError::UnexpectedError(e));
        }
        Ok(x) => x,
    };

    let mut booking = match booking_option {
        None => {
            tracing::error!("Booking not found for booking id: {}", booking_id);
            return Err(AppError::DoesNotExistError(String::from(
                "Booking not found",
            )));
        }
        Some(booking) => booking,
    };

    let rental_id = booking.rental_id;
    let rental = get_rental_by_rental_id(&rental_id, executor).await?;
    booking.rental = Some(rental);

    Ok(booking)
}

#[tracing::instrument(name = "Update booking by booking id", skip(executor))]
pub async fn update_booking_status_by_booking_id<'e>(
    booking_id: &Uuid,
    booking_status: &BookingStatus,
    executor: &mut DbExecutor<'e>,
) -> Result<Booking, AppError> {
    // TODO: For updates, we should setup a transaction instead and use "FOR UPDATE"
    // at the end of the select clause. This will lock the row until the transaction is committed.

    update_booking_status_in_database_by_booking_id(booking_id, booking_status, executor).await?;

    let booking_new = get_booking_by_booking_id(booking_id, executor).await?;

    Ok(booking_new)
}
