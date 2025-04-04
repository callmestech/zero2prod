#![allow(clippy::let_underscore_future)]

use once_cell::sync::Lazy;
use reqwest::Url;
use sqlx::{Connection, PgConnection, PgPool};
use tokio::net::TcpListener;
use uuid::Uuid;
use zero2prod::{
    configuration::{get_configuration, DatabaseSettings},
    email_client::EmailClient,
    startup::build_server,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let test_app = spawn_app().await;

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let test_app = spawn_app().await;

    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    // Act
    let respone = client
        .post(&format!("{}/subscriptions", &test_app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(respone.status().as_u16(), 200);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&test_app.pg_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let test_app = spawn_app().await;

    let client = reqwest::Client::new();
    let test_cases = [
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    // Act
    for (invalid_body, error_message) in test_cases {
        let respone = client
            .post(&format!("{}/subscriptions", &test_app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            respone.status().as_u16(),
            422,
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}

#[tokio::test]
async fn subscribe_returns_a_200_when_fields_are_present_but_empty() {
    // Arrange
    let test_app = spawn_app().await;

    let client = reqwest::Client::new();
    let test_cases = [
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        ("name=Ursula&email=", "empty email"),
        ("name=Ursula&email=not_email", ""),
    ];

    // Act
    for (invalid_body, error_message) in test_cases {
        let respone = client
            .post(&format!("{}/subscriptions", &test_app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            respone.status().as_u16(),
            400,
            "The API did not return a 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info";
    let subscriber_name = "test";

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(
            subscriber_name.to_string(),
            default_filter_level.to_string(),
            std::io::stdout,
        );
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(
            subscriber_name.to_string(),
            default_filter_level.to_string(),
            std::io::sink,
        );
        init_subscriber(subscriber);
    }
});

struct TestApp {
    pub address: String,
    pub pg_pool: sqlx::PgPool,
}

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let pg_pool = configure_db(&configuration.database).await;
    let base_url = Url::parse(&format!("http://{}", &configuration.email_client.base_url)).unwrap();
    let email_client = EmailClient::new(
        base_url,
        configuration.email_client.sender().unwrap(),
        configuration.email_client.authorization_token,
    );

    let _ = tokio::spawn(build_server(listener, pg_pool.clone(), email_client));
    TestApp { address, pg_pool }
}

async fn configure_db(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres.");

    // Create the database
    sqlx::query(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_ref())
        .execute(&mut connection)
        .await
        .expect("Failed to create database.");

    // Migrate the database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database.");

    connection_pool
}
