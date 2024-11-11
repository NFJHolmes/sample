use crate::routes::auth::credentials::UserEmail;
use crate::startup::AppState;
use crate::utilities::email::email::BookingEmailParams;
use crate::utilities::errors::AppError;
use anyhow::Context;
use std::sync::Arc;

// TODO: Hookup canceled email
// TODO: Not sure that this one is necessary. We can just send a refund email.
//  otherwise only the accept/decline emails are sent
#[tracing::instrument(
    name = "Send a booking canceled email to user",
    skip(state, user_email, params)
)]
pub async fn send_booking_canceled_email(
    state: Arc<AppState>,
    user_email: UserEmail,
    params: BookingEmailParams,
) -> anyhow::Result<(), AppError> {
    let base_url = &state.configuration.client.base_url;
    let enable_emails = &state.configuration.application.enable_emails;
    let email_client = &state.email_client;

    if !enable_emails {
        return Ok(());
    }

    let bookings_link = format!("{}/bookings", base_url,);

    let mut tera_context = tera::Context::new();
    tera_context.insert("bookings_link", bookings_link.as_str());
    tera_context.insert("confirmation_code", &params.confirmation_code);
    tera_context.insert("start_date", &params.start_date);
    tera_context.insert("end_date", &params.end_date);
    tera_context.insert("total", &params.total);
    tera_context.insert("rentals", &params.rentals);

    let booking_canceled_template = email_client
        .tera
        .render("booking_canceled.html", &tera_context)
        .context("Failed to parse booking canceled email template")?;

    let plain_body = format!(
        "Your rental booking has been canceled.\nPlease visit {} for details.",
        bookings_link
    );

    email_client
        .send_email(
            &user_email,
            "Your rental booking has been canceled",
            booking_canceled_template.as_str(),
            &plain_body,
        )
        .await
        .context("Failed to send a booking canceled email")?;

    Ok(())
}

// TODO: Lets generate another token for each email
// then link to /bookings/{confirmation_code}?token={token}
// if token exists, then we dont need to validate user is logged in.
// This way the links always work, regardless if its an anon user or logged in user.
#[tracing::instrument(
    name = "Send a booking confirmed email to user",
    skip(state, user_email, params)
)]
pub async fn send_booking_confirmed_email(
    state: Arc<AppState>,
    user_email: UserEmail,
    params: BookingEmailParams,
) -> anyhow::Result<(), AppError> {
    let base_url = &state.configuration.client.base_url;
    let enable_emails = &state.configuration.application.enable_emails;
    let email_client = &state.email_client;

    if !enable_emails {
        return Ok(());
    }

    let bookings_link = format!("{}/bookings", base_url,);

    let mut tera_context = tera::Context::new();
    tera_context.insert("bookings_link", &bookings_link);
    tera_context.insert("confirmation_code", &params.confirmation_code);
    tera_context.insert("start_date", &params.start_date);
    tera_context.insert("end_date", &params.end_date);
    tera_context.insert("total", &params.total);
    tera_context.insert("rentals", &params.rentals);

    let booking_confirmed_template = email_client
        .tera
        .render("booking_confirmed.html", &tera_context)
        .context("Failed to parse booking confirmed email template")?;

    let plain_body = format!(
        "Your rental booking has been confirmed.\nPlease visit {} for details.",
        bookings_link
    );

    email_client
        .send_email(
            &user_email,
            "Your rental booking has been confirmed",
            booking_confirmed_template.as_str(),
            &plain_body,
        )
        .await
        .context("Failed to send a booking confirmed email")?;

    Ok(())
}

#[tracing::instrument(
    name = "Send a booking declined email to user",
    skip(state, user_email, params)
)]
pub async fn send_booking_declined_email(
    state: Arc<AppState>,
    user_email: UserEmail,
    params: BookingEmailParams,
) -> anyhow::Result<(), AppError> {
    let base_url = &state.configuration.client.base_url;
    let enable_emails = &state.configuration.application.enable_emails;
    let email_client = &state.email_client;

    if !enable_emails {
        return Ok(());
    }

    let bookings_link = format!("{}/bookings", base_url,);

    let mut tera_context = tera::Context::new();
    tera_context.insert("bookings_link", bookings_link.as_str());
    tera_context.insert("confirmation_code", &params.confirmation_code);
    tera_context.insert("start_date", &params.start_date);
    tera_context.insert("end_date", &params.end_date);
    tera_context.insert("total", &params.total);
    tera_context.insert("rentals", &params.rentals);

    let booking_declined_template = email_client
        .tera
        .render("booking_declined.html", &tera_context)
        .context("Failed to parse booking declined email template")?;

    let plain_body = format!(
        "Your rental booking has been declined.\nPlease visit {} for details.",
        bookings_link
    );

    email_client
        .send_email(
            &user_email,
            "Your rental booking has been declined",
            booking_declined_template.as_str(),
            &plain_body,
        )
        .await
        .context("Failed to send a booking declined email")?;

    Ok(())
}

