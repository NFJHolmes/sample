use crate::routes::booking_holds::booking_holds_model::BookingHoldStatus;
use crate::routes::rentals::rentals_model::Rental;
use crate::routes::transactions::transactions_model::TransactionType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::Display;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, sqlx::Type, Clone, Copy, Display, Eq, Hash)]
#[sqlx(type_name = "booking_status")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum BookingStatus {
    Requested,
    Accepted,
    Declined,
    Canceled,
    Confirmed,
    Completed,
    Disputed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Booking {
    pub booking_id: Uuid,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
    pub transaction_id: Uuid,
    pub rental_id: Uuid,
    pub vendor_id: Uuid,
    pub pricing_id: Option<Uuid>,
    pub quantity: i32,
    #[serde(with = "time::serde::iso8601")]
    pub start_date: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub end_date: OffsetDateTime,
    pub booking_status: BookingStatus,
    pub total: f64,
    pub rental: Option<Rental>,
    pub available: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Availability {
    #[serde(with = "time::serde::iso8601")]
    pub date: OffsetDateTime,
    pub available_quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Availabilities {
    pub availabilities: HashMap<Uuid, Vec<Availability>>,
}

// Booking Forms
#[derive(Debug, Deserialize, Default)]
pub struct GetBookingsQuery {
    pub transaction_ids: Option<Vec<Uuid>>,
    pub rental_id: Option<Uuid>,
    pub vendor_id: Option<Uuid>,
    #[serde(default, with = "time::serde::iso8601::option")]
    pub start_date: Option<OffsetDateTime>,
    #[serde(default, with = "time::serde::iso8601::option")]
    pub end_date: Option<OffsetDateTime>,
    pub booking_status: Option<BookingStatus>,
    pub include_rental: Option<bool>, // Whether to include rental details in the response
    pub check_availability: Option<bool>, // Whether to check availability for the booking
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct RequestBooking {
    pub transaction_id: Option<Uuid>,
    pub transaction_type: TransactionType,
    pub rental_id: Uuid,
    pub vendor_id: Uuid,
    pub pricing_id: Option<Uuid>, // For marketplace, bid, invoice transactions
    pub total: Option<f64>,       // For external transactions
    pub quantity: i32,
    #[serde(with = "time::serde::iso8601")]
    pub start_date: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub end_date: OffsetDateTime,
}

// TODO: Implement disputes
#[derive(Debug, Deserialize)]
pub struct DisputeBooking {
    pub vendor_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct GetAvailabilityQuery {
    pub rental_id: Uuid,
    #[serde(with = "time::serde::iso8601")]
    pub start_date: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub end_date: OffsetDateTime,
    pub exclude_transaction_id: Option<Uuid>,
    pub exclude_booking_id: Option<Uuid>,
    pub booking_hold_status: Option<BookingHoldStatus>,
}

#[derive(Debug, Deserialize)]
pub struct GetAvailabilitiesQuery {
    pub rental_ids: Vec<Uuid>,
    #[serde(with = "time::serde::iso8601")]
    pub start_date: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub end_date: OffsetDateTime,
    pub exclude_transaction_id: Option<Uuid>,
    pub booking_hold_status: Option<BookingHoldStatus>,
}
