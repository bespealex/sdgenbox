use actix_web::{get, web::Data, HttpResponse, Responder};
use askama::Template;
use sqlx::{Pool, Sqlite};

use crate::utils::{errors::MapErrToInternal, render::render_html};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[get("/")]
async fn index() -> actix_web::Result<HttpResponse> {
    render_html(IndexTemplate, HttpResponse::Ok())
}

#[derive(Template)]
#[template(path = "db_test.html")]
struct DBTestTemplate {
    result: i64,
}

#[get("/db-test")]
async fn db_test(pool: Data<Pool<Sqlite>>) -> actix_web::Result<impl Responder> {
    let mut conn = pool.acquire().await.map_err_to_internal()?;

    let result = sqlx::query_scalar!("select id from (select $1+0 as id)", 16)
        .fetch_one(&mut conn)
        .await
        .map_err_to_internal()?
        .unwrap();

    render_html(
        DBTestTemplate {
            result: result as i64,
        },
        HttpResponse::Ok(),
    )
}
