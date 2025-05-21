use crate::domain::SubscriberName;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres, types::Uuid};
use validator::Validate;

use crate::startup::AppState;

#[derive(Deserialize, Validate)]
pub struct CreateSubscriber {
    pub name: SubscriberName,
    #[validate(email)]
    pub email: String,
}

#[derive(Serialize, FromRow)]
struct Subscriber {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(new_subscriber, _state),
    fields(
        subscriber_email = %new_subscriber.email,
        subscriber_name = %new_subscriber.name
    )
)]
pub async fn create_subscriber(
    State(_state): State<AppState>,
    Json(new_subscriber): Json<CreateSubscriber>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(_e) = new_subscriber.validate() {
        return Err((StatusCode::BAD_REQUEST, "Validation Error".to_string()));
    }
    match insert_subscriber(&new_subscriber, &_state.pool).await {
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
        subscriber.email,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(result)
}
