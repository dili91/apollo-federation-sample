use std::net::TcpListener;

#[actix_rt::test]
async fn should_be_up() {
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

#[actix_rt::test]
async fn should_return_a_list_of_products(){
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/products", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    //todo: body assertions
}

#[actix_rt::test]
async fn should_return_a_product_by_id(){
// Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    let product_id = "123";

    // Act
    let response = client
        .get(&format!("{}/products/{}", &address, product_id))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    //todo: body assertions
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind listener.");
    let port = listener.local_addr().unwrap().port();
    let server = products_service::run(listener).expect("Failed to start test server");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
