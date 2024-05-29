use std::sync::Arc;
use actix_web::{delete, get, HttpResponse, patch, post, Responder};
use actix_web::web::{Data, Json, Path};
use sqlx::{MySql, Pool};
use crate::db::services::FeedService;
use crate::db::traits::{FeedManage, Service};
use crate::endpoints::utils::{send_service_result, validate_json_body};
use crate::json_structs::PathId;
use crate::models::Feed;

#[utoipa::path(responses(
    (status = 200, description = "Feed get all"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/feed")]
async fn feed_get_all(feed_service: Data<Arc<FeedService<Pool<MySql>>>>) -> impl Responder{
    let result = feed_service.get_all().await;
    send_service_result(result)
}


#[utoipa::path(params(PathId), responses(
    (status = 200, description = "Feed get by id"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/feed/{id}")]
async fn feed_get_by_id(feed_service: Data<Arc<FeedService<Pool<MySql>>>>, params_url: Path<PathId>) -> impl Responder{
    let params = params_url.into_inner();
    let result = feed_service.get_by_id(params.id).await;
    send_service_result(result)
}

#[utoipa::path(responses(
    (status = 200, description = "Feed get all vms"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[get("/feed-vms")]
async fn feed_get_all_vms(feed_service: Data<Arc<FeedService<Pool<MySql>>>>) -> impl Responder{
    let result = feed_service.get_all_vms().await;
    send_service_result(result)
}


#[utoipa::path(responses(
    (status = 200, description = "Feed created"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[post("/feed/create")]
async fn feed_create(feed_service: Data<Arc<FeedService<Pool<MySql>>>>, feed_json: Json<Feed>) -> impl Responder{
    let feed = match validate_json_body(feed_json) {
        Ok(feed) => feed,
        Err(error_response) => return error_response,
    };
    let result = feed_service.create(feed).await;
    send_service_result(result)
}


#[utoipa::path(responses(
    (status = 200, description = "Feed updated"),
    (status = 400, description = "Validation error or bad request"),
    (status = 500, description = "Internal server error")
))]
#[patch("/feed/update")]
async fn feed_update(feed_service: Data<Arc<FeedService<Pool<MySql>>>>, feed_json: Json<Feed>) -> impl Responder{
    let feed = match validate_json_body(feed_json) {
        Ok(feed) => feed,
        Err(error_response) => return error_response,
    };
    let result = feed_service.update(feed).await;
    send_service_result(result)
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