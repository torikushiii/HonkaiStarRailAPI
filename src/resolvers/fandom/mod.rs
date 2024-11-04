use async_trait::async_trait;
use log::{debug, error};
use reqwest::Client;
use scraper::{Html, Selector};
use mongodb::bson::DateTime;
use regex::Regex;

use super::{CodeResolver, RedemptionCode};

pub struct FandomResolver {
    client: Client,
}

impl FandomResolver {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    fn parse_html(&self, html: &str) -> Vec<RedemptionCode> {
        debug!("Parsing Fandom HTML content");
        let document = Html::parse_document(html);
        let mut codes = Vec::new();
        
        let table_selector = Selector::parse("#mw-content-text > div.mw-parser-output > table > tbody").unwrap();
        
        if let Some(table) = document.select(&table_selector).next() {
            let table_text = table.text().collect::<String>();
            let table_list: Vec<&str> = table_text.split('\n')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();

            let code_regex = Regex::new(r"HSRGRANDOPEN[0-9]|[A-Z0-9]{11,15}").unwrap();
            let cleanup_regex = Regex::new(r"All|\[\d+\]|Quick Redeem|CodeServerRewardsDuration").unwrap();
            let amount_regex = Regex::new(r"×\d+").unwrap();
            
            for row in table_list {
                if row.contains("China") {
                    continue;
                }

                let clean_text = cleanup_regex.replace_all(row, "").trim().to_string();
                if let Some(code_match) = code_regex.find(&clean_text) {
                    let code = code_match.as_str().to_string();
                    
                    let rewards_text = clean_text[code_match.end()..]
                        .trim()
                        .split("Discovered")
                        .next()
                        .unwrap_or("")
                        .trim();

                    let reward_list: Vec<&str> = amount_regex.split(rewards_text)
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .collect();

                    let amounts: Vec<&str> = amount_regex.find_iter(rewards_text)
                        .map(|m| m.as_str().trim_start_matches('×'))
                        .collect();

                    let mut rewards = Vec::new();
                    for (i, reward) in reward_list.iter().enumerate() {
                        if i < amounts.len() {
                            rewards.push(format!("{} x{}", reward.trim(), amounts[i]));
                        }
                    }

                    if !rewards.is_empty() {
                        codes.push(RedemptionCode {
                            id: None,
                            code,
                            rewards,
                            source: "star-rail-fandom".to_string(),
                            date: Some(DateTime::now()),
                            active: true,
                        });
                    }
                }
            }
        }

        debug!("Found {} codes from Fandom", codes.len());
        codes
    }
}

#[async_trait]
impl CodeResolver for FandomResolver {
    fn name(&self) -> String {
        "Fandom".to_string()
    }

    fn base_url(&self) -> String {
        "https://honkai-star-rail.fandom.com/wiki/Redemption_Code".to_string()
    }

    async fn fetch_codes(&self) -> Result<Vec<RedemptionCode>, Box<dyn std::error::Error + Send + Sync>> {
        debug!("Fetching codes from Fandom");
        
        let response = self.client
            .get(self.base_url())
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36")
            .send()
            .await?;

        if !response.status().is_success() {
            error!("Failed to fetch Fandom page: {}", response.status());
            return Err("Failed to fetch Fandom page".into());
        }

        let html = response.text().await?;
        Ok(self.parse_html(&html))
    }
}

#[cfg(test)]
mod tests;