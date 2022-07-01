use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn up() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/up", web::get().to(up)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
