mod config;
mod logger;
mod resolvers;
mod services;
mod scheduler;
mod utils;
mod handlers;

use actix_web::{web, App, HttpServer, middleware::Logger};
use log::{info, error};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use actix_web::error::ErrorTooManyRequests;
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::task::{Context, Poll};
use crate::services::rate_limiter::RateLimiter;
use std::sync::Arc;
use crate::config::Settings;
use crate::handlers::{
    endpoints::get_api_endpoints,
    codes::get_codes,
    news::{get_news_events, get_news_notices, get_news_info},
};

pub struct RateLimiterMiddleware {
    rate_limiter: Arc<RateLimiter>,
}

impl RateLimiterMiddleware {
    pub fn new(max_requests: u32, window_seconds: u64) -> Self {
        Self {
            rate_limiter: Arc::new(RateLimiter::new(max_requests, window_seconds)),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimiterMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RateLimiterMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimiterMiddlewareService {
            service,
            rate_limiter: self.rate_limiter.clone(),
        }))
    }
}

pub struct RateLimiterMiddlewareService<S> {
    service: S,
    rate_limiter: Arc<RateLimiter>,
}

impl<S, B> Service<ServiceRequest> for RateLimiterMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let rate_limiter = self.rate_limiter.clone();
        
        let ip = req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string();
            
        let fut = self.service.call(req);

        Box::pin(async move {
            if !rate_limiter.check_rate_limit(&ip).await {
                return Err(ErrorTooManyRequests("Rate limit exceeded"));
            }

            fut.await
        })
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::init_logger();
    
    info!("Starting Starrail API server");
    
    let config = Settings::new().expect("Failed to load configuration");
    info!("Configuration loaded successfully");
    
    if let Err(e) = scheduler::init_scheduler().await {
        error!("Failed to initialize scheduler: {}", e);
    }
    
    info!("Server running at http://{}:{}", config.server.host, config.server.port);
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
            .wrap(RateLimiterMiddleware::new(60, 60))
            .route("/starrail", web::get().to(get_api_endpoints))
            .route("/starrail/code", web::get().to(get_codes))
            .route("/starrail/news/events", web::get().to(get_news_events))
            .route("/starrail/news/notices", web::get().to(get_news_notices))
            .route("/starrail/news/info", web::get().to(get_news_info))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))
    .map_err(|e| {
        error!("Failed to bind to address: {}", e);
        e
    })?
    .run()
    .await
}
