use serde::{Deserialize, Serialize};
use reqwest::Client;
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewsItem {
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "id")]
    #[serde(alias = "external_id")]
    pub external_id: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "createdAt")]
    #[serde(alias = "created_at")]
    pub created_at: i64,
    pub banner: Option<Vec<String>>,
    pub url: String,
    #[serde(rename = "type")]
    pub news_type: String,
    pub lang: String,
}

#[derive(Debug, Deserialize)]
struct HoyolabResponse<T> {
    data: T,
}

#[derive(Debug, Deserialize)]
struct EventList {
    list: Vec<EventItem>,
}

#[derive(Debug, Deserialize)]
struct EventItem {
    id: String,
    name: String,
    desc: String,
    #[serde(deserialize_with = "deserialize_timestamp")]
    create_at: i64,
    banner_url: String,
}

#[derive(Debug, Deserialize)]
struct NewsPost {
    post: Post,
    image_list: Vec<ImageItem>,
}

#[derive(Debug, Deserialize)]
struct Post {
    post_id: String,
    subject: String,
    content: String,
    #[serde(deserialize_with = "deserialize_timestamp")]
    created_at: i64,
}

#[derive(Debug, Deserialize)]
struct ImageItem {
    url: String,
}

#[derive(Debug, Deserialize)]
struct NewsList {
    list: Vec<NewsPost>,
}

pub struct NewsResolver {
    client: Client,
}

fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum TimestampFormat {
        String(String),
        Integer(i64),
    }

    match TimestampFormat::deserialize(deserializer)? {
        TimestampFormat::String(s) => s.parse::<i64>()
            .map_err(|e| Error::custom(format!("Failed to parse string timestamp: {}", e))),
        TimestampFormat::Integer(i) => Ok(i),
    }
}

impl NewsResolver {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch_news(&self, lang: &str) -> Result<Vec<NewsItem>, Box<dyn std::error::Error + Send + Sync>> {
        let events = self.fetch_events(lang).await?;
        let notices = self.fetch_notices(lang).await?;
        let info = self.fetch_info(lang).await?;

        let mut all_news = Vec::new();
        all_news.extend(events);
        all_news.extend(notices);
        all_news.extend(info);

        Ok(all_news)
    }

    async fn fetch_events(&self, lang: &str) -> Result<Vec<NewsItem>, Box<dyn std::error::Error + Send + Sync>> {
        let response = self.client
            .get("https://bbs-api-os.hoyolab.com/community/community_contribution/wapi/event/list")
            .query(&[
                ("page_size", "15"),
                ("size", "15"),
                ("gids", "6"),
            ])
            .header("x-rpc-app_version", "2.42.0")
            .header("x-rpc-client_type", "4")
            .header("x-rpc-language", lang)
            .send()
            .await?;

        let data: HoyolabResponse<EventList> = response.json().await?;
        
        Ok(data.data.list.into_iter().map(|item| {
            let id = item.id.clone();
            NewsItem {
                id: None,
                external_id: item.id,
                title: item.name,
                description: item.desc,
                created_at: item.create_at,
                banner: Some(vec![item.banner_url]),
                url: format!("https://www.hoyolab.com/article/{}", id),
                news_type: "event".to_string(),
                lang: lang.to_string(),
            }
        }).collect())
    }

    async fn fetch_notices(&self, lang: &str) -> Result<Vec<NewsItem>, Box<dyn std::error::Error + Send + Sync>> {
        self.fetch_news_type(lang, 1, "notice").await
    }

    async fn fetch_info(&self, lang: &str) -> Result<Vec<NewsItem>, Box<dyn std::error::Error + Send + Sync>> {
        self.fetch_news_type(lang, 3, "info").await
    }

    async fn fetch_news_type(&self, lang: &str, news_type: i32, type_name: &str) 
        -> Result<Vec<NewsItem>, Box<dyn std::error::Error + Send + Sync>> {
        let response = self.client
            .get("https://bbs-api-os.hoyolab.com/community/post/wapi/getNewsList")
            .query(&[
                ("gids", "6"),
                ("page_size", "15"),
                ("type", &news_type.to_string()),
            ])
            .header("x-rpc-app_version", "2.42.0")
            .header("x-rpc-client_type", "4")
            .header("x-rpc-language", lang)
            .send()
            .await?;

        let data: HoyolabResponse<NewsList> = response.json().await?;
        
        Ok(data.data.list.into_iter().map(|item| {
            let post_id = item.post.post_id.clone();
            NewsItem {
                id: None,
                external_id: item.post.post_id,
                title: item.post.subject,
                description: item.post.content,
                created_at: item.post.created_at,
                banner: if item.image_list.is_empty() {
                    None
                } else {
                    Some(item.image_list.into_iter().map(|img| img.url).collect())
                },
                url: format!("https://www.hoyolab.com/article/{}", post_id),
                news_type: type_name.to_string(),
                lang: lang.to_string(),
            }
        }).collect())
    }
} 