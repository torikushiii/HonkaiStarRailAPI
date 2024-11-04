use async_trait::async_trait;
use log::{debug, error, info};
use reqwest::Client;
use scraper::{Html, Selector};
use mongodb::bson::DateTime;

use super::{CodeResolver, RedemptionCode};

pub struct Game8Resolver {
    client: Client,
}

impl Game8Resolver {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    fn parse_html(&self, html: &str) -> Vec<RedemptionCode> {
        debug!("Parsing Game8 HTML content");
        let document = Html::parse_document(html);
        let mut codes = Vec::new();
        
        // First find the section with active codes
        let title_selector = Selector::parse("h2").unwrap();
        let list_selector = Selector::parse("ul.a-list").unwrap();
        let item_selector = Selector::parse("li.a-listItem").unwrap();
        let link_selector = Selector::parse("a.a-link").unwrap();

        // Find the correct section by title
        for title in document.select(&title_selector) {
            if title.text().collect::<String>().contains("Active Redeem Codes for") {
                if let Some(code_list) = document.select(&list_selector).next() {
                    for item in code_list.select(&item_selector) {
                        // Get the code from the first a.a-link element
                        if let Some(code_element) = item.select(&link_selector).next() {
                            let code = code_element.text().collect::<String>().trim().to_string();
                            
                            // Get rewards text by removing the code and "NEW" from the full text
                            let full_text = item.text().collect::<String>();
                            let rewards_text = full_text
                                .replace(&code, "")
                                .replace("NEW", "")
                                .trim()
                                .to_string();
                            
                            // Clean up rewards text and split into individual rewards
                            let rewards_text = rewards_text
                                .trim_start_matches('(')
                                .trim_end_matches(')')
                                .to_string();
                            
                            // Process rewards, handling cases where numbers might be split by commas
                            let mut rewards = Vec::new();
                            let mut current_reward = String::new();
                            
                            for part in rewards_text.split(',') {
                                let part = part.trim();
                                if current_reward.chars().last().map_or(false, |c| c.is_ascii_digit()) 
                                   && part.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                                    // If current reward ends with number and next part starts with number,
                                    // treat it as a thousands separator
                                    current_reward.push(',');
                                    current_reward.push_str(part);
                                } else {
                                    if !current_reward.is_empty() {
                                        rewards.push(current_reward.trim().to_string());
                                    }
                                    current_reward = part.to_string();
                                }
                            }
                            if !current_reward.is_empty() {
                                rewards.push(current_reward.trim().to_string());
                            }

                            if !code.is_empty() && !rewards.is_empty() 
                               && code.len() >= 8 
                               && code.chars().all(|c| c.is_ascii_alphanumeric()) {
                                codes.push(RedemptionCode {
                                    id: None,
                                    code,
                                    rewards,
                                    source: "Game8".to_string(),
                                    date: Some(DateTime::now()),
                                    active: true,
                                });
                            }
                        }
                    }
                }
                break; // We found and processed the section we wanted
            }
        }

        info!("Found {} codes from Game8", codes.len());
        codes
    }
}

#[async_trait]
impl CodeResolver for Game8Resolver {
    fn name(&self) -> String {
        "Game8".to_string()
    }

    fn base_url(&self) -> String {
        "https://game8.co/games/Honkai-Star-Rail/archives/410296".to_string()
    }

    async fn fetch_codes(&self) -> Result<Vec<RedemptionCode>, Box<dyn std::error::Error + Send + Sync>> {
        debug!("Fetching codes from Game8");
        
        let response = self.client
            .get(self.base_url())
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .send()
            .await?;

        if !response.status().is_success() {
            error!("Failed to fetch Game8 page: {}", response.status());
            return Err("Failed to fetch Game8 page".into());
        }

        let html = response.text().await?;
        Ok(self.parse_html(&html))
    }
}

#[cfg(test)]
mod tests; 