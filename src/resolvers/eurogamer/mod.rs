use async_trait::async_trait;
use log::{debug, error};
use reqwest::Client;
use scraper::{Html, Selector};
use mongodb::bson::DateTime;

use super::{CodeResolver, RedemptionCode};

pub struct EurogamerResolver {
    client: Client,
}

impl EurogamerResolver {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    fn parse_html(&self, html: &str) -> Vec<RedemptionCode> {
        debug!("Parsing Eurogamer HTML content");
        let document = Html::parse_document(html);
        let mut codes = Vec::new();
        
        let list_selector = Selector::parse("#content_above > div.page_content > article > div > div > ul:nth-child(14) > li").unwrap();
        for item in document.select(&list_selector) {
            let text = item.text().collect::<String>().trim().to_string();
            if let Some((code, rewards_str)) = text.split_once(':') {
                let code = code.trim().to_string();
                let rewards: Vec<String> = rewards_str
                    .trim()
                    .split(" and ")
                    .flat_map(|s| {
                        // Split by comma only if not preceded by a digit or followed by a digit
                        s.split(|c| c == ',' && !s.chars().take_while(|&x| x != ',').last().unwrap_or(' ').is_ascii_digit())
                    })
                    .map(|s| s.replace("(new!)", "").trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                if !code.is_empty() {
                    codes.push(RedemptionCode {
                        id: None,
                        code,
                        rewards,
                        source: "Eurogamer".to_string(),
                        date: Some(DateTime::now()),
                        active: true,
                    });
                }
            }
        }

        // Parse table codes
        let table_selector = Selector::parse("table").unwrap();
        if let Some(table) = document.select(&table_selector).next() {
            let mut current_code = String::new();
            let mut current_rewards = Vec::new();
            let mut count = 0;

            for cell in table.text().collect::<String>().split('\n')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .skip(3) // Skip header rows
            {
                match count % 3 {
                    0 => current_code = cell.to_string(),
                    1 => {
                        current_rewards = cell
                            .split(" and ")
                            .flat_map(|s| {
                                // Split by comma only if not preceded by a digit or followed by a digit
                                s.split(|c| c == ',' && !s.chars().take_while(|&x| x != ',').last().unwrap_or(' ').is_ascii_digit())
                            })
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    },
                    2 => {
                        if !current_code.is_empty() {
                            codes.push(RedemptionCode {
                                id: None,
                                code: current_code.clone(),
                                rewards: current_rewards.clone(),
                                source: "Eurogamer".to_string(),
                                date: Some(DateTime::now()),
                                active: true,
                            });
                        }
                    },
                    _ => unreachable!(),
                }
                count += 1;
            }
        }

        debug!("Found {} codes from Eurogamer", codes.len());
        codes
    }
}

#[async_trait]
impl CodeResolver for EurogamerResolver {
    fn name(&self) -> String {
        "Eurogamer".to_string()
    }

    fn base_url(&self) -> String {
        "https://www.eurogamer.net/honkai-star-rail-codes-livestream-active-working-how-to-redeem-9321".to_string()
    }

    async fn fetch_codes(&self) -> Result<Vec<RedemptionCode>, Box<dyn std::error::Error + Send + Sync>> {
        debug!("Fetching codes from Eurogamer");
        
        let response = self.client
            .get(self.base_url())
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36")
            .send()
            .await?;

        if !response.status().is_success() {
            error!("Failed to fetch Eurogamer page: {}", response.status());
            return Err("Failed to fetch Eurogamer page".into());
        }

        let html = response.text().await?;
        Ok(self.parse_html(&html))
    }
}

#[cfg(test)]
mod tests; 