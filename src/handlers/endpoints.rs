use actix_web::{HttpResponse, Responder};
use serde::Serialize;
use log::{info, debug};

#[derive(Serialize)]
pub struct ApiEndpoints {
    pub endpoints: Vec<String>,
}

pub async fn get_api_endpoints() -> impl Responder {
    debug!("Handling request to list API endpoints");
    let endpoints = ApiEndpoints {
        endpoints: vec![
            String::from("/starrail"),
            String::from("/starrail/code"),
            String::from("/starrail/news/events"),
            String::from("/starrail/news/notices"),
            String::from("/starrail/news/info"),
        ],
    };
    info!("Returning list of API endpoints");
    HttpResponse::Ok().json(endpoints)
} 