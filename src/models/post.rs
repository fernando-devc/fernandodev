use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Post {
    pub fn new(title: String, content: String, image_url: Option<String>) -> Self {
        let slug = title.to_lowercase()
            .replace(" ", "-")
            .replace(|c: char| !c.is_alphanumeric() && c != '-', "");
            
        Self {
            title,
            slug,
            content,
            image_url,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
} 