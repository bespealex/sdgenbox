use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{
    error::InternalError,
    get,
    http::{header::ContentType, StatusCode},
    post,
    web::{self, Data, Redirect},
    HttpResponse, Responder,
};
use askama::Template;
use sqlx::{Connection, Pool, Sqlite};

use crate::{
    models::{create_image, fetch_image_by_id, Image},
    utils::{errors::MapErrToInternal, image::extract_metadata_from_image, render::render_html},
};

#[derive(Template)]
#[template(path = "images/upload_form.html")]
struct UploadFormTemplate<'a> {
    error_message: Option<&'a str>,
}

#[derive(serde::Deserialize)]
struct UploadGetQuery {
    error_message: Option<String>,
}

#[get("/images/upload")]
async fn upload_get(query: web::Query<UploadGetQuery>) -> actix_web::Result<HttpResponse> {
    render_html(
        UploadFormTemplate {
            error_message: query.error_message.as_deref(),
        },
        HttpResponse::Ok(),
    )
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart]
    file: TempFile,
}

#[post("/images/upload")]
async fn upload_post(
    MultipartForm(form): MultipartForm<UploadForm>,
    pool: Data<Pool<Sqlite>>,
) -> actix_web::Result<impl Responder> {
    let image =
        extract_metadata_from_image(form.file.file.path().to_str().ok_or(InternalError::new(
            "file path cannot be translated to &str",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))?)
        .map_err_to_internal()?;
    let mut image = match image {
        None => return Ok(Redirect::to("/images/upload?error_message=No info").see_other()),
        Some(image) => image,
    };

    let mut connection = pool.acquire().await.map_err_to_internal()?;
    let mut transaction = connection.begin().await.map_err_to_internal()?;
    create_image(&mut transaction, &mut image, form.file)
        .await
        .map_err_to_internal()?;
    transaction.commit().await.map_err_to_internal()?;

    Ok(Redirect::to(format!("/images/{}", image.id)).see_other())
}

#[derive(Template)]
#[template(path = "images/image.html")]
struct GetImageTemplate {
    image: Image,
}

#[get("/images/{id}")]
pub async fn get_image(
    pool: web::Data<Pool<Sqlite>>,
    path: web::Path<(i64,)>,
) -> actix_web::Result<impl Responder> {
    let (image_id,) = path.into_inner();

    let mut connection = pool.acquire().await.map_err_to_internal()?;
    let image = fetch_image_by_id(&mut connection, image_id)
        .await
        .map_err_to_internal()?;
    let image = match image {
        None => {
            return Ok(HttpResponse::NotFound()
                .content_type(ContentType::html())
                .body("No image"))
        }
        Some(image) => image,
    };

    render_html(GetImageTemplate { image }, HttpResponse::Created())
}
