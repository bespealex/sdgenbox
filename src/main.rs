use std::env;

use actix_web::{web::Data, App, HttpServer};
use anyhow::Context;
use tokio::fs::create_dir_all;

mod handlers;
mod models;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let host = "localhost";
    let port = 8080;

    env_logger::init();
    dotenvy::dotenv()?;

    // Create missing folders
    create_dir_all("media/images").await?;

    let database_url = env::var("DATABASE_URL").context("DATABASE_URL is missing")?;
    let connection = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(&database_url)
        .await?;

    let app = HttpServer::new(move || {
        App::new()
            .service(actix_files::Files::new("/media", "media"))
            .service(handlers::index::index)
            .service(handlers::images::upload_get)
            .service(handlers::images::upload_post)
            .service(handlers::images::get_image)
            .app_data(Data::new(connection.clone()))
    })
    .bind((host, port))?;
    log::info!("Running server on http://{}:{}/", host, port);
    app.run().await?;
    Ok(())
}
