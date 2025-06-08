use crate::models::blog::{BlogPost, BlogPostSummary};
use anyhow::Result;
use sqlx::{PgPool, Row};

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPool::connect(database_url).await?;

        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Database { pool })
    }

    pub async fn get_blog_post_by_slug(&self, slug: &str) -> Result<Option<BlogPost>> {
        let row = sqlx::query(
            "SELECT id, title, slug, published_at, cover_image, components, created_at, updated_at
             FROM blog_posts
             WHERE slug = $1 AND published_at IS NOT NULL",
        )
        .bind(slug)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let components_json: serde_json::Value = row.get("components");
            let post = BlogPost {
                id: row.get("id"),
                title: row.get("title"),
                slug: row.get("slug"),
                published_at: row.get("published_at"),
                cover_image: row.get("cover_image"),
                components: serde_json::from_value(components_json)?,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            Ok(Some(post))
        } else {
            Ok(None)
        }
    }

    pub async fn list_blog_posts(&self, limit: i64, offset: i64) -> Result<Vec<BlogPostSummary>> {
        let rows = sqlx::query(
            "SELECT id, title, slug, published_at, cover_image, created_at, updated_at
             FROM blog_posts
             WHERE published_at IS NOT NULL
             ORDER BY published_at DESC
             LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let posts = rows
            .into_iter()
            .map(|row| BlogPostSummary {
                id: row.get("id"),
                title: row.get("title"),
                slug: row.get("slug"),
                published_at: row.get("published_at"),
                cover_image: row.get("cover_image"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(posts)
    }

    pub async fn count_blog_posts(&self) -> Result<i64> {
        let row =
            sqlx::query("SELECT COUNT(*) as count FROM blog_posts WHERE published_at IS NOT NULL")
                .fetch_one(&self.pool)
                .await?;

        Ok(row.get("count"))
    }
}