#[tracing::instrument(
    name = "Send a booking partial email to user",
    skip(state, user_email, params)
)]
pub async fn send_booking_partial_email(
    state: Arc<AppState>,
    user_email: UserEmail,
    params: BookingEmailParams,
) -> anyhow::Result<(), AppError> {
    let base_url = &state.configuration.client.base_url;
    let enable_emails = &state.configuration.application.enable_emails;
    let email_client = &state.email_client;

    if !enable_emails {
        return Ok(());
    }

    let bookings_link = format!("{}/bookings", base_url,);

    let mut tera_context = tera::Context::new();
    tera_context.insert("bookings_link", bookings_link.as_str());
    tera_context.insert("confirmation_code", &params.confirmation_code);
    tera_context.insert("start_date", &params.start_date);
    tera_context.insert("end_date", &params.end_date);
    tera_context.insert("total", &params.total);
    tera_context.insert("rentals", &params.rentals);

    let booking_partial_template = email_client
        .tera
        .render("booking_partial.html", &tera_context)
        .context("Failed to parse booking partial email template")?;

    let plain_body = format!("At least one of the items in your latest request were declined.\nPlease visit {} to confirm or deny you
        would like to continue with a partial booking.", bookings_link);

    email_client
        .send_email(
            &user_email,
            "Would you like to accept a partial booking?",
            booking_partial_template.as_str(),
            &plain_body,
        )
        .await
        .context("Failed to send a booking partial email")?;

    Ok(())
}

#[tracing::instrument(
    name = "Send a booking refunded email to user",
    skip(state, user_email, params)
)]
pub async fn send_booking_refunded_email(
    state: Arc<AppState>,
    user_email: UserEmail,
    params: BookingEmailParams,
) -> anyhow::Result<(), AppError> {
    let base_url = &state.configuration.client.base_url;
    let enable_emails = &state.configuration.application.enable_emails;
    let email_client = &state.email_client;

    if !enable_emails {
        return Ok(());
    }

    let bookings_link = format!("{}/bookings", base_url);

    let mut tera_context = tera::Context::new();
    tera_context.insert("bookings_link", bookings_link.as_str());
    tera_context.insert("confirmation_code", &params.confirmation_code);
    tera_context.insert("start_date", &params.start_date);
    tera_context.insert("end_date", &params.end_date);
    tera_context.insert("total", &params.total);
    tera_context.insert("rentals", &params.rentals);

    let booking_refunded_template = email_client
        .tera
        .render("booking_refunded.html", &tera_context)
        .context("Failed to parse booking refunded email template")?;

    let plain_body = format!(
        "Your rental booking has been refunded.\nPlease visit {} for details.",
        bookings_link
    );

    email_client
        .send_email(
            &user_email,
            "Your rental booking has been refunded",
            booking_refunded_template.as_str(),
            &plain_body,
        )
        .await
        .context("Failed to send a booking refunded email")?;

    Ok(())
}

// TODO: Hookup reminder email
#[tracing::instrument(
    name = "Send a booking reminder email to user",
    skip(state, user_email, params)
)]
pub async fn send_booking_reminder_email(
    state: Arc<AppState>,
    user_email: UserEmail,
    params: BookingEmailParams,
) -> anyhow::Result<(), AppError> {
    let base_url = &state.configuration.client.base_url;
    let enable_emails = &state.configuration.application.enable_emails;
    let email_client = &state.email_client;

    if !enable_emails {
        return Ok(());
    }

    let bookings_link = format!("{}/bookings", base_url,);

    let mut tera_context = tera::Context::new();
    tera_context.insert("bookings_link", bookings_link.as_str());
    tera_context.insert("confirmation_code", &params.confirmation_code);
    tera_context.insert("start_date", &params.start_date);
    tera_context.insert("end_date", &params.end_date);
    tera_context.insert("total", &params.total);
    tera_context.insert("rentals", &params.rentals);

    let booking_reminder_template = email_client
        .tera
        .render("booking_reminder.html", &tera_context)
        .context("Failed to parse booking reminder email template")?;

    let plain_body = format!(
        "Your rental booking is coming up soon.\nPlease visit {} for details.",
        bookings_link
    );

    email_client
        .send_email(
            &user_email,
            "Your upcoming rental booking details",
            booking_reminder_template.as_str(),
            &plain_body,
        )
        .await
        .context("Failed to send a booking reminder email")?;

    Ok(())
}
