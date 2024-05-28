use std::sync::Arc;
use actix_web::{delete, get, HttpResponse, post, Responder};
use actix_web::web::{Data, Json, Path};
use sqlx::{MySql, Pool};
use validator::Validate;
use crate::db::service_error::ServiceError;
use crate::db::services::FeedingLogService;
use crate::db::traits::{FeedingLogManage, Service};
use crate::json_structs::PathId;
use crate::models::FeedingLog;

#[utoipa::path(responses(
    (status = 200, description = "Feeding log get all"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/feeding-log")]
async fn feeding_log_get_all(feeding_log_service: Data<Arc<FeedingLogService<Pool<MySql>>>>) -> impl Responder{
    match feeding_log_service.get_all().await {
        Ok(feeding_log) => HttpResponse::Ok().json(feeding_log),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Feeding log get by id"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/feeding-log/{id}")]
async fn feeding_log_get_by_id(feeding_log_service: Data<Arc<FeedingLogService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    match feeding_log_service.get_by_id(params.id).await {
        Ok(feeding_log) => HttpResponse::Ok().json(feeding_log),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Feeding log all vms by sheep id"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/feeding-log/sheep/{id}")]
async fn feeding_log_get_all_vms_by_sheep_id(feeding_log_service: Data<Arc<FeedingLogService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    match feeding_log_service.get_all_vms_by_sheep_id(params.id).await {
        Ok(feeding_logs) => HttpResponse::Ok().json(feeding_logs),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Feeding log all vms by feed id"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/feeding-log/feed/{id}")]
async fn feeding_log_get_all_vms_by_feed_id(feeding_log_service: Data<Arc<FeedingLogService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    match feeding_log_service.get_all_vms_by_feed_id(params.id).await {
        Ok(feeding_logs) => HttpResponse::Ok().json(feeding_logs),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(responses(
    (status = 200, description = "Feeding log created"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[post("/feeding-log/create")]
async fn feeding_log_create(feeding_log_service: Data<Arc<FeedingLogService<Pool<MySql>>>>, feeding_log_json: Json<FeedingLog>) -> impl Responder{
    let feeding_log = match feeding_log_json.validate() {
        Ok(_) => feeding_log_json.into_inner(),
        Err(error) => return HttpResponse::BadRequest().json(ServiceError::ValidationError(error).to_string())
    };
    match feeding_log_service.create(feeding_log).await {
        Ok(created_feeding_log) => HttpResponse::Ok().json(created_feeding_log),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Feeding log deleted"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[delete("/feeding-log/delete/{id}")]
async fn feeding_log_delete(feeding_log_service: Data<Arc<FeedingLogService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    match feeding_log_service.delete(params.id).await {
        Ok(_) => HttpResponse::Ok().json("Deleted"),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}