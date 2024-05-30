use actix_web::HttpResponse;
use actix_web::web::Json;
use serde::Serialize;
use validator::Validate;
use crate::db::service_error::ServiceError;

pub(super) fn validate_json_body<T: Validate>(body: Json<T>) -> Result<T, HttpResponse>{
    match body.validate() {
        Ok(_) => Ok(body.into_inner()),
        Err(error) => Err(HttpResponse::BadRequest().json(ServiceError::ValidationError(error).to_string()))
    }
}

pub(super) fn send_service_result<T: Serialize>(result: Result<T, ServiceError>) -> HttpResponse{
    match result{
        Ok(res) => HttpResponse::Ok().json(res),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}
pub(super) fn send_service_message<T: Serialize>(result: Result<T, ServiceError>, message: &str) -> HttpResponse{
    match result{
        Ok(_) => HttpResponse::Ok().json(message),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}