use actix_web::{middleware, App, HttpServer};
use nested_routing::app_config::config_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .configure(config_app)
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}
