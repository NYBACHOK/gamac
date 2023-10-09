use crate::configs::ApiConfig;
use crate::errors::ApiErrors;
use crate::routes;
use crate::routes::delete_instruction::delete_instruction;
use crate::routes::install_instruction::install_instruction;
use crate::routes::software_list::software_list;
use crate::routes::system_list::systems_list;
use crate::routes::update_instruction::update_instruction;
use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::PgPool;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn startup(pool: PgPool, config: ApiConfig) -> Result<Server, ApiErrors> {
    let config_var = config.clone();
    Ok(HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(pool.clone())
            .service(
                web::scope("list")
                    .service(software_list)
                    .service(systems_list),
            )
            .service(
                web::scope("instruction")
                    .service(install_instruction)
                    .service(update_instruction)
                    .service(delete_instruction),
            )
            .route("/health", web::get().to(health_check))
    })
    .bind(format!("{}:{}", config_var.ip, config_var.port))
    .map_err(ApiErrors::IoError)?
    .run())
}
