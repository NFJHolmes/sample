use crate::routes::booking_holds::booking_holds_model::BookingHoldStatus;
use crate::routes::bookings::bookings_model::{
    Availabilities, Availability, Booking, BookingStatus, GetAvailabilitiesQuery,
    GetAvailabilityQuery,
};
use crate::routes::bookings::bookings_service::{
    accept_booking, cancel_booking, check_availability, complete_booking, decline_booking,
    get_availabilities, get_availability, get_booking_by_booking_id,
};
use crate::routes::bookings::bookings_utils::validate_booking_status_transition;
use crate::routes::rbac::rbac_service::{
    verify_rbac_user_employee_session, verify_rbac_user_session,
};
use crate::routes::transactions::transactions_service::get_transaction_by_transaction_id;
use crate::session::UserSession;
use crate::startup::AppState;
use crate::utilities::database::db_executor::DbExecutor;
use crate::utilities::errors::AppError;
use crate::utilities::extractors::query::SerdeQsQuery;
use anyhow::{Context, Result};
use axum::extract::Path;
use axum::{extract, Json};
use std::sync::Arc;
use time::OffsetDateTime;
use uuid::Uuid;

#[tracing::instrument(name = "Accept booking handler", skip(session, state))]
pub async fn handle_accept_booking(
    session: UserSession,
    booking_id: Path<Uuid>,
    extract::State(state): extract::State<Arc<AppState>>,
) -> Result<Json<Booking>, AppError> {
    let transaction = state
        .db_pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;
    let mut executor = DbExecutor::Transaction(transaction);

    let booking = get_booking_by_booking_id(&booking_id, &mut executor).await?;
    verify_rbac_user_employee_session(&session, &booking.vendor_id, &mut executor).await?;
    validate_booking_status_transition(booking.booking_status, BookingStatus::Accepted)?;

    // Validate that vendor has sufficient quantity to accept the booking
    let availability_query: GetAvailabilityQuery = GetAvailabilityQuery {
        rental_id: booking.rental_id,
        start_date: booking.start_date,
        end_date: booking.end_date,
        exclude_transaction_id: None,
        exclude_booking_id: Some(booking.booking_id),
        // Don't consider pending booking holds, only blocked
        booking_hold_status: Some(BookingHoldStatus::Blocked),
    };
    check_availability(booking.quantity, availability_query, &mut executor).await?;

    let booking = accept_booking(&booking_id, state, &mut executor).await?;

    executor
        .commit()
        .await
        .context("Failed to commit SQL transaction to accept booking.")?;

    Ok(Json(booking))
}

#[tracing::instrument(name = "Decline booking handler", skip(session, state))]
pub async fn handle_decline_booking(
    session: UserSession,
    booking_id: Path<Uuid>,
    extract::State(state): extract::State<Arc<AppState>>,
) -> Result<Json<Booking>, AppError> {
    let transaction = state
        .db_pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;
    let mut executor = DbExecutor::Transaction(transaction);

    let booking = get_booking_by_booking_id(&booking_id, &mut executor).await?;
    verify_rbac_user_employee_session(&session, &booking.vendor_id, &mut executor).await?;
    validate_booking_status_transition(booking.booking_status, BookingStatus::Declined)?;

    let booking = decline_booking(&booking_id, state, &mut executor).await?;

    executor
        .commit()
        .await
        .context("Failed to commit SQL transaction to decline booking.")?;

    Ok(Json(booking))
}

#[tracing::instrument(name = "Cancel booking handler", skip(session, state))]
pub async fn handle_cancel_booking(
    session: UserSession,
    booking_id: Path<Uuid>,
    extract::State(state): extract::State<Arc<AppState>>,
) -> Result<Json<Booking>, AppError> {
    let transaction = state
        .db_pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;
    let mut executor = DbExecutor::Transaction(transaction);

    let booking = get_booking_by_booking_id(&booking_id, &mut executor).await?;

    let is_employee =
        verify_rbac_user_employee_session(&session, &booking.vendor_id, &mut executor).await;
    match is_employee {
        Ok(_) => {}
        Err(_) => {
            let transaction =
                get_transaction_by_transaction_id(&booking.transaction_id, &mut executor).await?;
            let user_id = &transaction.user_id.expect("Missing user id in transaction");
            verify_rbac_user_session(&session, user_id).await?;
        }
    }

    validate_booking_status_transition(booking.booking_status, BookingStatus::Canceled)?;

    let booking = cancel_booking(booking, state, &mut executor).await?;

    executor
        .commit()
        .await
        .context("Failed to commit SQL transaction to cancel booking.")?;

    Ok(Json(booking))
}

