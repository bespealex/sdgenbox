use std::path::Path;

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{
    http::header::ContentType,
    web::{self, Data, Redirect},
    HttpResponse, Responder,
};
use askama::Template;
use serde::Deserialize;
use sqlx::{Connection, Pool, Sqlite, Transaction};

use crate::{
    models::{create_image, fetch_image_by_id, fetch_images, fetch_images_count, Image, Limits},
    utils::{errors::MapErrToInternal, image::extract_metadata_from_image, render::render_html},
};

#[derive(Template)]
#[template(path = "images/upload_form.html")]
pub struct UploadFormTemplate<'a> {
    error_message: Option<&'a str>,
}

#[derive(serde::Deserialize)]
pub struct UploadGetQuery {
    error_message: Option<String>,
}

pub async fn upload_get(query: web::Query<UploadGetQuery>) -> actix_web::Result<HttpResponse> {
    render_html(
        UploadFormTemplate {
            error_message: query.error_message.as_deref(),
        },
        HttpResponse::Ok(),
    )
}

#[derive(Debug, thiserror::Error)]
enum ParseAndSaveImageError {
    #[error("Failed to parse metadata from image")]
    ParseError,
    #[error("Internal error occuried")]
    InternalError(#[from] anyhow::Error),
}

async fn parse_and_save_image(
    transaction: &mut Transaction<'_, Sqlite>,
    original_file: TempFile,
) -> Result<Image, ParseAndSaveImageError> {
    let mut image = extract_metadata_from_image(original_file.file.path().to_str().unwrap())?
        .ok_or(ParseAndSaveImageError::ParseError)?;

    let mut image_file = tokio::fs::File::from_std(original_file.file.into_file());
    create_image(transaction, &mut image, &mut image_file, Path::new("media")).await?;

    Ok(image)
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart]
    files: Vec<TempFile>,
}

pub async fn upload_post(
    MultipartForm(form): MultipartForm<UploadForm>,
    pool: Data<Pool<Sqlite>>,
) -> actix_web::Result<impl Responder> {
    let mut connection = pool.acquire().await.map_err_to_internal()?;
    let mut transaction = connection.begin().await.map_err_to_internal()?;

    let mut images: Vec<Image> = Vec::new();
    for original_file in form.files {
        let image = match parse_and_save_image(&mut transaction, original_file).await {
            Ok(image) => image,
            Err(ParseAndSaveImageError::ParseError) => {
                return Ok(Redirect::to(
                    "/images/upload?error_message=Cannot parse metadata from provided file",
                )
                .see_other())
            }
            Err(ParseAndSaveImageError::InternalError(error)) => {
                Err(error).map_err_to_internal()?
            }
        };
        images.push(image);
    }

    transaction.commit().await.map_err_to_internal()?;

    match &images[..] {
        [] => {
            Ok(Redirect::to("/images/upload?error_message=Provide at least one file").see_other())
        }
        [image] => Ok(Redirect::to(format!("/images/{}", image.id)).see_other()),
        _ => Ok(Redirect::to("/images").see_other()),
    }
}

#[derive(Template)]
#[template(path = "images/image.html")]
pub struct GetImageTemplate {
    image: Image,
}

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

#[derive(Template)]
#[template(path = "images/list.html")]
pub struct ListImagesTemplate<'a> {
    images: &'a [Image],
    search_form: &'a SearchForm,
    current_page: u32,
    pages: u32,
}

#[derive(Deserialize)]
pub struct SearchForm {
    search: Option<String>,
}

#[derive(Deserialize)]
pub struct PageQuery {
    page: Option<u32>,
}

const PAGE_SIZE: u32 = 18;

pub async fn list_images(
    pool: web::Data<Pool<Sqlite>>,
    search_form: web::Query<SearchForm>,
    page: web::Query<PageQuery>,
) -> actix_web::Result<impl Responder> {
    let mut connection = pool.acquire().await.map_err_to_internal()?;
    let search_form = search_form.into_inner();
    let search = &search_form.search.as_deref();
    let page = page.into_inner().page.unwrap_or(0);

    let limits = Limits::from_page(page, PAGE_SIZE);
    let images = fetch_images(&mut connection, *search, &limits)
        .await
        .map_err_to_internal()?;
    let count = fetch_images_count(&mut connection, *search)
        .await
        .map_err_to_internal()?;

    let pages = (count as f32 / PAGE_SIZE as f32).ceil() as u32;
    render_html(
        ListImagesTemplate {
            images: &images[..],
            search_form: &search_form,
            current_page: page,
            pages,
        },
        HttpResponse::Ok(),
    )
}

#[cfg(test)]
mod test {}
