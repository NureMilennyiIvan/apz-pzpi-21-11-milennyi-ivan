use std::sync::Arc;
use actix_web::{delete, get, HttpResponse, patch, post, Responder};
use actix_web::web::{Data, Json, Path};
use sqlx::{MySql, Pool};
use validator::Validate;
use crate::db::service_error::ServiceError;
use crate::db::services::StorekeeperService;
use crate::db::traits::{Service, AuthService};
use crate::json_structs::PathId;
use crate::models::Storekeeper;

#[utoipa::path(responses(
    (status = 200, description = "Storekeeper get all"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/storekeeper")]
async fn storekeeper_get_all(storekeeper_service: Data<Arc<StorekeeperService<Pool<MySql>>>>) -> impl Responder{
    match storekeeper_service.get_all().await {
        Ok(storekeeper) => HttpResponse::Ok().json(storekeeper),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Storekeeper get by id"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/storekeeper/{id}")]
async fn storekeeper_get_by_id(storekeeper_service: Data<Arc<StorekeeperService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    match storekeeper_service.get_by_id(params.id).await {
        Ok(storekeeper) => HttpResponse::Ok().json(storekeeper),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(responses(
    (status = 200, description = "Storekeeper created"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[post("/storekeeper/create")]
async fn storekeeper_create(storekeeper_service: Data<Arc<StorekeeperService<Pool<MySql>>>>, storekeeper_json: Json<Storekeeper>) -> impl Responder{
    let storekeeper = match storekeeper_json.validate() {
        Ok(_) => storekeeper_json.into_inner(),
        Err(error) => return HttpResponse::BadRequest().json(ServiceError::ValidationError(error).to_string())
    };
    match storekeeper_service.check_username(&storekeeper).await{
        Ok(res) => if res{
            return HttpResponse::BadRequest().json(ServiceError::UniqueError.to_string())
        },
        Err(error) => return HttpResponse::InternalServerError().json(error.to_string())
    }
    match storekeeper_service.create(storekeeper).await {
        Ok(created_storekeeper) => HttpResponse::Ok().json(created_storekeeper),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(responses(
    (status = 200, description = "Storekeeper updated"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[patch("/storekeeper/update")]
async fn storekeeper_update(storekeeper_service: Data<Arc<StorekeeperService<Pool<MySql>>>>, storekeeper_json: Json<Storekeeper>) -> impl Responder{
    let storekeeper = match storekeeper_json.validate() {
        Ok(_) => storekeeper_json.into_inner(),
        Err(error) => return HttpResponse::BadRequest().json(ServiceError::ValidationError(error).to_string())
    };
    match storekeeper_service.update(storekeeper).await {
        Ok(updated_storekeeper) => HttpResponse::Ok().json(updated_storekeeper),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Storekeeper deleted"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[delete("/storekeeper/delete/{id}")]
async fn storekeeper_delete(storekeeper_service: Data<Arc<StorekeeperService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    match storekeeper_service.delete(params.id).await {
        Ok(_) => HttpResponse::Ok().json("Deleted"),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}