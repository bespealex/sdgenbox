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
    config::Config,
    models::{create_image, fetch_image_by_id, fetch_images, fetch_images_count, Image, Limits},
    utils::{
        errors::MapErrToInternal, image::extract_metadata_from_image, pager, render::render_html,
    },
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
    media_root: &Path,
) -> Result<Image, ParseAndSaveImageError> {
    let mut image = extract_metadata_from_image(original_file.file.path().to_str().unwrap())?
        .ok_or(ParseAndSaveImageError::ParseError)?;

    create_image(
        transaction,
        &mut image,
        original_file.file.path(),
        media_root,
    )
    .await?;

    Ok(image)
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart]
    files: Vec<TempFile>,
}

pub async fn upload_post(
    config: Data<Config>,
    MultipartForm(form): MultipartForm<UploadForm>,
    pool: Data<Pool<Sqlite>>,
) -> actix_web::Result<impl Responder> {
    let mut connection = pool.acquire().await.map_err_to_internal()?;
    let mut transaction = connection.begin().await.map_err_to_internal()?;

    let mut results = Vec::new();
    for original_file in form.files {
        let result =
            parse_and_save_image(&mut transaction, original_file, &config.media_root).await;
        results.push(result);
    }

    transaction.commit().await.map_err_to_internal()?;

    match &results[..] {
        [] => {
            Ok(Redirect::to("/images/upload?error_message=Provide at least one file").see_other())
        }
        [Ok(image)] => Ok(Redirect::to(format!("/images/{}", image.id)).see_other()),
        _ if results.iter().all(|r| r.is_ok()) => Ok(Redirect::to("/images").see_other()),
        _ => Ok(Redirect::to(
            "/images/upload?error_message=There are error while saving at least one image",
        )
        .see_other()),
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
    current_page: &'a u32,
    pager: Vec<Option<u32>>,
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
    page_query: web::Query<PageQuery>,
) -> actix_web::Result<impl Responder> {
    let mut connection = pool.acquire().await.map_err_to_internal()?;
    let search_form = search_form.into_inner();
    let search = &search_form.search.as_deref();
    let page = match page_query.page {
        Some(n) if n >= 1 => n,
        _ => 1,
    };

    let limits = Limits::from_page(page, PAGE_SIZE);
    let images = fetch_images(&mut connection, *search, &limits)
        .await
        .map_err_to_internal()?;
    let count = fetch_images_count(&mut connection, *search)
        .await
        .map_err_to_internal()?;

    let pages = (count as f32 / PAGE_SIZE as f32).ceil() as usize;
    let pager = pager::pager(pages as u32, page, 2, 2);
    render_html(
        ListImagesTemplate {
            images: &images[..],
            search_form: &search_form,
            current_page: &page,
            pager,
        },
        HttpResponse::Ok(),
    )
}

#[cfg(test)]
mod test {}
