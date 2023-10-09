use crate::errors::ApiErrors;
use actix_web::web::{Data, Path};
use actix_web::{post, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct UpdateRequest {
    software_uuid: Uuid,
    system_uuid: Uuid,
}

#[derive(Serialize)]
struct QueryResult {
    package_name: String,
    command: String,
    update_instruction: String,
}

#[tracing::instrument(name = "update_instruction", skip(pool, params))]
#[post("/software/{software_uuid}/system/{system_uuid}")]
pub async fn update_instruction(
    pool: Data<PgPool>,
    params: Path<UpdateRequest>,
) -> Result<HttpResponse, ApiErrors> {
    let params = params.into_inner();

    let result = sqlx::query_as!( QueryResult, "select pk.package_name, pk.command, pk.update_instruction from
( select * from packages pk join package_manger pm on pm.uuid = pk.package_manger where pm.system =$1) pk
where pk.software = (select s.uuid from software s where s.uuid = $2);", params.system_uuid, params.software_uuid)
        .fetch_one( &*pool.into_inner())
        .await
        .map_err( ApiErrors::Sqlx)?;

    Ok(HttpResponse::Ok().json(result))
}
