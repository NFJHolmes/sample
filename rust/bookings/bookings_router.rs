use crate::routes::bookings::bookings_handler::{
    handle_accept_booking, handle_cancel_booking, handle_check_availability,
    handle_complete_booking, handle_decline_booking, handle_get_availabilities,
    handle_get_availability, handle_get_booking,
};
use crate::startup::AppState;
use crate::utilities::middleware::require_auth::require_auth_middleware;
use axum::routing::{get, patch};
use axum::{middleware, Router};
use std::sync::Arc;

pub fn bookings_router() -> Router<Arc<AppState>> {
    Router::new()
        // .route("/bookings", get(handle_get_bookings_by_query))
        .route("/bookings/:id", get(handle_get_booking))
        // .route("/bookings", post(handle_request_booking))
        // .route("/bookings/request", post(handle_request_bookings))
        .route("/bookings/:id/accept", patch(handle_accept_booking))
        .route("/bookings/:id/decline", patch(handle_decline_booking))
        .route("/bookings/:id/cancel", patch(handle_cancel_booking))
        .route("/bookings/:id/complete", patch(handle_complete_booking))
        .layer(middleware::from_fn(require_auth_middleware))
        .route("/bookings/availability", get(handle_get_availability))
        .route("/bookings/availabilities", get(handle_get_availabilities))
        .route(
            "/bookings/availability/:quantity",
            get(handle_check_availability),
        )
}
