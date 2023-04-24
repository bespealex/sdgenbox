use actix_web::{
    web::{get, post, resource, Data},
    App, HttpServer,
};
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
            // Files serving
            .service(actix_files::Files::new("/media", "media"))
            .service(actix_files::Files::new("/static", "static"))
            // Dynamic handlers
            .service(resource("/").route(get().to(handlers::index::index)))
            .service(resource("/images").route(get().to(handlers::images::list_images)))
            .service(
                resource("/images/upload")
                    .route(get().to(handlers::images::upload_get))
                    .route(post().to(handlers::images::upload_post)),
            )
            .service(resource("/images/{id}").route(get().to(handlers::images::get_image)))
            // Services
            .app_data(Data::new(pool.clone()))
    })
    .bind((config.host, config.port))?;
    log::info!("Running server on http://{}:{}/", config.host, config.port);
    app.run().await?;
    Ok(())
}
