use async_trait::async_trait;
use log::{debug, error};
use reqwest::Client;
use scraper::{Html, Selector};
use mongodb::bson::DateTime;

use super::{CodeResolver, RedemptionCode};

pub struct PrydwenResolver {
    client: Client,
}

impl PrydwenResolver {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    fn parse_html(&self, html: &str) -> Vec<RedemptionCode> {
        debug!("Parsing Prydwen HTML content");
        let document = Html::parse_document(html);
        let mut codes = Vec::new();
        
        // Select the codes container and boxes
        let codes_selector = Selector::parse(".codes .box").unwrap();
        let code_selector = Selector::parse(".code").unwrap();
        let rewards_selector = Selector::parse(".rewards").unwrap();
        
        for code_box in document.select(&codes_selector) {
            // Extract code
            if let Some(code_element) = code_box.select(&code_selector).next() {
                let code = code_element.text()
                    .collect::<String>()
                    .replace(" NEW!", "")
                    .trim()
                    .to_string();
                
                // Extract rewards
                if let Some(rewards_element) = code_box.select(&rewards_selector).next() {
                    let rewards: Vec<String> = rewards_element.text()
                        .collect::<String>()
                        .split(" + ")
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();

                    if !code.is_empty() && !rewards.is_empty() {
                        codes.push(RedemptionCode {
                            id: None,
                            code,
                            rewards,
                            source: "Prydwen".to_string(),
                            date: Some(DateTime::now()),
                            active: true,
                        });
                    }
                }
            }
        }

        debug!("Found {} codes from Prydwen", codes.len());
        codes
    }
}

#[async_trait]
impl CodeResolver for PrydwenResolver {
    fn name(&self) -> String {
        "Prydwen".to_string()
    }

    fn base_url(&self) -> String {
        "https://www.prydwen.gg/star-rail/".to_string()
    }

    async fn fetch_codes(&self) -> Result<Vec<RedemptionCode>, Box<dyn std::error::Error + Send + Sync>> {
        debug!("Fetching codes from Prydwen");
        
        let response = self.client
            .get(self.base_url())
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36")
            .send()
            .await?;

        if !response.status().is_success() {
            error!("Failed to fetch Prydwen page: {}", response.status());
            return Err("Failed to fetch Prydwen page".into());
        }

        let html = response.text().await?;
        Ok(self.parse_html(&html))
    }
}

#[cfg(test)]
mod tests; 