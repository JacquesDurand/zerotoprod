use std::net::TcpListener;

use sqlx::{query, Connection, PgConnection};

use zerotoprod::configuration::get_configuration;

#[tokio::test]
async fn health_check_works() {
    let port = spawn_app();

    let response = reqwest::get(format!("http://127.0.0.1:{}/health_check", port))
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_ok_for_valid_form_data() {
    let port = spawn_app();

    let connection_string = get_configuration()
        .expect("Failed to read config file")
        .database
        .connection_string();

    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to PostGre");
    let client = reqwest::Client::new();

    let body = "email=ursula_le_guin%40gmail.com&name=le%20guin";
    let response = client
        .post(format!("http://127.0.0.1:{}/subscribe", port))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

    let saved = query!("SELECT email, name from subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!("le guin", saved.name);
    assert_eq!("ursula_le_guin@gmail.com", saved.email);
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let port = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(format!("http://127.0.0.1:{}/subscribe", port))
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

fn spawn_app() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind Address");
    let port = listener.local_addr().unwrap().port();
    let server = zerotoprod::startup::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    port
}
