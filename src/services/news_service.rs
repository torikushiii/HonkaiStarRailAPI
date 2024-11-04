use mongodb::{Collection, bson::{doc, Document}};
use crate::resolvers::news::{NewsResolver, NewsItem};
use log::{info, error, debug};
use futures::TryStreamExt;
use futures::future::join_all;
use crate::utils::lang_parser::{SUPPORTED_LANGUAGES, is_supported_language, parse_language_code};

pub struct NewsService {
    collection: Collection<NewsItem>,
    resolver: NewsResolver,
}

impl NewsService {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        let db_service = super::db_service::DbService::instance().await;
        let collection = db_service.get_database().collection("news");
        let resolver = NewsResolver::new();
        
        Ok(Self { collection, resolver })
    }

    pub async fn fetch_all_news(&self) -> Result<Vec<NewsItem>, Box<dyn std::error::Error + Send + Sync>> {
        let futures: Vec<_> = SUPPORTED_LANGUAGES
            .iter()
            .map(|&lang| self.resolver.fetch_news(lang))
            .collect();

        let results = join_all(futures).await;
        
        let mut all_news = Vec::new();
        for result in results {
            match result {
                Ok(news) => all_news.extend(news),
                Err(e) => error!("Failed to fetch news: {}", e),
            }
        }

        Ok(all_news)
    }

    pub async fn save_news(&self, news: &[NewsItem]) -> Result<(), mongodb::error::Error> {
        debug!("Starting to save news items to database");
        
        for item in news {
            let filter = doc! {
                "id": &item.external_id,
                "lang": &item.lang
            };

            let update = doc! {
                "$set": {
                    "id": &item.external_id,
                    "title": &item.title,
                    "description": &item.description,
                    "createdAt": &item.created_at,
                    "banner": &item.banner,
                    "url": &item.url,
                    "type": &item.news_type,
                    "lang": &item.lang,
                }
            };

            let update_result = self.collection
                .update_one(filter, update)
                .upsert(true)
                .await?;

            if update_result.upserted_id.is_some() {
                info!("New news item added: {} ({})", item.title, item.lang);
            }
        }

        debug!("Successfully saved news items to database");
        Ok(())
    }

    pub async fn get_news(&self, news_type: Option<&str>, lang: Option<&str>) 
        -> Result<Vec<NewsItem>, mongodb::error::Error> {
        let mut filter = Document::new();
        
        if let Some(type_str) = news_type {
            filter.insert("type", type_str);
        }
        
        if let Some(lang_str) = lang {
            if !is_supported_language(lang_str) {
                debug!("Unsupported language code: {}, defaulting to en-us", lang_str);
                filter.insert("lang", "en-us");
            } else {
                filter.insert("lang", parse_language_code(lang_str));
            }
        }

        let mut cursor = self.collection
            .find(filter)
            .sort(doc! { "created_at": -1 })
            .await?;

        let mut news = Vec::new();
        
        while let Some(item) = cursor.try_next().await? {
            news.push(item);
        }

        Ok(news)
    }
} 