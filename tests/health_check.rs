use std::net::TcpListener;

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

fn spawn_app() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let port = listener.local_addr().unwrap().port();
    let server = zerotoprod::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);
    port
}
