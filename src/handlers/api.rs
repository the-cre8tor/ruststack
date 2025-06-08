use crate::services::blog::BlogService;
use actix_web::{web, HttpResponse, Result as ActixResult};
use std::sync::Arc;

pub async fn list_blog_posts(
    blog_service: web::Data<Arc<BlogService>>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> ActixResult<HttpResponse> {
    let page = query.get("page").and_then(|p| p.parse().ok()).unwrap_or(1);

    let per_page = query
        .get("per_page")
        .and_then(|p| p.parse().ok())
        .unwrap_or(10);

    match blog_service.list_posts(page, per_page).await {
        Ok((posts, total)) => {
            let response = serde_json::json!({
                "posts": posts,
                "pagination": {
                    "page": page,
                    "per_page": per_page,
                    "total": total,
                    "total_pages": ((total as f64) / (per_page as f64)).ceil() as i64
                }
            });
            Ok(HttpResponse::Ok().json(response))
        }
        Err(_) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to fetch blog posts"
        }))),
    }
}

pub async fn get_blog_post(
    path: web::Path<String>,
    blog_service: web::Data<Arc<BlogService>>,
) -> ActixResult<HttpResponse> {
    let slug = path.into_inner();

    match blog_service.get_post_by_slug(&slug).await {
        Ok(Some(post)) => Ok(HttpResponse::Ok().json(post)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Post not found"
        }))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to fetch blog post"
        }))),
    }
}
