use crate::graphql::{graphql_api, graphql_playground, ProductsQuery};
use crate::repository::ProductsRepository;
use crate::routes::{get_product_by_sku, list_products, up};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use std::net::TcpListener;

pub fn run(
    listener: TcpListener,
    products_repository: ProductsRepository,
) -> Result<Server, std::io::Error> {
    let graphql_schema = Schema::build(ProductsQuery, EmptyMutation, EmptySubscription)
        .data(products_repository.clone())
        .finish();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(graphql_schema.clone()))
            .app_data(Data::new(products_repository.clone()))
            .route("/up", web::get().to(up))
            .route("/products", web::get().to(list_products))
            .route("/products/{sku}", web::get().to(get_product_by_sku))
            .route("/graphql/api", web::post().to(graphql_api))
            .route("/graphql/ui", web::get().to(graphql_playground))
    })
    .listen(listener.try_clone()?)?
    .run();

    Ok(server)
}
