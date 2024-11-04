use crate::config::Settings;
use crate::resolvers::RedemptionCode;
use log::{info, error, warn};
use serde::Deserialize;
use reqwest::Client;

#[derive(Debug, Deserialize)]
struct HoyolabResponse {
    retcode: i32,
    message: String,
}

#[derive(Debug)]
pub enum ValidationResult {
    Valid,
    AlreadyRedeemed,
    Expired,
    Invalid,
    Cooldown,
    InvalidCredentials,
    Unknown(i32, String),
}

pub struct ValidatorService {
    client: Client,
    config: Settings,
}

impl ValidatorService {
    pub fn new() -> Self {
        let client = Client::new();
        let config = Settings::new().expect("Failed to load configuration");
        Self { client, config }
    }

    pub async fn validate_code(&self, code: &RedemptionCode) -> Result<ValidationResult, Box<dyn std::error::Error + Send + Sync>> {
        let url = "https://sg-hkrpg-api.hoyoverse.com/common/apicdkey/api/webExchangeCdkey";
        
        let timestamp = chrono::Utc::now().timestamp_millis();
        let game_biz = String::from("hkrpg_global");
        let lang = String::from("en");
        
        let response = self.client
            .get(url)
            .header("User-Agent", &self.config.hoyolab.user_agent)
            .header("Cookie", &self.config.hoyolab.cookie)
            .query(&[
                ("cdkey", &code.code),
                ("game_biz", &game_biz),
                ("lang", &lang),
                ("region", &self.config.hoyolab.region),
                ("t", &timestamp.to_string()),
                ("uid", &self.config.hoyolab.uid),
            ])
            .send()
            .await?;

        let status = response.status();
        
        if !status.is_success() {
            error!("Failed HTTP request for code {}: Status {}", code.code, status);
            return Ok(ValidationResult::Unknown(status.as_u16() as i32, "HTTP request failed".to_string()));
        }

        let response_body: HoyolabResponse = response.json().await?;
        
        let result = match response_body.retcode {
            0 => ValidationResult::Valid,
            -2017 | -2018 => {
                warn!("Code {} is already redeemed", code.code);
                ValidationResult::AlreadyRedeemed
            },
            -2001 => {
                warn!("Code {} is expired", code.code);
                ValidationResult::Expired
            },
            -2003 => {
                warn!("Code {} is invalid", code.code);
                ValidationResult::Invalid
            },
            -2016 => {
                warn!("Code {} is in cooldown", code.code);
                ValidationResult::Cooldown
            },
            -1071 => {
                error!("Invalid account credentials");
                ValidationResult::InvalidCredentials
            },
            _ => {
                error!("Unknown response code {} for code {}: {}", 
                    response_body.retcode, code.code, response_body.message);
                ValidationResult::Unknown(response_body.retcode, response_body.message)
            }
        };

        info!("Validation result for code {}: {:?}", code.code, result);
        Ok(result)
    }
} 