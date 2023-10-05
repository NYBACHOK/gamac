use crate::errors::ApiErrors;
use actix_web::web::Data;
use actix_web::{get, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Systems {
    uuid: Uuid,
    os_name: String,
    web_site: Option<String>,
}

#[tracing::instrument(name = "systems_list", skip(pool))]
#[get("/system")]
pub async fn systems_list(pool: Data<PgPool>) -> Result<HttpResponse, ApiErrors> {
    let pool = pool.into_inner();

    let result = sqlx::query_as!(
        Systems,
        "select s.uuid, s.os_name, s.web_site from system s;"
    )
    .fetch_all(&*pool)
    .await
    .map_err(ApiErrors::Sqlx)?;

    Ok(HttpResponse::Ok().json(result))
}
