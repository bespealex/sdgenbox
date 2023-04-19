use actix_web::{get, HttpResponse};
use tera::Context;

use crate::utils::render::render_html;

#[get("/")]
async fn index() -> actix_web::Result<HttpResponse> {
    render_html("index.html", &Context::new(), HttpResponse::Ok())
}
