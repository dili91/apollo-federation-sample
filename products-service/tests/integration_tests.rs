use actix_web::test;
use products_service::configuration::get_configuration;
use products_service::repository::ProductsRepositoryBuilder;
use products_service::routes::ProductsList;
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;

#[actix_rt::test]
async fn should_be_up() {
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
async fn should_return_a_list_of_products() {
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
    let body: ProductsList = response.json()
        .await.expect("failed to get JSON payload");
    assert_eq!(3, body.items.len());
}

#[actix_rt::test]
async fn should_return_a_product_by_id() {
    // Arrange
    let address = spawn_app().await;
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
