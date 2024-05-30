use std::sync::Arc;
use actix_web::{delete, get, patch, post, Responder};
use actix_web::web::{Data, Json, Path};
use sqlx::{MySql, Pool};
use crate::db::services::TemperatureScannerService;
use crate::db::traits::{Service, TemperatureScannerManage};
use crate::endpoints::utils::{send_service_message, send_service_result, validate_json_body};
use crate::json_structs::{PathId, TemperatureScannerAuthJson, TemperatureScannerTempJson};
use crate::models::TemperatureScanner;

#[utoipa::path(responses(
    (status = 200, description = "Temperature scanner get all"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/temperature-scanner")]
async fn temperature_scanner_get_all(temperature_scanner_service: Data<Arc<TemperatureScannerService<Pool<MySql>>>>) -> impl Responder{
    let result = temperature_scanner_service.get_all().await;
    send_service_result(result)
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Temperature scanner get by id"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/temperature-scanner/{id}")]
async fn temperature_scanner_get_by_id(temperature_scanner_service: Data<Arc<TemperatureScannerService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    let result = temperature_scanner_service.get_by_id(params.id).await;
    send_service_result(result)
}

#[utoipa::path(responses(
    (status = 200, description = "Temperature scanner authenticate"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[post("/temperature-scanner/authenticate")]
async fn temperature_scanner_authenticate(temperature_scanner_service: Data<Arc<TemperatureScannerService<Pool<MySql>>>>, temperature_scanner_authenticate_json: Json<TemperatureScannerAuthJson>) -> impl Responder{
    let temperature_scanner_authenticate = match validate_json_body(temperature_scanner_authenticate_json) {
        Ok(temperature_scanner_authenticate) => temperature_scanner_authenticate,
        Err(error_response) => return error_response,
    };
    let result = temperature_scanner_service.authenticate(temperature_scanner_authenticate.id, temperature_scanner_authenticate.hash_password).await;
    send_service_result(result)
}

#[utoipa::path(responses(
    (status = 200, description = "Temperature scanner created"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[post("/temperature-scanner/create")]
async fn temperature_scanner_create(temperature_scanner_service: Data<Arc<TemperatureScannerService<Pool<MySql>>>>, temperature_scanner_json: Json<TemperatureScanner>) -> impl Responder{
    let temperature_scanner = match validate_json_body(temperature_scanner_json) {
        Ok(temperature_scanner) => temperature_scanner,
        Err(error_response) => return error_response,
    };
    let result = temperature_scanner_service.create(temperature_scanner).await;
    send_service_result(result)
}

#[utoipa::path(responses(
    (status = 200, description = "Temperature scanner authenticate"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[patch("/temperature-scanner/update-temperature")]
async fn temperature_scanner_update_temperature(temperature_scanner_service: Data<Arc<TemperatureScannerService<Pool<MySql>>>>, temperature_scanner_update_temp_json: Json<TemperatureScannerTempJson>) -> impl Responder{
    let temperature_scanner_update_temp = match validate_json_body(temperature_scanner_update_temp_json) {
        Ok(temperature_scanner_update_temp) => temperature_scanner_update_temp,
        Err(error_response) => return error_response,
    };
    let result = temperature_scanner_service.update_temperature(temperature_scanner_update_temp.id, temperature_scanner_update_temp.temperature).await;
    send_service_result(result)
}


#[utoipa::path(responses(
    (status = 200, description = "Temperature scanner updated"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[patch("/temperature-scanner/update")]
async fn temperature_scanner_update(temperature_scanner_service: Data<Arc<TemperatureScannerService<Pool<MySql>>>>, temperature_scanner_json: Json<TemperatureScanner>) -> impl Responder{
    let temperature_scanner = match validate_json_body(temperature_scanner_json) {
        Ok(temperature_scanner) => temperature_scanner,
        Err(error_response) => return error_response,
    };
    let result = temperature_scanner_service.update(temperature_scanner).await;
    send_service_result(result)
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Temperature scanner deleted"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[delete("/temperature-scanner/delete/{id}")]
async fn temperature_scanner_delete(temperature_scanner_service: Data<Arc<TemperatureScannerService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    send_service_message(temperature_scanner_service.delete(params.id).await, "Deleted")
}