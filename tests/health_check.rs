use std::net::TcpListener;

fn spawn_app() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let port = listener.local_addr().unwrap().port();
    let server = zerotoprod::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);
    port
}

#[tokio::test]
async fn health_check_works() {
    let port = spawn_app();
    let address = format!("http://127.0.0.1:{}", port);
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health-check", address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let port = spawn_app();
    let address = format!("http://127.0.0.1:{}", port);
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(format!("{}/subscriptions", address))
        .header("Content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn subscribe_returns_400_for_invalid_form_data() {
    let port = spawn_app();
    let address = format!("http://127.0.0.1:{}", port);
    let client = reqwest::Client::new();

    let test_case = vec![
        ("name=le%20guin", "Email is missing"),
        ("email=ursula_le_guin%40gmail.com", "Name is missing"),
        ("", "Both name and email are missing"),
    ];

    for (body, error_message) in test_case {
        let response = client
            .post(format!("{}/subscriptions", address))
            .header("Content-type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not respond with a 400 for the payload {}",
            error_message
        );
    }
}
