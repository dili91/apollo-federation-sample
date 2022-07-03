use crate::repository::ProductsRepository;
use crate::routes::{get_product_by_sku, list_products, up};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub fn run(
    listener: TcpListener,
    products_repository: ProductsRepository,
) -> Result<Server, std::io::Error> {
    let products_repository = web::Data::new(products_repository);

    let server = HttpServer::new(move || {
        App::new()
            .route("/up", web::get().to(up))
            .route("/products", web::get().to(list_products))
            .route("/products/{sku}", web::get().to(get_product_by_sku))
            .app_data(products_repository.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
