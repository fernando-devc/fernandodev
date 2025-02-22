use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub content: String,
    pub technologies: Vec<String>,
    pub github_url: Option<String>,
    pub live_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub fn new(
        title: String,
        description: String,
        content: String,
        technologies: Vec<String>,
        github_url: Option<String>,
        live_url: Option<String>,
    ) -> Self {
        let slug = title.to_lowercase()
            .replace(" ", "-")
            .replace(|c: char| !c.is_alphanumeric() && c != '-', "");
            
        Self {
            title,
            slug,
            description,
            content,
            technologies,
            github_url,
            live_url,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
} 