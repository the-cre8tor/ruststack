use crate::database::Database;
use crate::models::blog::{BlogPost, BlogPostSummary};
use anyhow::Result;

pub struct BlogService {
    db: Database,
}

impl BlogService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn get_post_by_slug(&self, slug: &str) -> Result<Option<BlogPost>> {
        self.db.get_blog_post_by_slug(slug).await
    }

    pub async fn list_posts(
        &self,
        page: usize,
        per_page: usize,
    ) -> Result<(Vec<BlogPostSummary>, i64)> {
        let offset = (page.saturating_sub(1)) * per_page;
        let posts = self
            .db
            .list_blog_posts(per_page as i64, offset as i64)
            .await?;

        let total = self.db.count_blog_posts().await?;
        Ok((posts, total))
    }
}
