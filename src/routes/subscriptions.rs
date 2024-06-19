use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Form,
};
use chrono::Utc;
use hyper::StatusCode;
use uuid::Uuid;

use crate::startup::AppState;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(State(state): State<Arc<AppState>>, Form(data): Form<FormData>) -> Response {
    match sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, name, email, subscribed_at)
            VALUES ($1, $2, $3, $4)
            "#,
        Uuid::new_v4(),
        data.name,
        data.email,
        Utc::now(),
    )
    .execute(state.pg_pool())
    .await
    {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => {
            println!("Failed to execute query: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
