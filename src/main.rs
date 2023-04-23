use actix_web::{web::Data, App, HttpServer};
use tokio::fs::create_dir_all;

use crate::config::Config;

mod config;
mod handlers;
mod models;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load config
    dotenvy::dotenv()?;
    let config = envy::from_env::<Config>()?;

    // Configure logging
    env_logger::init();

    // Create missing folders
    create_dir_all("media/images").await?;

    // Establish sqlite connection
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(&config.database_url)
        .await?;

    let app = HttpServer::new(move || {
        App::new()
            .service(actix_files::Files::new("/media", "media"))
            .service(handlers::index::index)
            .service(handlers::images::upload_get)
            .service(handlers::images::upload_post)
            .service(handlers::images::get_image)
            .service(handlers::images::list_images)
            .app_data(Data::new(pool.clone()))
    })
    .bind((config.host, config.port))?;
    log::info!("Running server on http://{}:{}/", config.host, config.port);
    app.run().await?;
    Ok(())
}
