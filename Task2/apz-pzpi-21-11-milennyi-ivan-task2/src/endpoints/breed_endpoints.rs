use std::sync::Arc;
use actix_web::{delete, get, HttpResponse, patch, post, Responder};
use actix_web::web::{Data, Json, Path};
use sqlx::{MySql, Pool};
use crate::db::services::BreedService;
use crate::db::traits::{BreedManage, Service};
use crate::endpoints::utils::{validate_json_body, send_service_result};
use crate::json_structs::PathId;
use crate::models::Breed;


#[utoipa::path(responses(
    (status = 200, description = "Breed get all"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/breed")]
async fn breed_get_all(breed_service: Data<Arc<BreedService<Pool<MySql>>>>) -> impl Responder{
    let result = breed_service.get_all().await;
    send_service_result(result)
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Breed get by id"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/breed/{id}")]
async fn breed_get_by_id(breed_service: Data<Arc<BreedService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    let result = breed_service.get_by_id(params.id).await;
    send_service_result(result)
}

#[utoipa::path(responses(
    (status = 200, description = "Breed get all vms"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/breed-vms")]
async fn breed_get_all_vms(breed_service: Data<Arc<BreedService<Pool<MySql>>>>) -> impl Responder{
    let result = breed_service.get_all_vms().await;
    send_service_result(result)
}


#[utoipa::path(responses(
    (status = 200, description = "Breed created"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[post("/breed/create")]
async fn breed_create(breed_service: Data<Arc<BreedService<Pool<MySql>>>>, breed_json: Json<Breed>) -> impl Responder{
    let breed = match validate_json_body(breed_json) {
        Ok(breed) => breed,
        Err(error_response) => return error_response,
    };
    let result = breed_service.create(breed).await;
    send_service_result(result)
}


#[utoipa::path(responses(
    (status = 200, description = "Breed updated"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[patch("/breed/update")]
async fn breed_update(breed_service: Data<Arc<BreedService<Pool<MySql>>>>, breed_json: Json<Breed>) -> impl Responder{
    let breed = match validate_json_body(breed_json) {
        Ok(breed) => breed,
        Err(error_response) => return error_response,
    };
    let result = breed_service.update(breed).await;
    send_service_result(result)
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Breed deleted"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[delete("/breed/delete/{id}")]
async fn breed_delete(breed_service: Data<Arc<BreedService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    match breed_service.delete(params.id).await {
        Ok(_) => HttpResponse::Ok().json("Deleted"),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}