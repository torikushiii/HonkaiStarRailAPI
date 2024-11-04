use log::{info, error, warn};
use crate::resolvers::{CodeResolver, RedemptionCode};
use super::db_service::DbService;
use super::validator_service::{ValidatorService, ValidationResult};
use std::sync::Arc;

pub struct CodeService {
    resolvers: Vec<Arc<dyn CodeResolver>>,
    db_service: DbService,
    validator: ValidatorService,
}

impl CodeService {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        let resolvers: Vec<Arc<dyn CodeResolver>> = vec![
            Arc::new(crate::resolvers::eurogamer::EurogamerResolver::new()),
            Arc::new(crate::resolvers::game8::Game8Resolver::new()),
            Arc::new(crate::resolvers::fandom::FandomResolver::new()),
            Arc::new(crate::resolvers::polygon::PolygonResolver::new()),
            Arc::new(crate::resolvers::prydwen::PrydwenResolver::new()),
            Arc::new(crate::resolvers::hoyolab::HoyolabResolver::new()),
        ];
        
        let db_service = DbService::new().await?;
        let validator = ValidatorService::new();
        
        Ok(Self { resolvers, db_service, validator })
    }

    pub async fn get_all_codes(&self) -> Result<(Vec<RedemptionCode>, Vec<RedemptionCode>), Box<dyn std::error::Error + Send + Sync>> {
        let mut all_codes = Vec::new();
        
        let (existing_active, existing_inactive) = self.db_service.get_codes().await?;
        let existing_codes: std::collections::HashMap<String, bool> = existing_active
            .iter()
            .map(|code| (code.code.clone(), true))
            .chain(existing_inactive.iter().map(|code| (code.code.clone(), false)))
            .collect();
        
        for resolver in &self.resolvers {
            match resolver.fetch_codes().await {
                Ok(codes) => {
                    info!("Successfully retrieved codes from {}", resolver.name());
                    all_codes.extend(codes);
                },
                Err(e) => {
                    error!("Failed to fetch codes from {}: {}", resolver.name(), e);
                }
            }
        }
        
        // Deduplicate codes while preserving existing status
        // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup_by
        all_codes.sort_by(|a, b| a.code.cmp(&b.code));
        all_codes.dedup_by(|a, b| a.code == b.code);
        
        for code in &mut all_codes {
            if !existing_codes.contains_key(&code.code) {
                info!("Found new code: {}. Validating...", code.code);
                match self.validator.validate_code(&code).await {
                    Ok(validation_result) => {
                        match validation_result {
                            ValidationResult::Valid | ValidationResult::AlreadyRedeemed => {
                                info!("New code {} is valid", code.code);
                                code.active = true;
                            },
                            ValidationResult::Expired | ValidationResult::Invalid | ValidationResult::MaxUsageReached => {
                                info!("New code {} is invalid", code.code);
                                code.active = false;
                            },
                            ValidationResult::Cooldown => {
                                warn!("New code {} is in cooldown, marking as active", code.code);
                                code.active = true;
                            },
                            ValidationResult::InvalidCredentials => {
                                error!("Invalid account credentials during validation");
                                return Err("Invalid account credentials".into());
                            },
                            ValidationResult::Unknown(code_num, message) => {
                                warn!("Unknown validation result for code {}: {} - {}", code.code, code_num, message);
                                code.active = true; // Assume active until proven otherwise
                            }
                        }
                        // Add delay between validations to avoid rate limiting
                        tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;
                    },
                    Err(e) => {
                        error!("Failed to validate new code {}: {}", code.code, e);
                        code.active = true; // Assume active if validation fails
                    }
                }
            } else {
                code.active = *existing_codes.get(&code.code).unwrap();
            }
        }
        
        self.db_service.save_codes(&all_codes).await?;
        
        Ok(self.db_service.get_codes().await?)
    }

    pub async fn validate_active_codes(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let validator = ValidatorService::new();
        let (active_codes, _) = self.db_service.get_codes().await?;
        
        for code in active_codes {
            match validator.validate_code(&code).await {
                Ok(validation_result) => {
                    match validation_result {
                        ValidationResult::Valid | ValidationResult::AlreadyRedeemed => {
                            info!("Code {} is still valid", code.code);
                        },
                        ValidationResult::Expired | ValidationResult::Invalid | ValidationResult::MaxUsageReached => {
                            info!("Marking code {} as inactive", code.code);
                            if let Err(e) = self.db_service.update_code_status(&code.code, false).await {
                                error!("Failed to update code status: {}", e);
                            }
                        },
                        ValidationResult::Cooldown => {
                            warn!("Code {} is in cooldown, will check again later", code.code);
                        },
                        ValidationResult::InvalidCredentials => {
                            error!("Invalid account credentials, stopping validation");
                            return Err("Invalid account credentials".into());
                        },
                        ValidationResult::Unknown(code, message) => {
                            error!("Unknown validation result: {} - {}", code, message);
                        }
                    }
                },
                Err(e) => {
                    error!("Failed to validate code {}: {}", code.code, e);
                }
            }
            
            tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
        }
        
        Ok(())
    }
} 