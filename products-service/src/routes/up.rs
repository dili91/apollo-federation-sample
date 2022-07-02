use actix_web::{HttpResponse, Responder};

pub async fn up() -> impl Responder {
    HttpResponse::Ok().body("I'm Alive!")
}
