use crate::repository::{ProductsRepository, ProductsRepositoryQueries};
use actix_web::{web, HttpResponse, Responder};

pub async fn get_product_by_sku(
    sku: web::Path<String>,
    products_repository: web::Data<ProductsRepository>,
) -> impl Responder {
    let product = products_repository
        .get_ref()
        .get_product_by_sku(sku.into_inner().as_str())
        .await;

    match product {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(err) => match err.as_database_error() {
            None => HttpResponse::NotFound().finish(),
            Some(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
    }
}
