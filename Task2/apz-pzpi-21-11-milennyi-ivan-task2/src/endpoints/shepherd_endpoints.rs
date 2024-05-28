use std::sync::Arc;
use actix_web::{delete, get, HttpResponse, patch, post, Responder};
use actix_web::web::{Data, Json, Path};
use sqlx::{MySql, Pool};
use validator::Validate;
use crate::db::service_error::ServiceError;
use crate::db::services::ShepherdService;
use crate::db::traits::{AuthService, Service};
use crate::json_structs::{AuthorizeJson, PathId};
use crate::models::Shepherd;

#[utoipa::path(responses(
    (status = 200, description = "Shepherd get all"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/shepherd")]
async fn shepherd_get_all(shepherd_service: Data<Arc<ShepherdService<Pool<MySql>>>>) -> impl Responder{
    match shepherd_service.get_all().await {
        Ok(shepherd) => HttpResponse::Ok().json(shepherd),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Shepherd get by id"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/shepherd/{id}")]
async fn shepherd_get_by_id(shepherd_service: Data<Arc<ShepherdService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    match shepherd_service.get_by_id(params.id).await {
        Ok(shepherd) => HttpResponse::Ok().json(shepherd),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}



#[utoipa::path(responses(
    (status = 200, description = "Shepherd authorize"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[post("/shepherd/authorize")]
async fn shepherd_authorize(shepherd_service: Data<Arc<ShepherdService<Pool<MySql>>>>, authorize_json: Json<AuthorizeJson>) -> impl Responder{
    let authorize = match authorize_json.validate() {
        Ok(_) => authorize_json.into_inner(),
        Err(error) => return HttpResponse::BadRequest().json(ServiceError::ValidationError(error).to_string())
    };
    match shepherd_service.authorize(authorize.username, authorize.password_hash).await {
        Ok(shepherd) => HttpResponse::Ok().json(shepherd),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(responses(
    (status = 200, description = "Shepherd created"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[post("/shepherd/create")]
async fn shepherd_create(shepherd_service: Data<Arc<ShepherdService<Pool<MySql>>>>, shepherd_json: Json<Shepherd>) -> impl Responder{
    let shepherd = match shepherd_json.validate() {
        Ok(_) => shepherd_json.into_inner(),
        Err(error) => return HttpResponse::BadRequest().json(ServiceError::ValidationError(error).to_string())
    };
    match shepherd_service.check_username(&shepherd).await{
        Ok(res) => if res{
            return HttpResponse::BadRequest().json(ServiceError::UniqueError.to_string())
        },
        Err(error) => return HttpResponse::InternalServerError().json(error.to_string())
    }
    match shepherd_service.create(shepherd).await {
        Ok(created_shepherd) => HttpResponse::Ok().json(created_shepherd),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(responses(
    (status = 200, description = "Shepherd updated"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[patch("/shepherd/update")]
async fn shepherd_update(shepherd_service: Data<Arc<ShepherdService<Pool<MySql>>>>, shepherd_json: Json<Shepherd>) -> impl Responder{
    let shepherd = match shepherd_json.validate() {
        Ok(_) => shepherd_json.into_inner(),
        Err(error) => return HttpResponse::BadRequest().json(ServiceError::ValidationError(error).to_string())
    };
    match shepherd_service.update(shepherd).await {
        Ok(updated_shepherd) => HttpResponse::Ok().json(updated_shepherd),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Shepherd deleted"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[delete("/shepherd/delete/{id}")]
async fn shepherd_delete(shepherd_service: Data<Arc<ShepherdService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    match shepherd_service.delete(params.id).await {
        Ok(_) => HttpResponse::Ok().json("Deleted"),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}