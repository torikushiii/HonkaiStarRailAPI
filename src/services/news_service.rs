use mongodb::{Collection, bson::{doc, Document}};
use crate::resolvers::news::{NewsResolver, NewsItem};
use log::{info, error, debug};
use futures::TryStreamExt;

pub struct NewsService {
    collection: Collection<NewsItem>,
    resolver: NewsResolver,
}

impl NewsService {
    pub async fn new(db_service: &super::db_service::DbService) -> Result<Self, mongodb::error::Error> {
        let collection = db_service.get_database().collection("news");
        let resolver = NewsResolver::new();
        
        Ok(Self { collection, resolver })
    }

    pub async fn fetch_all_news(&self) -> Result<Vec<NewsItem>, Box<dyn std::error::Error + Send + Sync>> {
        let supported_languages = vec![
            "en-us", "zh-cn", "zh-tw", "de-de", "es-es", "fr-fr", "id-id",
            "it-it", "ja-jp", "ko-kr", "pt-pt", "ru-ru", "th-th", "tr-tr", "vi-vn"
        ];

        let mut all_news = Vec::new();

        for lang in supported_languages {
            match self.resolver.fetch_news(lang).await {
                Ok(news) => {
                    debug!("Successfully fetched {} news items for language {}", news.len(), lang);
                    all_news.extend(news);
                },
                Err(e) => {
                    error!("Failed to fetch news for language {}: {}", lang, e);
                }
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
            filter.insert("lang", lang_str);
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