use actix_web::{App, HttpServer};

mod handlers;
mod models;
mod templates;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "localhost";
    let port = 8080;

    env_logger::init();

    let app = HttpServer::new(|| {
        App::new()
            .service(handlers::index::index)
            .service(handlers::upload::upload_get)
            .service(handlers::upload::upload_post)
    })
    .bind((host, port))?;
    log::info!("Running server on http://{}:{}/", host, port);
    app.run().await
}
