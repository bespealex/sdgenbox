use crate::{errors::MapErrToInternal, utils::extract_metadata_from_image};
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{
    error::InternalError,
    get,
    http::{header::ContentType, StatusCode},
    post, App, HttpResponse, HttpServer, Responder,
};

mod errors;
mod models;
mod utils;

#[get("/upload")]
async fn upload_get() -> impl Responder {
    HttpResponse::Ok().content_type(ContentType::html()).body(
        r#"
    <html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/upload" method="post" enctype="multipart/form-data">
                <input type="file" name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>
    "#,
    )
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart]
    file: TempFile,
}

#[post("/upload")]
async fn upload_post(
    MultipartForm(form): MultipartForm<UploadForm>,
) -> actix_web::Result<HttpResponse> {
    let image =
        extract_metadata_from_image(form.file.file.path().to_str().ok_or(InternalError::new(
            "file path cannot be translated to &str",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))?)
        .map_err_to_internal()?;

    let body = match image {
        None => "No info".to_string(),
        Some(image) => format!("<pre>Parameters: \n{:#?}</pre>", image),
    };

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "localhost";
    let port = 8080;

    env_logger::init();

    let app = HttpServer::new(|| App::new().service(upload_get).service(upload_post))
        .bind((host, port))?;
    log::info!("Running server on http://{}:{}/", host, port);
    app.run().await
}
