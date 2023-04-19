use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{error::InternalError, get, http::StatusCode, post, HttpResponse};
use tera::Context;

use crate::{
    errors::MapErrToInternal, handlers::index::render_html, utils::extract_metadata_from_image,
};

#[get("/upload")]
async fn upload_get() -> actix_web::Result<HttpResponse> {
    render_html("upload_form.html", &Context::new())
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

    let mut context = Context::new();
    context.insert("message", &body);
    render_html("upload_result.html", &context)
}
