use actix_web::{get, web::Data, HttpResponse, Responder};
use serde_json::json;
use sqlx::{Pool, Sqlite};
use tera::Context;

use crate::utils::{errors::MapErrToInternal, render::render_html};

#[get("/")]
async fn index() -> actix_web::Result<HttpResponse> {
    render_html("index.html", &Context::new(), HttpResponse::Ok())
}

#[get("/db-test")]
async fn db_test(pool: Data<Pool<Sqlite>>) -> actix_web::Result<impl Responder> {
    let mut conn = pool.acquire().await.map_err_to_internal()?;

    let result = sqlx::query!("select id from (select $1+0 as id)", 16)
        .fetch_one(&mut conn)
        .await
        .map_err_to_internal()?;

    render_html(
        "db_test.html",
        &Context::from_value(json!({ "result": result.id })).map_err_to_internal()?,
        HttpResponse::Ok(),
    )
}
