use std::sync::Arc;
use actix_web::{delete, get, HttpResponse, patch, post, Responder};
use actix_web::web::{Data, Json, Path};
use sqlx::{MySql, Pool};
use crate::db::service_error::ServiceError;
use crate::db::services::ShepherdService;
use crate::db::traits::{AuthService, Service};
use crate::endpoints::utils::{send_service_result, validate_json_body};
use crate::json_structs::{AuthorizeJson, PathId};
use crate::models::Shepherd;

#[utoipa::path(responses(
    (status = 200, description = "Shepherd get all"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/shepherd")]
async fn shepherd_get_all(shepherd_service: Data<Arc<ShepherdService<Pool<MySql>>>>) -> impl Responder{
    let result = shepherd_service.get_all().await;
    send_service_result(result)
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Shepherd get by id"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/shepherd/{id}")]
async fn shepherd_get_by_id(shepherd_service: Data<Arc<ShepherdService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    let result = shepherd_service.get_by_id(params.id).await;
    send_service_result(result)
}



#[utoipa::path(responses(
    (status = 200, description = "Shepherd authorize"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[post("/shepherd/authorize")]
async fn shepherd_authorize(shepherd_service: Data<Arc<ShepherdService<Pool<MySql>>>>, authorize_json: Json<AuthorizeJson>) -> impl Responder{
    let authorize = match validate_json_body(authorize_json) {
        Ok(authorize) => authorize,
        Err(error_response) => return error_response,
    };
    let result = shepherd_service.authorize(authorize.username, authorize.password_hash).await;
    send_service_result(result)
}


#[utoipa::path(responses(
    (status = 200, description = "Shepherd created"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[post("/shepherd/create")]
async fn shepherd_create(shepherd_service: Data<Arc<ShepherdService<Pool<MySql>>>>, shepherd_json: Json<Shepherd>) -> impl Responder{
    let shepherd = match validate_json_body(shepherd_json) {
        Ok(shepherd) => shepherd,
        Err(error_response) => return error_response,
    };
    match shepherd_service.check_username(&shepherd).await{
        Ok(res) => if res{
            return HttpResponse::BadRequest().json(ServiceError::UniqueError.to_string())
        },
        Err(error) => return HttpResponse::InternalServerError().json(error.to_string())
    }
    let result = shepherd_service.create(shepherd).await;
    send_service_result(result)
}


#[utoipa::path(responses(
    (status = 200, description = "Shepherd updated"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[patch("/shepherd/update")]
async fn shepherd_update(shepherd_service: Data<Arc<ShepherdService<Pool<MySql>>>>, shepherd_json: Json<Shepherd>) -> impl Responder{
    let shepherd = match validate_json_body(shepherd_json) {
        Ok(shepherd) => shepherd,
        Err(error_response) => return error_response,
    };
    let result = shepherd_service.update(shepherd).await;
    send_service_result(result)
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