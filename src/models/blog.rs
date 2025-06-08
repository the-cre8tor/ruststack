use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlogPost {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub published_at: Option<DateTime<Utc>>,
    pub cover_image: Option<String>,
    pub components: Vec<BlogComponent>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlogPostSummary {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub published_at: Option<DateTime<Utc>>,
    pub cover_image: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum BlogComponent {
    #[serde(rename = "heading")]
    Heading { text: String },

    #[serde(rename = "paragraph")]
    Paragraph { markdown: String },

    #[serde(rename = "code")]
    Code { language: String, code: String },

    #[serde(rename = "callout")]
    Callout { style: String, markdown: String },

    #[serde(rename = "card")]
    Card {
        title: String,
        description: String,
        link: String,
    },

    #[serde(rename = "image")]
    Image {
        src: String,
        alt: String,
        caption: Option<String>,
    },

    #[serde(rename = "quote")]
    Quote {
        text: String,
        author: Option<String>,
    },
}
