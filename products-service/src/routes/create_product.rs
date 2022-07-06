use crate::repository;
use crate::repository::{ProductsRepository, ProductsRepositoryQueries};
use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CreateProductRequest {
    pub sku: String,
    pub name: String,
    pub description: String,
    pub price_in_minor: i32,
}

impl From<web::Json<CreateProductRequest>> for repository::CreateProductRequest {
    fn from(request: web::Json<CreateProductRequest>) -> Self {
        repository::CreateProductRequest {
            sku: request.0.sku,
            name: request.0.name,
            description: request.0.description,
            price_in_minor: request.0.price_in_minor,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateProductResponse {
    pub id: Uuid,
}

pub async fn create_product(
    products_repository: web::Data<ProductsRepository>,
    new_product: web::Json<CreateProductRequest>,
) -> impl Responder {
    let result = products_repository
        .get_ref()
        .create_product(new_product.into())
        .await;

    match result {
        Ok(id) => HttpResponse::Created().json(CreateProductResponse{id}),
        Err(err) => {
            HttpResponse::InternalServerError().body(err.as_database_error().unwrap().to_string())
        }
    }
}
