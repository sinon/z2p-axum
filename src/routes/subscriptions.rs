use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres, types::Uuid};
use validator::Validate;

use crate::startup::AppState;

#[derive(Deserialize, Validate)]
pub struct CreateSubscriber {
    #[validate(length(min = 3, message = "name is required", max = 256))]
    pub name: String,
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
        subscriber.name,
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

#[cfg(test)]
mod tests {
    use validator::Validate;

    use super::CreateSubscriber;

    // TODO: Investigate if validate can be made utf-8 aware
    // #[test]
    // fn a_256_grapheme_long_name_is_valid() {
    //     let name = "ё".repeat(256);
    //     let email = "email@example.com".to_string();
    //     assert!(CreateSubscriber { name, email }.validate().is_err());
    // }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        let email = "email@example.com".to_string();
        assert!(CreateSubscriber { name, email }.validate().is_err());
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        let email = "email@example.com".to_string();
        assert!(CreateSubscriber { name, email }.validate().is_err());
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        let email = "email@example.com".to_string();
        assert!(CreateSubscriber { name, email }.validate().is_err());
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            let email = "email@example.com".to_string();
            assert!(CreateSubscriber { name, email }.validate().is_err());
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Ursula Le Guin".to_string();
        let email = "email@example.com".to_string();
        assert!(CreateSubscriber { name, email }.validate().is_ok());
    }
}
