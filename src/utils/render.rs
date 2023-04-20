use actix_web::{http::header::ContentType, HttpResponse, HttpResponseBuilder};
use askama::Template;

use super::errors::MapErrToInternal;

pub fn render_html(
    template: impl Template,
    mut http_response: HttpResponseBuilder,
) -> actix_web::Result<HttpResponse> {
    let html = template.render().map_err_to_internal()?;

    Ok(http_response.content_type(ContentType::html()).body(html))
}
