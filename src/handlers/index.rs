use crate::{errors::MapErrToInternal, templates::TEMPLATES};
use actix_web::{get, http::header::ContentType, HttpResponse};
use tera::Context;

pub fn render_html(template: &str, context: &Context) -> actix_web::Result<HttpResponse> {
    let html = TEMPLATES.render(template, context).map_err_to_internal()?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

#[get("/")]
async fn index() -> actix_web::Result<HttpResponse> {
    render_html("index.html", &Context::new())
}
