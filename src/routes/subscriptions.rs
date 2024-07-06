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

use crate::startup::AppState;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

#[tracing::instrument(name = "Adding a new subcriber", skip(state, data), fields(user_name = data.name, user_email = data.email))]
#[autometrics]
pub async fn subscribe(State(state): State<Arc<AppState>>, Form(data): Form<FormData>) -> Response {
    match insert_subscriber(state.pg_pool(), data.name, data.email).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[tracing::instrument(name = "Saving new subcriber details", skip(pool))]
async fn insert_subscriber(pool: &PgPool, name: String, email: String) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, name, email, subscribed_at)
            VALUES ($1, $2, $3, $4)
            "#,
        Uuid::new_v4(),
        name,
        email,
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
