use crate::repository::{Product, ProductsRepository, ProductsRepositoryQueries};
use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProductsList {
    pub items: Vec<Product>,
}

pub async fn list_products(products_repository: web::Data<ProductsRepository>) -> impl Responder {
    let products = products_repository.get_ref().list_products().await;

    match products {
        Ok(p) => HttpResponse::Ok().json(ProductsList { items: p }),
        Err(err) => match err.as_database_error() {
            None => HttpResponse::NotFound().finish(),
            Some(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
    }
}
