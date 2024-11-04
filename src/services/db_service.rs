use mongodb::{
    Client, Collection, Database,
    options::{ClientOptions, UpdateOptions},
    bson::{doc, DateTime},
};
use log::info;
use futures::TryStreamExt;
use crate::resolvers::RedemptionCode;
use crate::config::Settings;

pub struct DbService {
    db: Database,
    collection: Collection<RedemptionCode>,
}

impl DbService {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        let config = Settings::new().expect("Failed to load configuration");
        let client_options = ClientOptions::parse(&config.mongodb.uri).await?;
        let client = Client::with_options(client_options)?;
        
        let db = client.database(&config.mongodb.database);
        let collection = db.collection("codes");
        
        info!("Connected to MongoDB successfully");
        Ok(Self { db, collection })
    }

    pub async fn save_codes(&self, codes: &[RedemptionCode]) -> Result<(), mongodb::error::Error> {
        for code in codes {
            let now = DateTime::now();
            
            let existing = self.collection
                .find_one(doc! { "code": &code.code })
                .await?;
            
            let update = match existing {
                Some(_) => {
                    doc! {
                        "$set": {
                            "rewards": &code.rewards,
                            "source": &code.source,
                        }
                    }
                },
                None => {
                    doc! {
                        "$set": {
                            "code": &code.code,
                            "rewards": &code.rewards,
                            "source": &code.source,
                            "date": now,
                            "active": code.active,
                        }
                    }
                }
            };

            let _options = UpdateOptions::builder()
                .upsert(true)
                .build();

            self.collection
                .update_one(doc! { "code": &code.code }, update)
                .await?;
        }
        info!("Saved {} codes to database", codes.len());
        Ok(())
    }

    pub async fn get_codes(&self) -> Result<(Vec<RedemptionCode>, Vec<RedemptionCode>), mongodb::error::Error> {
        let mut active = Vec::new();
        let mut inactive = Vec::new();

        let mut cursor = self.collection
            .find(doc! {})
            .await?;

        while let Some(code) = cursor.try_next().await? {
            if code.active {
                active.push(code);
            } else {
                inactive.push(code);
            }
        }

        Ok((active, inactive))
    }

    pub async fn update_code_status(&self, code: &str, active: bool) -> Result<(), mongodb::error::Error> {
        let update = doc! {
            "$set": {
                "active": active
            }
        };

        self.collection
            .update_one(doc! { "code": code }, update)
            .await?;

        info!("Updated code {} status to active={}", code, active);
        Ok(())
    }

    pub fn get_database(&self) -> Database {
        self.db.clone()
    }
} 