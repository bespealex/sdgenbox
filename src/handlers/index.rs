use crate::{errors::MapErrToInternal, templates::TEMPLATES};
use actix_web::{get, http::header::ContentType, HttpResponse, HttpResponseBuilder};
use tera::Context;

pub fn render_html(
    template: &str,
    context: &Context,
    mut http_response: HttpResponseBuilder,
) -> actix_web::Result<HttpResponse> {
    let html = TEMPLATES.render(template, context).map_err_to_internal()?;

    Ok(http_response.content_type(ContentType::html()).body(html))
}

#[get("/")]
async fn index() -> actix_web::Result<HttpResponse> {
    render_html("index.html", &Context::new(), HttpResponse::Ok())
}
