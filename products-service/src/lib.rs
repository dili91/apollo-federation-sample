use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn up() -> impl Responder {
    HttpResponse::Ok().body("I'm Alive!")
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/up", web::get().to(up)))
        .bind(("127.0.0.1", 8080))?
        .run();
    Ok(server)
}
