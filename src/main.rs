use actix_files::Files;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use tracing_actix_web::TracingLogger;

mod config;
mod database;
mod handlers;
mod models;
mod services;
mod telemetry;
mod templates;

use config::Config;
use database::Database;
use services::blog::BlogService;
use telemetry::Telemetry;
use templates::TemplateEngine;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Telemetry::init_subscriber("Rust Backend", "info".into(), std::io::stdout);

    dotenvy::dotenv().ok();

    let config = Config::from_env().expect("Failed to load configuration");
    let database = Database::new(&config.database_url)
        .await
        .expect("Failed to connect to database");

    let template_engine = Arc::new(TemplateEngine::new().expect("Failed to initialize templates"));
    let blog_service = Arc::new(BlogService::new(database.clone()));

    println!("Starting server at http://{}:{}", config.host, config.port);

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(template_engine.clone()))
            .app_data(web::Data::new(blog_service.clone()))
            .service(
                web::scope("/api")
                    .route("/blog/{slug}", web::get().to(handlers::api::get_blog_post))
                    .route("/blog", web::get().to(handlers::api::list_blog_posts)),
            )
            .route("/", web::get().to(handlers::web::home))
            .route("/blog", web::get().to(handlers::web::blog_list))
            .route("/blog/{slug}", web::get().to(handlers::web::blog_post))
            .route("/contact", web::get().to(handlers::web::contact))
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}
