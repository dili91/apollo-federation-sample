use env_logger::Env;
use products_service::configuration::get_configuration;
use products_service::repository::ProductsRepositoryBuilder;
use products_service::startup::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let settings = get_configuration().expect("Failed to load configuration");

    let address = format!("127.0.0.1:{}", settings.port);
    let listener = TcpListener::bind(address)?;

    let products_repository = ProductsRepositoryBuilder::new(settings.database)
        .build()
        .await
        .expect("Failed to initialize products repository.");

    run(listener, products_repository)?.await
}
