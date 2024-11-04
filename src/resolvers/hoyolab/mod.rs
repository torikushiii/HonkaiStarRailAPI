use async_trait::async_trait;
use log::{debug, error, info};
use reqwest::Client;
use serde::Deserialize;
use mongodb::bson::DateTime;

use super::{CodeResolver, RedemptionCode};

#[derive(Debug, Deserialize)]
struct HoyolabResponse {
    retcode: i32,
    message: String,
    data: Option<HoyolabData>,
}

#[derive(Debug, Deserialize)]
struct HoyolabData {
    modules: Vec<Module>,
}

#[derive(Debug, Deserialize)]
struct Module {
    exchange_group: Option<ExchangeGroup>,
}

#[derive(Debug, Deserialize)]
struct ExchangeGroup {
    bonuses: Vec<Bonus>,
}

#[derive(Debug, Deserialize)]
struct Bonus {
    exchange_code: String,
    code_status: String,
    icon_bonuses: Vec<IconBonus>,
}

#[derive(Debug, Deserialize)]
struct IconBonus {
    bonus_num: String,
    icon_url: String,
}

pub struct HoyolabResolver {
    client: Client,
}

impl HoyolabResolver {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    fn map_reward_type(&self, icon_url: &str) -> &str {
        match icon_url {
            url if url.contains("77cb5426637574ba524ac458fa963da0_6409817950389238658") => "Stellar Jade",
            url if url.contains("7cb0e487e051f177d3f41de8d4bbc521_2556290033227986328") => "Refined Aether",
            url if url.contains("508229a94e4fa459651f64c1cd02687a_6307505132287490837") => "Traveler's Guide",
            url if url.contains("0b12bdf76fa4abc6b4d1fdfc0fb4d6f5_4521150989210768295") => "Credit",
            _ => "Unknown",
        }
    }
}

#[async_trait]
impl CodeResolver for HoyolabResolver {
    fn name(&self) -> String {
        "Hoyolab".to_string()
    }

    fn base_url(&self) -> String {
        "https://bbs-api-os.hoyolab.com/community/painter/wapi/circle/channel/guide/material".to_string()
    }

    async fn fetch_codes(&self) -> Result<Vec<RedemptionCode>, Box<dyn std::error::Error + Send + Sync>> {
        debug!("Fetching codes from Hoyolab API");
        
        let response = self.client
            .get(self.base_url())
            .query(&[("game_id", "6")])
            .header("x-rpc-app_version", "2.42.0")
            .header("x-rpc-client_type", "4")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36")
            .send()
            .await?;

        if !response.status().is_success() {
            error!("Failed to fetch Hoyolab API: {}", response.status());
            return Err("Failed to fetch Hoyolab API".into());
        }

        let hoyolab_response: HoyolabResponse = response.json().await?;
        
        if hoyolab_response.retcode != 0 {
            info!("Hoyolab API returned non-zero retcode: {} - {}", 
                hoyolab_response.retcode, 
                hoyolab_response.message
            );
            return Ok(Vec::new());
        }

        let mut codes = Vec::new();

        if let Some(data) = hoyolab_response.data {
            if let Some(exchange_group) = data.modules.iter()
                .find(|m| m.exchange_group.is_some())
                .and_then(|m| m.exchange_group.as_ref()) 
            {
                for bonus in &exchange_group.bonuses {
                    if bonus.code_status == "ON" {
                        let rewards: Vec<String> = bonus.icon_bonuses.iter()
                            .map(|icon_bonus| {
                                format!("{} {}", 
                                    icon_bonus.bonus_num,
                                    self.map_reward_type(&icon_bonus.icon_url)
                                )
                            })
                            .collect();

                        if !rewards.is_empty() {
                            codes.push(RedemptionCode {
                                id: None,
                                code: bonus.exchange_code.clone(),
                                rewards,
                                source: "Hoyolab".to_string(),
                                date: Some(DateTime::now()),
                                active: true,
                            });
                        }
                    }
                }
            }
        } else {
            debug!("No active redemption codes available from Hoyolab");
        }

        debug!("Found {} codes from Hoyolab", codes.len());
        Ok(codes)
    }
}

#[cfg(test)]
mod tests; 