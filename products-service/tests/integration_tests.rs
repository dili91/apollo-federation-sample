use products_service::configuration::get_configuration;
use products_service::repository::{Product, ProductsRepositoryBuilder};
use products_service::routes::{CreateProductRequest, CreateProductResponse, ProductsList};
use std::net::TcpListener;

#[actix_rt::test]
async fn up_should_return_200() {
    // Arrange
    let address = spawn_app().await;
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
async fn list_products_should_return_200() {
    // Arrange
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/products", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    let body: ProductsList = response.json().await.expect("failed to get JSON payload");
    assert_eq!(3, body.items.len());
}

#[actix_rt::test]
async fn get_product_by_sku_should_return_200() {
    // Arrange
    let address = spawn_app().await;
    let client = reqwest::Client::new();
    let sku = "izp-sng-cc";

    // Act
    let response = client
        .get(&format!("{}/products/{}", &address, sku))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    let body: Product = response.json().await.expect("failed to get JSON payload");
    assert_eq!(sku, body.sku);
}

#[actix_rt::test]
async fn get_product_by_sku_should_return_404() {
    // Arrange
    let address = spawn_app().await;
    let client = reqwest::Client::new();
    let sku = "does-not-exist";

    // Act
    let response = client
        .get(&format!("{}/products/{}", &address, sku))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(404, response.status().as_u16());
}

#[actix_rt::test]
async fn create_product_should_return_201_and_the_product_id() {
    // Arrange
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .post(&format!("{}/products", &address))
        .json(&CreateProductRequest {
            sku: "apl-iph-13".to_string(),
            name: "Apple Iphone 13".to_string(),
            description: "Yet another Iphone".to_string(),
            price_in_minor: 99000,
        })
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(
        201,
        response.status().as_u16(),
        "{}",
        response.text().await.unwrap()
    );
    let body: CreateProductResponse = response.json().await.expect("failed to get JSON payload");
    assert_eq!(false, body.id.is_nil())
}

#[actix_rt::test]
async fn graphql_api_should_return_200() {
    // Arrange
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .post(&format!("{}/graphql/api", &address))
        .header("content-type", "application/json")
        .body("{\"query\":\"introspectionQuery\"}")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
}

#[actix_rt::test]
async fn graphql_ui_playground_should_return_200() {
    // Arrange
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/graphql/ui", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind listener.");
    let port = listener.local_addr().unwrap().port();

    let settings = get_configuration().expect("Failed to load configration");

    let products_repository = ProductsRepositoryBuilder::new(settings.database)
        .build()
        .await
        .expect("Failed to build products repository");

    let server = products_service::startup::run(listener, products_repository)
        .expect("Failed to start test server");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
