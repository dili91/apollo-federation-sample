#[actix_rt::test]
async fn products_service_is_up() {
    // Arrange
    spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8080/up")
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

fn spawn_app() {
    let server = products_service::run().expect("failed to start test server");
    let _ = tokio::spawn(server);
}
