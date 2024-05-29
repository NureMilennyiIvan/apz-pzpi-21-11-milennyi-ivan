use std::env;
use std::net::Ipv4Addr;
use std::sync::Arc;
use actix_cors::Cors;
use actix_web::{App, HttpServer, main};
use actix_web::http::header;
use actix_web::web::Data;
use dotenv::dotenv;
use sqlx::{MySql, Pool};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::configs::{api_doc, breed_configure, feed_configure, feed_supply_configure, feeding_log_configure, shearing_log_configure, sheep_configure, shepherd_configure, storekeeper_configure, temperature_scanner_configure};
use db::traits::Service;
use crate::db::db_context::DbContextMySql;
use crate::db::services::{BreedService, FeedingLogService, FeedService, FeedSupplyService, ShearingLogService, SheepService, ShepherdService, StorekeeperService, TemperatureScannerService};
use crate::db::traits::Context;

mod configs;
mod db;
mod models;
mod view_models;
mod endpoints;
mod json_structs;


#[main]
async fn main() -> std::io::Result<()>{

    dotenv().ok();


    let app_ip_bytes: Vec<u8> = env::var("APP_IP").unwrap_or_else(|_| "".to_string())
        .split('.')
        .map(|byte| byte.parse::<u8>().expect("Invalid number in APP_IP"))
        .collect();
    let app_ip: Ipv4Addr = Ipv4Addr::new(app_ip_bytes[0], app_ip_bytes[1], app_ip_bytes[2], app_ip_bytes[3]);
    let app_port: u16 = env::var("APP_PORT").unwrap_or_else(|_| "".to_string())
        .parse().expect("Invalid app port number");

    let db_string = env::var("DATABASE_URL").unwrap_or_else(|_| "".to_string());
    let db_context: DbContextMySql<Pool<MySql>> = DbContextMySql::new(db_string).await;

    let breed_service: Arc<BreedService<Pool<MySql>>> = Arc::new(BreedService::new(db_context.get_pool()));
    let feed_service: Arc<FeedService<Pool<MySql>>> = Arc::new(FeedService::new(db_context.get_pool()));
    let feed_supply_service: Arc<FeedSupplyService<Pool<MySql>>> = Arc::new(FeedSupplyService::new(db_context.get_pool()));
    let feeding_log_service: Arc<FeedingLogService<Pool<MySql>>> = Arc::new(FeedingLogService::new(db_context.get_pool()));
    let shearing_log_service: Arc<ShearingLogService<Pool<MySql>>> = Arc::new(ShearingLogService::new(db_context.get_pool()));
    let sheep_service: Arc<SheepService<Pool<MySql>>> = Arc::new(SheepService::new(db_context.get_pool()));
    let shepherd_service: Arc<ShepherdService<Pool<MySql>>> = Arc::new(ShepherdService::new(db_context.get_pool()));
    let storekeeper_service: Arc<StorekeeperService<Pool<MySql>>> = Arc::new(StorekeeperService::new(db_context.get_pool()));
    let temperature_service: Arc<TemperatureScannerService<Pool<MySql>>> = Arc::new(TemperatureScannerService::new(db_context.get_pool()));

    let openapi = api_doc::ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(breed_service.clone()))
            .app_data(Data::new(feed_service.clone()))
            .app_data(Data::new(feed_supply_service.clone()))
            .app_data(Data::new(feeding_log_service.clone()))
            .app_data(Data::new(shearing_log_service.clone()))
            .app_data(Data::new(sheep_service.clone()))
            .app_data(Data::new(shepherd_service.clone()))
            .app_data(Data::new(storekeeper_service.clone()))
            .app_data(Data::new(temperature_service.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::ACCESS_CONTROL_REQUEST_METHOD, header::ACCESS_CONTROL_REQUEST_HEADERS, header::ORIGIN])
                    .allowed_header(header::CONTENT_TYPE)
            )
            .configure(breed_configure)
            .configure(feed_configure)
            .configure(feed_supply_configure)
            .configure(feeding_log_configure)
            .configure(shearing_log_configure)
            .configure(sheep_configure)
            .configure(shepherd_configure)
            .configure(storekeeper_configure)
            .configure(temperature_scanner_configure)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    }).bind((app_ip, app_port))?.run().await
}