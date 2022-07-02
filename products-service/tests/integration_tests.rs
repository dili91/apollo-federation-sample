use std::net::TcpListener;

#[actix_rt::test]
async fn products_service_is_up() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/up", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(
        "I'm Alive!",
        response.text().await.expect("Failed to get response body")
    );
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind listener.");
    let port = listener.local_addr().unwrap().port();
    let server = products_service::run(listener).expect("Failed to start test server");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
