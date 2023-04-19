use actix_web::{http::header::ContentType, HttpResponse, HttpResponseBuilder};
use tera::Context;

use crate::templates::TEMPLATES;

use super::errors::MapErrToInternal;

pub fn render_html(
    template: &str,
    context: &Context,
    mut http_response: HttpResponseBuilder,
) -> actix_web::Result<HttpResponse> {
    let html = TEMPLATES.render(template, context).map_err_to_internal()?;

    Ok(http_response.content_type(ContentType::html()).body(html))
}
