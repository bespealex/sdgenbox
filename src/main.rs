use actix_web::{web::Data, App, HttpServer};

mod handlers;
mod models;
mod templates;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let host = "localhost";
    let port = 8080;

    env_logger::init();

    let connection = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(":memory:")
        .await?;

    let app = HttpServer::new(move || {
        App::new()
            .service(handlers::index::index)
            .service(handlers::index::db_test)
            .service(handlers::upload::upload_get)
            .service(handlers::upload::upload_post)
            .app_data(Data::new(connection.clone()))
    })
    .bind((host, port))?;
    log::info!("Running server on http://{}:{}/", host, port);
    app.run().await?;
    Ok(())
}
