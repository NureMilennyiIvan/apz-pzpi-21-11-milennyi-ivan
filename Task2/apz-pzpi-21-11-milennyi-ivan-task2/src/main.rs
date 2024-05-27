use std::env;
use std::net::Ipv4Addr;
use std::sync::Arc;
use actix_cors::Cors;
use actix_web::{App, HttpServer, main};
use actix_web::http::header;
use actix_web::web::Data;
use dotenv::dotenv;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::configs::api_doc;
use crate::db::{BreedService, DbContextMySql, FeedingLogsService, FeedService, FeedSupplyService, ShearingLogsService, SheepService, ShepherdService, StorekeeperService, TemperatureScannerService};
use crate::db::traits::Service;

mod configs;
mod db;

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
    let db_string = env::var("DB_STRING").unwrap_or_else(|_| "".to_string());
    let db_context = DbContextMySql::new(db_string).await;

    let breed_service: Arc<BreedService> = Arc::new(BreedService::new(db_context.get_pool()));
    let feed_service: Arc<FeedService> = Arc::new(FeedService::new(db_context.get_pool()));
    let feed_supply_service: Arc<FeedSupplyService> = Arc::new(FeedSupplyService::new(db_context.get_pool()));
    let feeding_logs_service: Arc<FeedingLogsService> = Arc::new(FeedingLogsService::new(db_context.get_pool()));
    let shearing_logs_service: Arc<ShearingLogsService> = Arc::new(ShearingLogsService::new(db_context.get_pool()));
    let sheep_service: Arc<SheepService> = Arc::new(SheepService::new(db_context.get_pool()));
    let shepherd_service: Arc<ShepherdService> = Arc::new(ShepherdService::new(db_context.get_pool()));
    let storekeeper_service: Arc<StorekeeperService> = Arc::new(StorekeeperService::new(db_context.get_pool()));
    let temperature_service: Arc<TemperatureScannerService> = Arc::new(TemperatureScannerService::new(db_context.get_pool()));

    let openapi = api_doc::ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(breed_service.clone()))
            .app_data(Data::new(feed_service.clone()))
            .app_data(Data::new(feed_supply_service.clone()))
            .app_data(Data::new(feeding_logs_service.clone()))
            .app_data(Data::new(shearing_logs_service.clone()))
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
            )/*
            .configure(buy_config::configure)
            .configure(rent_config::configure)
            .configure(buy_info_config::configure)
            .configure(rent_info_config::configure)*/
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    }).bind((app_ip, app_port))?.run().await
}