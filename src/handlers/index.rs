use actix_web::{
    body::BoxBody, http::header::ContentType, web::Data, HttpResponse, HttpResponseBuilder,
    Responder,
};
use askama::Template;
use sqlx::{Acquire, Pool, Sqlite};

use crate::{
    config::Config,
    models::dedup_images,
    utils::{errors::MapErrToInternal, render::render_html},
};

pub struct TemplateResponse<T: Template> {
    template: T,
    response_builder: HttpResponseBuilder,
}

impl<T: Template> TemplateResponse<T> {
    pub fn new(template: T, response_builder: HttpResponseBuilder) -> Self {
        TemplateResponse {
            template,
            response_builder,
        }
    }
}

impl<T: Template> Responder for TemplateResponse<T> {
    type Body = BoxBody;

    fn respond_to(mut self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        self.response_builder
            .content_type(ContentType::html())
            .body(self.template.render().unwrap())
    }
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

pub async fn index() -> actix_web::Result<TemplateResponse<IndexTemplate>> {
    Ok(TemplateResponse::new(IndexTemplate, HttpResponse::Ok()))
}

#[derive(Template)]
#[template(path = "deduplication_result.html")]
pub struct DeduplicationResultTemplate {
    deduplicated: usize,
}

pub async fn deduplicate_images(
    pool: Data<Pool<Sqlite>>,
    config: Data<Config>,
) -> actix_web::Result<HttpResponse> {
    let mut connection = pool.acquire().await.map_err_to_internal()?;
    let mut transaction = connection.begin().await.map_err_to_internal()?;
    let deduplicated = dedup_images(&mut transaction, &config.media_root)
        .await
        .map_err_to_internal()?;
    transaction.commit().await.map_err_to_internal()?;
    render_html(
        DeduplicationResultTemplate { deduplicated },
        HttpResponse::Ok(),
    )
}

#[cfg(test)]
mod test {
    use actix_web::{http::StatusCode, test::TestRequest, Responder};

    use crate::handlers::index::index;

    #[actix_web::test]
    async fn test_index_ok() {
        let req = TestRequest::default().to_http_request();
        let resp = index().await.unwrap().respond_to(&req);
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
