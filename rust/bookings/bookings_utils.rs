use crate::routes::booking_holds::booking_holds_model::{BookingHold, BookingHoldStatus};
use crate::routes::bookings::bookings_model::{
    Availability, Booking, BookingStatus, GetAvailabilityQuery,
};
use crate::routes::bookings::bookings_service::check_availability;
use crate::routes::rentals::rentals_model::{GetRentalsQuery, Rental};
use crate::routes::rentals::rentals_service::get_rentals_by_query;
use crate::utilities::database::db_executor::DbExecutor;
use crate::utilities::errors::AppError;
use std::collections::HashMap;
use time::OffsetDateTime;
use uuid::Uuid;

pub fn merge_booked_quantities_and_holds(
    booked_quantities: Vec<(OffsetDateTime, i32)>,
    booking_holds: Vec<BookingHold>,
) -> HashMap<OffsetDateTime, (i32, i32)> {
    let mut merged = HashMap::new();

    // Aggregate booked quantities by date
    for (date, quantity) in booked_quantities {
        merged.entry(date).or_insert((0, 0)).0 += quantity;
    }

    // TODO: Booking holds are meant to lock in a quantity for a user once they've
    //  gone to the checkout page. Add a countdown timer on that page showing how long they
    //  have to finalize their request. We can make that time limit 30 minutes.
    //  Instead of just outright deleting the holds, we can throw away any that are >30 mins
    //  here in this fn. We should still cleanup pending holds and delete payment intents that are
    //  older than 24 hours.

    // Aggregate booking holds by date
    for hold in booking_holds {
        let start_date = hold.start_date;
        let end_date = hold.end_date;
        let per_day_hold = hold.quantity;

        let mut current_date = start_date;
        while current_date <= end_date {
            merged.entry(current_date).or_insert((0, 0)).1 += per_day_hold;
            current_date += time::Duration::days(1);
        }
    }

    merged
}

pub fn calculate_availability_from_merged_bookings(
    merged_data: HashMap<OffsetDateTime, (i32, i32)>,
    total_quantity: i32,
) -> Vec<Availability> {
    let mut availability: Vec<Availability> = merged_data
        .into_iter()
        .map(|(date, (booked, hold))| {
            let available_quantity = total_quantity - booked - hold;
            Availability {
                date,
                available_quantity,
            }
        })
        .collect();

    availability.sort_by_key(|entry| entry.date);

    availability
}

pub async fn build_booking_details<'e>(
    mut bookings: Vec<Booking>,
    include_rentals: bool,
    include_availability: bool,
    executor: &mut DbExecutor<'e>,
) -> Result<Vec<Booking>, AppError> {
    if include_rentals {
        // Step 1: Collect rental_ids from bookings
        let rental_ids: Vec<Uuid> = bookings.iter().map(|booking| booking.rental_id).collect();

        // Step 2 & 3: Fetch rentals by rental_ids and create a map
        let rentals_query = GetRentalsQuery {
            rental_ids: Some(rental_ids),
            per_page: Some(10000),
            ..Default::default()
        };
        let rentals = get_rentals_by_query(&rentals_query, executor).await?.data;
        let rental_map: HashMap<Uuid, Rental> = rentals
            .into_iter()
            .map(|rental| (rental.rental_id, rental))
            .collect();

        // Step 4: Assign rentals to bookings
        for booking in &mut bookings {
            if let Some(rental) = rental_map.get(&booking.rental_id) {
                booking.rental = Some(rental.clone());
            }
        }
    }

    if include_availability {
        for booking in &mut bookings {
            let availability_query = GetAvailabilityQuery {
                rental_id: booking.rental_id,
                start_date: booking.start_date,
                end_date: booking.end_date,
                exclude_transaction_id: None,
                exclude_booking_id: Some(booking.booking_id),
                booking_hold_status: Some(BookingHoldStatus::Blocked),
            };
            let availability =
                check_availability(booking.quantity, availability_query, executor).await;
            let available = availability.is_ok();
            booking.available = Some(available);
        }
    }

    Ok(bookings)
}

pub fn group_bookings_by_vendor(bookings: &[Booking]) -> HashMap<Uuid, Vec<&Booking>> {
    bookings.iter().fold(HashMap::new(), |mut acc, booking| {
        acc.entry(booking.vendor_id).or_default().push(booking);
        acc
    })
}

pub fn group_bookings_by_transaction(bookings: Vec<Booking>) -> HashMap<Uuid, Vec<Booking>> {
    bookings
        .into_iter()
        .fold(HashMap::new(), |mut acc, booking| {
            acc.entry(booking.transaction_id).or_default().push(booking);
            acc
        })
}

pub fn group_bookings_by_status(bookings: Vec<Booking>) -> HashMap<BookingStatus, Vec<Booking>> {
    bookings
        .into_iter()
        .fold(HashMap::new(), |mut acc, booking| {
            acc.entry(booking.booking_status).or_default().push(booking);
            acc
        })
}

pub fn count_booking_statuses(bookings: &[Booking]) -> HashMap<BookingStatus, usize> {
    bookings.iter().fold(HashMap::new(), |mut acc, booking| {
        *acc.entry(booking.booking_status).or_insert(0) += 1;
        acc
    })
}

pub fn validate_booking_status_transition(
    current_status: BookingStatus,
    new_status: BookingStatus,
) -> anyhow::Result<(), AppError> {
    match current_status {
        BookingStatus::Requested => match new_status {
            BookingStatus::Accepted | BookingStatus::Declined | BookingStatus::Canceled => Ok(()),
            _ => Err(AppError::ValidationError(String::from(
                "Invalid status transition",
            ))),
        },
        BookingStatus::Accepted => match new_status {
            BookingStatus::Confirmed | BookingStatus::Canceled => Ok(()),
            _ => Err(AppError::ValidationError(String::from(
                "Invalid status transition",
            ))),
        },
        BookingStatus::Confirmed => match new_status {
            BookingStatus::Completed | BookingStatus::Canceled => Ok(()),
            _ => Err(AppError::ValidationError(String::from(
                "Invalid status transition",
            ))),
        },
        BookingStatus::Declined
        | BookingStatus::Canceled
        | BookingStatus::Completed
        | BookingStatus::Disputed => Err(AppError::ValidationError(String::from(
            "Invalid status transition",
        ))),
    }
}
