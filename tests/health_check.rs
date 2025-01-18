use sqlx::PgPool;

mod common;

#[sqlx::test]
async fn health_check_works(_db: PgPool) {
    let test_app = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/healthcheck", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
