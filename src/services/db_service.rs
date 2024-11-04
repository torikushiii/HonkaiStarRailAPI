use mongodb::{
    Client, Database,
    options::{ClientOptions, ServerApi, ServerApiVersion, UpdateOptions},
    bson::{doc, DateTime},
};
use std::sync::Arc;
use tokio::sync::OnceCell;
use log::{info, debug, error};
use futures::TryStreamExt;
use crate::resolvers::RedemptionCode;
use crate::config::Settings;

static DB_INSTANCE: OnceCell<Arc<DbService>> = OnceCell::const_new();

pub struct DbService {
    db: Database,
}

impl DbService {
    pub async fn instance() -> Arc<DbService> {
        DB_INSTANCE.get_or_init(|| async {
            match Self::init().await {
                Ok(service) => Arc::new(service),
                Err(e) => {
                    error!("Failed to initialize database service: {}", e);
                    panic!("Database initialization failed");
                }
            }
        }).await.clone()
    }

    async fn init() -> Result<Self, mongodb::error::Error> {
        let config = Settings::new().expect("Failed to load configuration");
        
        let mut client_options = ClientOptions::parse(&config.mongodb.uri).await?;
        
        client_options.max_pool_size = Some(10);
        client_options.min_pool_size = Some(2);
        client_options.connect_timeout = Some(std::time::Duration::from_secs(5));
        client_options.max_idle_time = Some(std::time::Duration::from_secs(60));
        
        let server_api = ServerApi::builder()
            .version(ServerApiVersion::V1)
            .build();
        client_options.server_api = Some(server_api);

        client_options.compressors = Some(vec![
            mongodb::options::Compressor::Snappy,
            mongodb::options::Compressor::Zlib { level: Some(6) },
            mongodb::options::Compressor::Zstd { level: Some(3) },
        ]);

        let client = Client::with_options(client_options)?;
        let db = client.database(&config.mongodb.database);
        
        db.run_command(doc! {"ping": 1}).await?;
        
        info!("Connected to MongoDB successfully");
        Ok(Self { db })
    }

    pub async fn save_codes(&self, codes: &[RedemptionCode]) -> Result<(), mongodb::error::Error> {
        let collection = self.db.collection::<RedemptionCode>("codes");
        
        for code in codes {
            let now = DateTime::now();
            
            let existing = collection
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

            let options = UpdateOptions::builder()
                .upsert(true)
                .build();

            collection
                .update_one(doc! { "code": &code.code }, update)
                .with_options(options)
                .await?;
        }
        debug!("Saved {} codes to database", codes.len());
        Ok(())
    }

    pub async fn get_codes(&self) -> Result<(Vec<RedemptionCode>, Vec<RedemptionCode>), mongodb::error::Error> {
        let collection = self.db.collection::<RedemptionCode>("codes");
        let mut active = Vec::new();
        let mut inactive = Vec::new();

        let mut cursor = collection
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
        let collection = self.db.collection::<RedemptionCode>("codes");
        let update = doc! {
            "$set": {
                "active": active
            }
        };

        collection
            .update_one(doc! { "code": code }, update)
            .await?;

        info!("Updated code {} status to active={}", code, active);
        Ok(())
    }

    pub fn get_database(&self) -> Database {
        self.db.clone()
    }
} 