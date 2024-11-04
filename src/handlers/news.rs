use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use log::{debug, error};
use crate::services::news_service::NewsService;
use crate::utils::lang_parser::parse_language_code;

#[derive(Deserialize)]
pub struct NewsQueryParams {
    pub lang: Option<String>,
}

pub async fn get_news_events(query: web::Query<NewsQueryParams>) -> impl Responder {
    debug!("Handling request to get news events");
    let lang = parse_language_code(query.lang.as_deref().unwrap_or("en"));
    
    match NewsService::new().await {
        Ok(news_service) => {
            match news_service.get_news(Some("event"), Some(lang)).await {
                Ok(news) => {
                    debug!("Returning {} event news items for language {}", news.len(), lang);
                    HttpResponse::Ok().json(news)
                },
                Err(e) => {
                    error!("Failed to fetch event news: {}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to fetch event news"
                    }))
                }
            }
        },
        Err(e) => {
            error!("Failed to initialize news service: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to initialize news service"
            }))
        }
    }
}

pub async fn get_news_notices(query: web::Query<NewsQueryParams>) -> impl Responder {
    debug!("Handling request to get news notices");
    let lang = parse_language_code(query.lang.as_deref().unwrap_or("en"));
    
    match NewsService::new().await {
        Ok(news_service) => {
            match news_service.get_news(Some("notice"), Some(lang)).await {
                Ok(news) => {
                    debug!("Returning {} notice news items for language {}", news.len(), lang);
                    HttpResponse::Ok().json(news)
                },
                Err(e) => {
                    error!("Failed to fetch notice news: {}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to fetch notice news"
                    }))
                }
            }
        },
        Err(e) => {
            error!("Failed to initialize news service: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to initialize news service"
            }))
        }
    }
}

pub async fn get_news_info(query: web::Query<NewsQueryParams>) -> impl Responder {
    debug!("Handling request to get news info");
    let lang = parse_language_code(query.lang.as_deref().unwrap_or("en"));
    
    match NewsService::new().await {
        Ok(news_service) => {
            match news_service.get_news(Some("info"), Some(lang)).await {
                Ok(news) => {
                    debug!("Returning {} info news items for language {}", news.len(), lang);
                    HttpResponse::Ok().json(news)
                },
                Err(e) => {
                    error!("Failed to fetch info news: {}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to fetch info news"
                    }))
                }
            }
        },
        Err(e) => {
            error!("Failed to initialize news service: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to initialize news service"
            }))
        }
    }
} 