use std::net::TcpListener;

use sqlx::{query, Connection, Executor, PgConnection, PgPool, Pool, Postgres};
use uuid::Uuid;

use zerotoprod::configuration::{get_configuration, DatabaseSettings};

pub struct TestApp {
    pub address: String,
    pub pool: PgPool,
}

#[tokio::test]
async fn health_check_works() {
    let test_app = spawn_app().await;

    let response = reqwest::get(format!("{}/health_check", test_app.address))
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_ok_for_valid_form_data() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "email=ursula_le_guin%40gmail.com&name=le%20guin";
    let response = client
        .post(format!("{}/subscribe", test_app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

    let saved = query!("SELECT email, name from subscriptions")
        .fetch_one(&test_app.pool)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!("le guin", saved.name);
    assert_eq!("ursula_le_guin@gmail.com", saved.email);
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(format!("{}/subscribe", test_app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind Address");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read config file");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = create_database(&configuration.database).await;
    let server = zerotoprod::startup::run(listener, connection_pool.clone())
        .expect("Failed to bind address");

    let _ = tokio::spawn(server);

    TestApp {
        address,
        pool: connection_pool,
    }
}

pub async fn create_database(database_settings: &DatabaseSettings) -> Pool<Postgres> {
    PgConnection::connect(&database_settings.connection_string_without_db())
        .await
        .expect("Failed to connect to PG instance")
        .execute(&*format!(
            r#"CREATE DATABASE "{}";"#,
            database_settings.database_name
        ))
        .await
        .expect("Failed to execute database creation");

    let pool = PgPool::connect(&database_settings.connection_string())
        .await
        .expect("Failed to connect to db");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate the database");

    pool
}
