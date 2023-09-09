use actix_web::{
    web::{get, post, resource, Data, PayloadConfig},
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
    let dotenvy_result = dotenvy::dotenv();
    match dotenvy_result {
        Err(error) if error.not_found() => {}
        Ok(_) => {}
        Err(error) => return Err(error)?,
    };
    let config: Config = envy::from_env()?;

    // Configure logging
    env_logger::init();

    // Create missing folders
    create_dir_all(config.media_root.join("images")).await?;

    // Establish sqlite connection
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(&config.database_url)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let app_config = config.clone();
    let app = HttpServer::new(move || {
        App::new()
            .app_data(PayloadConfig::new(1000000 * 250))
            // Files serving
            .service(actix_files::Files::new(
                "/media",
                config.media_root.to_str().unwrap(),
            ))
            .service(actix_files::Files::new("/static", "./static"))
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
            .app_data(Data::new(app_config.clone()))
    })
    .bind((config.host, config.port))?;
    log::debug!("Running server on http://{}:{}/", config.host, config.port);
    app.run().await?;
    Ok(())
}
