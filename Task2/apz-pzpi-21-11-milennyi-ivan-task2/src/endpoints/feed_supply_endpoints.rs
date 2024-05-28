use std::sync::Arc;
use actix_web::{delete, get, HttpResponse, post, Responder};
use actix_web::web::{Data, Json, Path};
use sqlx::{MySql, Pool};
use validator::Validate;
use crate::db::service_error::ServiceError;
use crate::db::services::FeedSupplyService;
use crate::db::traits::{FeedSupplyManage, Service};
use crate::json_structs::PathId;
use crate::models::FeedSupply;

#[utoipa::path(responses(
    (status = 200, description = "Feed supply get all"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/feed-supply")]
async fn feed_supply_get_all(feed_supply_service: Data<Arc<FeedSupplyService<Pool<MySql>>>>) -> impl Responder{
    match feed_supply_service.get_all().await {
        Ok(feed_supply) => HttpResponse::Ok().json(feed_supply),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Feed supply get by id"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/feed-supply/{id}")]
async fn feed_supply_get_by_id(feed_supply_service: Data<Arc<FeedSupplyService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    match feed_supply_service.get_by_id(params.id).await {
        Ok(feed_supply) => HttpResponse::Ok().json(feed_supply),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}

#[utoipa::path(responses(
    (status = 200, description = "Feed supply get all vms"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/feed-supply-vms")]
async fn feed_supply_get_all_vms(feed_supply_service: Data<Arc<FeedSupplyService<Pool<MySql>>>>) -> impl Responder{
    match feed_supply_service.get_all_vms().await {
        Ok(feed_supply) => HttpResponse::Ok().json(feed_supply),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(responses(
    (status = 200, description = "Feed supply created"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[post("/feed-supply/create")]
async fn feed_supply_create(feed_supply_service: Data<Arc<FeedSupplyService<Pool<MySql>>>>, feed_supply_json: Json<FeedSupply>) -> impl Responder{
    let feed_supply = match feed_supply_json.validate() {
        Ok(_) => feed_supply_json.into_inner(),
        Err(error) => return HttpResponse::BadRequest().json(ServiceError::ValidationError(error).to_string())
    };
    match feed_supply_service.create(feed_supply).await {
        Ok(created_feed_supply) => HttpResponse::Ok().json(created_feed_supply),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Feed supply deleted"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[delete("/feed-supply/delete/{id}")]
async fn feed_supply_delete(feed_supply_service: Data<Arc<FeedSupplyService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    match feed_supply_service.delete(params.id).await {
        Ok(_) => HttpResponse::Ok().json("Deleted"),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}