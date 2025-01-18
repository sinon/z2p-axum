use std::collections::HashMap;

use hyper::StatusCode;
use sqlx::PgPool;

mod common;

#[sqlx::test]
async fn create_subscriber_works(db: PgPool) -> sqlx::Result<()> {
    let test_app = common::spawn_app().await;

    let mut map = HashMap::new();
    map.insert("email", "test@example.com");
    map.insert("name", "Joe B");

    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/api/subscriber", &test_app.address))
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status(), StatusCode::CREATED);

    let resp_json: serde_json::Value = serde_json::from_slice(&response.bytes().await.unwrap())
        .expect("failed to read response body as json");

    assert_eq!(resp_json["name"], "Joe B");
    assert_eq!(resp_json["email"], "test@example.com");

    common::expect_uuid(&resp_json["id"]);

    Ok(())
}

#[sqlx::test]
async fn create_subscriber_fails_when_data_is_missing(db: PgPool) -> sqlx::Result<()> {
    let test_app = common::spawn_app().await;

    let map: HashMap<String, String> = HashMap::default();
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/api/subscriber", &test_app.address))
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    Ok(())
}

#[sqlx::test]
async fn create_subsciber_fails_duplicate_email(db: PgPool) -> sqlx::Result<()> {
    let test_app = common::spawn_app().await;

    let mut map = HashMap::new();
    map.insert("email", "test@example.com");
    map.insert("name", "Joe B");

    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/api/subscriber", &test_app.address))
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status(), StatusCode::CREATED);

    let resp_json: serde_json::Value = serde_json::from_slice(&response.bytes().await.unwrap())
        .expect("failed to read response body as json");

    assert_eq!(resp_json["name"], "Joe B");
    assert_eq!(resp_json["email"], "test@example.com");

    common::expect_uuid(&resp_json["id"]);
    let response = client
        .post(&format!("{}/api/subscriber", &test_app.address))
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    Ok(())
}