#[tracing::instrument(name = "Complete booking handler", skip(session, state))]
pub async fn handle_complete_booking(
    session: UserSession,
    booking_id: Path<Uuid>,
    extract::State(state): extract::State<Arc<AppState>>,
) -> Result<Json<Booking>, AppError> {
    let transaction = state
        .db_pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;
    let mut executor = DbExecutor::Transaction(transaction);

    let booking = get_booking_by_booking_id(&booking_id, &mut executor).await?;
    verify_rbac_user_employee_session(&session, &booking.vendor_id, &mut executor).await?;
    validate_booking_status_transition(booking.booking_status, BookingStatus::Completed)?;

    let current_utc_date = OffsetDateTime::now_utc();
    if booking.end_date >= current_utc_date {
        return Err(AppError::ValidationError(String::from(
            "Booking cannot be completed before its end date.",
        )));
    }

    let booking = complete_booking(booking, state.clone(), &mut executor).await?;

    executor
        .commit()
        .await
        .context("Failed to commit SQL transaction to complete booking.")?;

    Ok(Json(booking))
}

// #[tracing::instrument(name = "Get all bookings by query handler", skip(session, state))]
// pub async fn handle_get_bookings_by_query(
//     session: UserSession,
//     extract::Query(query_params): extract::Query<GetBookingsQuery>,
//     extract::State(state): extract::State<Arc<AppState>>,
// ) -> Result<Json<Vec<Booking>>, AppError> {
//     let user_id = &session.id()?.expect("User id not found in session");
//     let mut executor = DbExecutor::Pool(&state.db_pool);
//     verify_get_bookings_rbac(user_id, &query_params, &mut executor).await?;
//
//     let bookings = get_bookings_by_query(&query_params, &mut executor).await?;
//
//     Ok(Json(bookings))
// }

#[tracing::instrument(name = "Handle get availability", skip(state))]
pub async fn handle_get_availability(
    extract::Query(query_params): extract::Query<GetAvailabilityQuery>,
    extract::State(state): extract::State<Arc<AppState>>,
) -> Result<Json<Vec<Availability>>, AppError> {
    let mut executor = DbExecutor::Pool(&state.db_pool);
    let availability = get_availability(query_params, &mut executor).await?;

    Ok(Json(availability))
}

#[tracing::instrument(name = "Handle get availabilities", skip(state))]
pub async fn handle_get_availabilities(
    SerdeQsQuery(query_params): SerdeQsQuery<GetAvailabilitiesQuery>,
    extract::State(state): extract::State<Arc<AppState>>,
) -> Result<Json<Availabilities>, AppError> {
    let mut executor = DbExecutor::Pool(&state.db_pool);
    let query_params = query_params.expect("Failed to deserialize query params");
    let availabilities = get_availabilities(query_params, &mut executor).await?;

    Ok(Json(availabilities))
}

#[tracing::instrument(name = "Handle check availability", skip(state))]
pub async fn handle_check_availability(
    Path(quantity): Path<i32>,
    extract::Query(query_params): extract::Query<GetAvailabilityQuery>,
    extract::State(state): extract::State<Arc<AppState>>,
) -> Result<Json<Vec<Availability>>, AppError> {
    let mut executor = DbExecutor::Pool(&state.db_pool);
    let availability = check_availability(quantity, query_params, &mut executor).await?;

    Ok(Json(availability))
}

#[tracing::instrument(name = "Get booking handler", skip(session, state))]
pub async fn handle_get_booking(
    session: UserSession,
    booking_id: Path<Uuid>,
    extract::State(state): extract::State<Arc<AppState>>,
) -> Result<Json<Booking>, AppError> {
    let mut executor = DbExecutor::Pool(&state.db_pool);
    let booking = get_booking_by_booking_id(&booking_id, &mut executor).await?;

    let is_employee =
        verify_rbac_user_employee_session(&session, &booking.vendor_id, &mut executor).await;
    match is_employee {
        Ok(_) => {}
        Err(_) => {
            let transaction =
                get_transaction_by_transaction_id(&booking.transaction_id, &mut executor).await?;
            let user_id = &transaction.user_id.expect("Missing user id in transaction");
            verify_rbac_user_session(&session, user_id).await?;
        }
    }

    Ok(Json(booking))
}
