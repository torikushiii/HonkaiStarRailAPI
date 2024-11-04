use log::{info, error};
use tokio_cron_scheduler::{JobScheduler, Job};
use crate::services::code_service::CodeService;
use crate::services::news_service::NewsService;
use crate::services::db_service::DbService;
use std::sync::Arc;

pub async fn init_scheduler() -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing scheduler");
    let sched = JobScheduler::new().await?;

    // Create a job that runs every minute to fetch new codes
    sched.add(Job::new_async("0 * * * * *", move |_, _| {
        Box::pin(async move {
            info!("Running scheduled code scraping");
            match CodeService::new().await {
                Ok(code_service) => {
                    let code_service = Arc::new(code_service);
                    match code_service.get_all_codes().await {
                        Ok((active, inactive)) => {
                            info!(
                                "Scheduled scraping completed. Found {} active and {} inactive codes",
                                active.len(),
                                inactive.len()
                            );
                        },
                        Err(e) => error!("Failed to fetch codes in scheduled job: {}", e)
                    }
                },
                Err(e) => error!("Failed to initialize code service in scheduled job: {}", e)
            }
        })
    })?).await?;

    // Add a new job that runs every 30 minutes to validate codes
    sched.add(Job::new_async("0 */30 * * * *", move |_, _| {
        Box::pin(async move {
            info!("Running scheduled code validation");
            match CodeService::new().await {
                Ok(code_service) => {
                    match code_service.validate_active_codes().await {
                        Ok(_) => {
                            info!("Scheduled code validation completed successfully");
                        },
                        Err(e) => error!("Failed to validate codes in scheduled job: {}", e)
                    }
                },
                Err(e) => error!("Failed to initialize code service for validation job: {}", e)
            }
        })
    })?).await?;

    // Add a new job that runs every 15 minutes to fetch news
    sched.add(Job::new_async("0 */15 * * * *", move |_, _| {
        Box::pin(async move {
            info!("Running scheduled news fetch");
            match DbService::new().await {
                Ok(db_service) => {
                    match NewsService::new(&db_service).await {
                        Ok(news_service) => {
                            match news_service.fetch_all_news().await {
                                Ok(news) => {
                                    match news_service.save_news(&news).await {
                                        Ok(_) => info!("Successfully updated {} news items", news.len()),
                                        Err(e) => error!("Failed to save news items: {}", e)
                                    }
                                },
                                Err(e) => error!("Failed to fetch news in scheduled job: {}", e)
                            }
                        },
                        Err(e) => error!("Failed to initialize news service in scheduled job: {}", e)
                    }
                },
                Err(e) => error!("Failed to initialize database service in scheduled job: {}", e)
            }
        })
    })?).await?;

    sched.start().await?;

    info!("Scheduler started successfully");

    Ok(())
} 