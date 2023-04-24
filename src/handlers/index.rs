use actix_web::{
    body::BoxBody, http::header::ContentType, HttpResponse, HttpResponseBuilder, Responder,
};
use askama::Template;

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
