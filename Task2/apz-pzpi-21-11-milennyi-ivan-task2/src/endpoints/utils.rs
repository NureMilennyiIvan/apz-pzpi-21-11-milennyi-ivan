use std::fmt::Debug;
use actix_web::HttpResponse;
use actix_web::web::Json;
use log::{error, info};
use serde::Serialize;
use validator::Validate;
use crate::db::service_error::ServiceError;

pub(super) fn validate_json_body<T: Validate + Debug>(body: Json<T>) -> Result<T, HttpResponse>{
    match body.validate() {
        Ok(_) => Ok(body.into_inner()),
        Err(error) => {
            error!("Validation error of {:?}\nError: {:?}", body, &error);
            Err(HttpResponse::BadRequest().json(ServiceError::ValidationError(error).to_string()))
        }
    }
}

pub(super) fn send_service_result<T: Serialize + Debug>(result: Result<T, ServiceError>) -> HttpResponse{
    match result{
        Ok(res) => {
            info!("Request result: {:?}", &res);
            HttpResponse::Ok().json(res)
        },
        Err(error) =>{
            error!("Request error: {:?}", &error);
            HttpResponse::InternalServerError().json(error.to_string())
        }
    }
}
pub(super) fn send_service_message(result: Result<(), ServiceError>, message: &str) -> HttpResponse{
    match result{
        Ok(_) => {
            info!("Request message: {:?}", message);
            HttpResponse::Ok().json(message)
        },
        Err(error) =>{
            error!("Request error: {:?}", &error);
            HttpResponse::InternalServerError().json(error.to_string())
        }
    }
}