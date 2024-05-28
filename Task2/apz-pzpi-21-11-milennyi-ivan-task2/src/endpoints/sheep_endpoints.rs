use std::sync::Arc;
use actix_web::{web::{Data, Json, Path}, get, HttpResponse, post, Responder, patch, delete};
use sqlx::{MySql, Pool};
use validator::Validate;
use crate::db::service_error::ServiceError;
use crate::db::services::SheepService;
use crate::db::traits::{Service, SheepManage};
use crate::json_structs::{ChangeShepherdJson, PathId};
use crate::models::Sheep;

#[utoipa::path(responses(
    (status = 200, description = "Sheep get all"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/sheep")]
async fn sheep_get_all(sheep_service: Data<Arc<SheepService<Pool<MySql>>>>) -> impl Responder{
    match sheep_service.get_all().await {
        Ok(sheep) => HttpResponse::Ok().json(sheep),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Sheep get by id"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/sheep/{id}")]
async fn sheep_get_by_id(sheep_service: Data<Arc<SheepService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    match sheep_service.get_by_id(params.id).await {
        Ok(sheep) => HttpResponse::Ok().json(sheep),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Sheep get details by id"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/sheep/details/{id}")]
async fn sheep_get_details_by_id(sheep_service: Data<Arc<SheepService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    match sheep_service.get_details_by_id(params.id).await {
        Ok(sheep) => HttpResponse::Ok().json(sheep),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Sheep get all vms by shepherd id"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/sheep/shepherd/{id}")]
async fn sheep_get_all_vms_by_shepherd_id(sheep_service: Data<Arc<SheepService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    match sheep_service.get_all_vms_by_shepherd_id(params.id).await {
        Ok(sheep) => HttpResponse::Ok().json(sheep),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(responses(
    (status = 200, description = "Sheep created"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[post("/sheep/create")]
async fn sheep_create(sheep_service: Data<Arc<SheepService<Pool<MySql>>>>, sheep_json: Json<Sheep>) -> impl Responder{
    let sheep = match sheep_json.validate() {
        Ok(_) => sheep_json.into_inner(),
        Err(error) => return HttpResponse::BadRequest().json(ServiceError::ValidationError(error).to_string())
    };
    match sheep_service.create(sheep).await {
        Ok(created_sheep) => HttpResponse::Ok().json(created_sheep),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(responses(
    (status = 200, description = "Sheep updated"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[patch("/sheep/update")]
async fn sheep_update(sheep_service: Data<Arc<SheepService<Pool<MySql>>>>, sheep_json: Json<Sheep>) -> impl Responder{
    let sheep = match sheep_json.validate() {
        Ok(_) => sheep_json.into_inner(),
        Err(error) => return HttpResponse::BadRequest().json(ServiceError::ValidationError(error).to_string())
    };
    match sheep_service.update(sheep).await {
        Ok(created_sheep) => HttpResponse::Ok().json(created_sheep),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(responses(
    (status = 200, description = "Sheep's shepherd changed"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[patch("/sheep/change-shepherd")]
async fn sheep_change_shepherd(sheep_service: Data<Arc<SheepService<Pool<MySql>>>>, change_shepherd_json: Json<ChangeShepherdJson>) -> impl Responder{
    let change_shepherd = change_shepherd_json.into_inner();
    match sheep_service.change_shepherd(change_shepherd.sheep_id, change_shepherd.shepherd_id).await {
        Ok(_) => HttpResponse::Ok().json("Changed"),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Sheep deleted"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[delete("/sheep/delete/{id}")]
async fn sheep_delete(sheep_service: Data<Arc<SheepService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    match sheep_service.delete(params.id).await {
        Ok(_) => HttpResponse::Ok().json("Deleted"),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}