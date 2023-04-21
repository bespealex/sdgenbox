use actix_web::{get, HttpResponse};
use askama::Template;

use crate::utils::render::render_html;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[get("/")]
async fn index() -> actix_web::Result<HttpResponse> {
    render_html(IndexTemplate, HttpResponse::Ok())
}
