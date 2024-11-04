use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use mongodb::bson::{DateTime, oid::ObjectId};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedemptionCode {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub code: String,
    #[serde(default)]
    pub rewards: Vec<String>,
    #[serde(default)]
    pub source: String,
    #[serde(default = "default_as_true")]
    pub active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<DateTime>,
}

fn default_as_true() -> bool {
    true
}

#[async_trait]
pub trait CodeResolver: Send + Sync {
    fn name(&self) -> String;
    fn base_url(&self) -> String;
    async fn fetch_codes(&self) -> Result<Vec<RedemptionCode>, Box<dyn std::error::Error + Send + Sync>>;
}

pub mod eurogamer;
pub mod game8;
pub mod fandom;
pub mod polygon;
pub mod prydwen;
pub mod hoyolab;
pub mod news;