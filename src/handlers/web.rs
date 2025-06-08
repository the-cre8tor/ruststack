use crate::services::blog::BlogService;
use crate::templates::TemplateEngine;
use actix_web::{web, HttpResponse, Result as ActixResult};
use std::sync::Arc;

pub async fn home(template_engine: web::Data<Arc<TemplateEngine>>) -> ActixResult<HttpResponse> {
    match template_engine.render_home() {
        Ok(html) => Ok(HttpResponse::Ok().content_type("text/html").body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Template error")),
    }
}

pub async fn blog_list(
    template_engine: web::Data<Arc<TemplateEngine>>,
    blog_service: web::Data<Arc<BlogService>>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> ActixResult<HttpResponse> {
    let page = query.get("page").and_then(|p| p.parse().ok()).unwrap_or(1);

    let per_page = 10;

    match blog_service.list_posts(page, per_page).await {
        Ok((posts, total)) => {
            let total_pages = ((total as f64) / (per_page as f64)).ceil() as usize;
            match template_engine.render_blog_list(&posts, page, total_pages) {
                Ok(html) => Ok(HttpResponse::Ok().content_type("text/html").body(html)),
                Err(error) => Ok(HttpResponse::InternalServerError().body(error.to_string())),
            }
        }
        Err(_) => Ok(HttpResponse::InternalServerError().body("Database error")),
    }
}

pub async fn blog_post(
    path: web::Path<String>,
    template_engine: web::Data<Arc<TemplateEngine>>,
    blog_service: web::Data<Arc<BlogService>>,
) -> ActixResult<HttpResponse> {
    let slug = path.into_inner();

    match blog_service.get_post_by_slug(&slug).await {
        Ok(Some(post)) => match template_engine.render_blog_post(&post) {
            Ok(html) => Ok(HttpResponse::Ok().content_type("text/html").body(html)),
            Err(_) => Ok(HttpResponse::InternalServerError().body("Template error")),
        },
        Ok(None) => Ok(HttpResponse::NotFound().body("Post not found")),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Database error")),
    }
}

pub async fn contact(template_engine: web::Data<Arc<TemplateEngine>>) -> ActixResult<HttpResponse> {
    match template_engine.render_contact() {
        Ok(html) => Ok(HttpResponse::Ok().content_type("text/html").body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Template error")),
    }
}
