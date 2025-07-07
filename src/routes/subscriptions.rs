use crate::domain::{CreateSubscriber, SubscriberEmail, SubscriberName};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use sqlx::{FromRow, Pool, Postgres, types::Uuid};

use crate::startup::AppState;

#[derive(Serialize, FromRow)]
struct Subscriber {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(new_subscriber, state),
    fields(
        subscriber_email = %new_subscriber.email,
        subscriber_name = %new_subscriber.name
    )
)]
pub async fn create_subscriber(
    State(state): State<AppState>,
    Json(new_subscriber): Json<CreateSubscriber>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(_e) = SubscriberName::parse(new_subscriber.name.as_ref().to_string()) {
        return Err((StatusCode::BAD_REQUEST, "Validation Error".to_string()));
    }
    if let Err(_e) = SubscriberEmail::parse(new_subscriber.email.as_ref().to_string()) {
        return Err((StatusCode::BAD_REQUEST, "Validation Error".to_string()));
    }
    match insert_subscriber(&new_subscriber, &state.pool).await {
        Ok(subscriber) => Ok((StatusCode::CREATED, Json(subscriber))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

#[tracing::instrument(
    name = "saving new subscriber details in the database",
    skip(pool, subscriber)
)]
async fn insert_subscriber(
    subscriber: &CreateSubscriber,
    pool: &Pool<Postgres>,
) -> Result<Subscriber, sqlx::Error> {
    let result = sqlx::query_as!(
        Subscriber,
        "insert into subscriber (name, email) values ($1, $2) returning id, name, email",
        subscriber.name.as_ref(),
        subscriber.email.as_ref(),
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(result)
}
