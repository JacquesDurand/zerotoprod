use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let port = spawn_app();

    let response = reqwest::get(format!("http://127.0.0.1:{}/health_check", port))
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
}

fn spawn_app() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind Address");
    let port = listener.local_addr().unwrap().port();
    let server = zerotoprod::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    port
}
