use std::sync::Arc;
use actix_web::{delete, get, HttpResponse, post, Responder};
use actix_web::web::{Data, Json, Path};
use sqlx::{MySql, Pool};
use validator::Validate;
use crate::db::service_error::ServiceError;
use crate::db::services::ShearingLogService;
use crate::db::traits::{Service, ShearingLogManage};
use crate::json_structs::PathId;
use crate::models::ShearingLog;

#[utoipa::path(responses(
    (status = 200, description = "Shearing log get all"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/shearing-log")]
async fn shearing_log_get_all(shearing_log_service: Data<Arc<ShearingLogService<Pool<MySql>>>>) -> impl Responder{
    match shearing_log_service.get_all().await {
        Ok(shearing_log) => HttpResponse::Ok().json(shearing_log),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Shearing log get by id"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/shearing-log/{id}")]
async fn shearing_log_get_by_id(shearing_log_service: Data<Arc<ShearingLogService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    match shearing_log_service.get_by_id(params.id).await {
        Ok(shearing_log) => HttpResponse::Ok().json(shearing_log),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Shearing log all vms by sheep id"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/shearing-log/sheep/{id}")]
async fn shearing_log_get_all_vms_by_sheep_id(shearing_log_service: Data<Arc<ShearingLogService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    match shearing_log_service.get_all_vms_by_sheep_id(params.id).await {
        Ok(shearing_logs) => HttpResponse::Ok().json(shearing_logs),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(responses(
    (status = 200, description = "Shearing log created"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[post("/shearing-log/create")]
async fn shearing_log_create(shearing_log_service: Data<Arc<ShearingLogService<Pool<MySql>>>>, shearing_log_json: Json<ShearingLog>) -> impl Responder{
    let shearing_log = match shearing_log_json.validate() {
        Ok(_) => shearing_log_json.into_inner(),
        Err(error) => return HttpResponse::BadRequest().json(ServiceError::ValidationError(error).to_string())
    };
    match shearing_log_service.create(shearing_log).await {
        Ok(created_shearing_log) => HttpResponse::Ok().json(created_shearing_log),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Shearing log deleted"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[delete("/shearing-log/delete/{id}")]
async fn shearing_log_delete(shearing_log_service: Data<Arc<ShearingLogService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    match shearing_log_service.delete(params.id).await {
        Ok(_) => HttpResponse::Ok().json("Deleted"),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}