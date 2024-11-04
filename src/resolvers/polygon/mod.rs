use async_trait::async_trait;
use log::{debug, error, info};
use reqwest::Client;
use scraper::{Html, Selector};
use mongodb::bson::DateTime;
use regex::Regex;

use super::{CodeResolver, RedemptionCode};

pub struct PolygonResolver {
    client: Client,
}

impl PolygonResolver {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    fn parse_html(&self, html: &str) -> Vec<RedemptionCode> {
        debug!("Parsing Polygon HTML content");
        let document = Html::parse_document(html);
        let mut codes = Vec::new();
        
        let list_selector = Selector::parse("ul li").unwrap();
        let code_regex = Regex::new(r"(.*?)\((.*?)\)").unwrap();
        
        for item in document.select(&list_selector) {
            let text = item.text().collect::<String>();
            
            if let Some(captures) = code_regex.captures(&text) {
                if let (Some(code), Some(rewards_text)) = (captures.get(1), captures.get(2)) {
                    let code = code.as_str().trim().to_string();
                    
                    let rewards: Vec<String> = rewards_text
                        .as_str()
                        .replace(" and ", ", ")
                        .split(", ")
                        .map(|s| {
                            let mut parts = s.trim().split(" — ");
                            let reward = parts.next().unwrap_or("").trim().to_string();
                            
                            let mut reward_parts = reward.split(' ');
                            let mut amount = reward_parts.next().unwrap_or("").to_string();
                            
                            // Handle cases where the amount has a comma (e.g., "10,000")
                            if let Some(next_part) = reward_parts.next() {
                                if next_part.parse::<u32>().is_ok() {
                                    amount.push(',');
                                    amount.push_str(next_part);
                                    format!("{} {}", amount, reward_parts.collect::<Vec<_>>().join(" "))
                                } else {
                                    format!("{} {}", amount, std::iter::once(next_part).chain(reward_parts).collect::<Vec<_>>().join(" "))
                                }
                            } else {
                                amount
                            }
                        })
                        .filter(|s| !s.is_empty())
                        .collect();

                    if !code.is_empty() && !rewards.is_empty() {
                        codes.push(RedemptionCode {
                            id: None,
                            code,
                            rewards,
                            source: "Polygon".to_string(),
                            date: Some(DateTime::now()),
                            active: true,
                        });
                    }
                }
            }
        }

        info!("Found {} codes from Polygon", codes.len());
        codes
    }
}

#[async_trait]
impl CodeResolver for PolygonResolver {
    fn name(&self) -> String {
        "Polygon".to_string()
    }

    fn base_url(&self) -> String {
        "https://www.polygon.com/honkai-star-rail-guides/23699079/code-redeem-redemption-gift-stellar-jade".to_string()
    }

    async fn fetch_codes(&self) -> Result<Vec<RedemptionCode>, Box<dyn std::error::Error + Send + Sync>> {
        debug!("Fetching codes from Polygon");
        
        let response = self.client
            .get(self.base_url())
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36")
            .send()
            .await?;

        if !response.status().is_success() {
            error!("Failed to fetch Polygon page: {}", response.status());
            return Err("Failed to fetch Polygon page".into());
        }

        let html = response.text().await?;
        Ok(self.parse_html(&html))
    }
}

#[cfg(test)]
mod tests; 