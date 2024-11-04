use actix_web::{HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use log::{debug, error};
use crate::services::db_service::DbService;

#[derive(Serialize, Deserialize)]
pub struct SimpleRedemptionCode {
    pub code: String,
    pub rewards: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleCodeResponse {
    pub active: Vec<SimpleRedemptionCode>,
    pub inactive: Vec<SimpleRedemptionCode>,
}

pub async fn get_codes() -> impl Responder {
    debug!("Handling request to get redemption codes");
    
    match DbService::new().await {
        Ok(db_service) => {
            match db_service.get_codes().await {
                Ok((active, inactive)) => {
                    debug!("Returning {} active and {} inactive codes", active.len(), inactive.len());
                    let response = SimpleCodeResponse {
                        active: active.into_iter()
                            .map(|code| SimpleRedemptionCode {
                                code: code.code,
                                rewards: code.rewards,
                            })
                            .collect(),
                        inactive: inactive.into_iter()
                            .map(|code| SimpleRedemptionCode {
                                code: code.code,
                                rewards: code.rewards,
                            })
                            .collect(),
                    };
                    HttpResponse::Ok().json(response)
                },
                Err(e) => {
                    error!("Failed to get codes from database: {}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to fetch codes from database"
                    }))
                }
            }
        },
        Err(e) => {
            error!("Failed to initialize database service: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to initialize database service"
            }))
        }
    }
} 