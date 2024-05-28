use std::sync::Arc;
use actix_web::{delete, get, HttpResponse, patch, post, Responder};
use actix_web::web::{Data, Json, Path};
use sqlx::{MySql, Pool};
use validator::Validate;
use crate::db::service_error::ServiceError;
use crate::db::services::FeedService;
use crate::db::traits::{FeedManage, Service};
use crate::json_structs::PathId;
use crate::models::Feed;

#[utoipa::path(responses(
    (status = 200, description = "Feed get all"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/feed")]
async fn feed_get_all(feed_service: Data<Arc<FeedService<Pool<MySql>>>>) -> impl Responder{
    match feed_service.get_all().await {
        Ok(feed) => HttpResponse::Ok().json(feed),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Feed get by id"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/feed/{id}")]
async fn feed_get_by_id(feed_service: Data<Arc<FeedService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    match feed_service.get_by_id(params.id).await {
        Ok(feed) => HttpResponse::Ok().json(feed),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}

#[utoipa::path(responses(
    (status = 200, description = "Feed get all vms"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/feed-vms")]
async fn feed_get_all_vms(feed_service: Data<Arc<FeedService<Pool<MySql>>>>) -> impl Responder{
    match feed_service.get_all_vms().await {
        Ok(feed) => HttpResponse::Ok().json(feed),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(responses(
    (status = 200, description = "Feed created"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[post("/feed/create")]
async fn feed_create(feed_service: Data<Arc<FeedService<Pool<MySql>>>>, feed_json: Json<Feed>) -> impl Responder{
    let feed = match feed_json.validate() {
        Ok(_) => feed_json.into_inner(),
        Err(error) => return HttpResponse::BadRequest().json(ServiceError::ValidationError(error).to_string())
    };
    match feed_service.create(feed).await {
        Ok(created_feed) => HttpResponse::Ok().json(created_feed),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(responses(
    (status = 200, description = "Feed updated"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[patch("/feed/update")]
async fn feed_update(feed_service: Data<Arc<FeedService<Pool<MySql>>>>, feed_json: Json<Feed>) -> impl Responder{
    let feed = match feed_json.validate() {
        Ok(_) => feed_json.into_inner(),
        Err(error) => return HttpResponse::BadRequest().json(ServiceError::ValidationError(error).to_string())
    };
    match feed_service.update(feed).await {
        Ok(created_feed) => HttpResponse::Ok().json(created_feed),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Feed deleted"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[delete("/feed/delete/{id}")]
async fn feed_delete(feed_service: Data<Arc<FeedService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    match feed_service.delete(params.id).await {
        Ok(_) => HttpResponse::Ok().json("Deleted"),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string())
    }
}