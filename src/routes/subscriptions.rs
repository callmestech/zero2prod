use std::sync::Arc;

use autometrics::autometrics;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Form,
};
use chrono::Utc;
use hyper::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::{NewSubscriber, SubscriberEmail, SubscriberName},
    startup::AppState,
};

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

#[tracing::instrument(name = "Adding a new subcriber", skip(state, data), fields(user_name = data.name, user_email = data.email))]
#[autometrics]
pub async fn subscribe(State(state): State<Arc<AppState>>, Form(data): Form<FormData>) -> Response {
    let new_subscriber = match data.try_into() {
        Ok(subscriber) => subscriber,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    match insert_subscriber(state.pg_pool(), &new_subscriber).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;
    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name.clone()).map_err(|_| "Invalid name")?;
        let email = SubscriberEmail::parse(value.email).map_err(|_| "Invalid email")?;
        Ok(NewSubscriber { name, email })
    }
}

#[tracing::instrument(name = "Saving new subcriber details", skip(pool))]
async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, name, email, subscribed_at)
            VALUES ($1, $2, $3, $4)
            "#,
        Uuid::new_v4(),
        new_subscriber.name.as_ref(),
        new_subscriber.email.as_ref(),
        Utc::now(),
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
