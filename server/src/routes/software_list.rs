use crate::errors::ApiErrors;
use actix_web::get;
use actix_web::web::Data;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Software {
    uuid: Uuid,
    long_name: String,
    short_name: String,
    developer: Option<String>,
    web_site: Option<String>,
}

#[tracing::instrument(name = "software_list", skip(pool))]
#[get("/software")]
pub async fn software_list(pool: Data<PgPool>) -> Result<HttpResponse, ApiErrors> {
    let pool = pool.into_inner();

    let result = sqlx::query_as!(
        Software,
        "select s.uuid, s.long_name, s.short_name, s.developer, s.web_site from software s"
    )
    .fetch_all(&*pool)
    .await
    .map_err(ApiErrors::Sqlx)?;

    Ok(HttpResponse::Ok().json(result))
}
