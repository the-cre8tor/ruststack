# Ruststack

A modern Rust backend application for sharing Rust backend development content with dynamic blog rendering.

## Features

- **Dynamic Blog System**: JSON-based component system for flexible content layouts
- **High Performance**: Built with Actix Web for maximum throughput
- **Type Safety**: SQLx for compile-time checked SQL queries
- **Modern Templates**: MiniJinja templating with Tailwind CSS
- **Component-Based Content**: Reusable components (headings, code blocks, callouts, cards, etc.)

## Architecture

### Backend Stack

- **Framework**: Actix Web
- **Database**: PostgreSQL with SQLx ORM
- **Templates**: MiniJinja with custom filters
- **Styling**: Tailwind CSS + custom CSS

### Blog Content System

Instead of traditional Markdown, this system uses JSON components for maximum flexibility:

```json
{
  "type": "callout",
  "style": "warning",
  "markdown": "Important information with **bold** text"
}
```

Supported component types:

- `heading` - Section headings
- `paragraph` - Text content with markdown support
- `code` - Syntax-highlighted code blocks
- `callout` - Styled information boxes
- `card` - Link cards with titles and descriptions
- `image` - Images with optional captions
- `quote` - Blockquotes with optional attribution

## Project Structure

```
src/
├── main.rs              # Application entry point
├── config.rs            # Configuration management
├── database.rs          # Database connection and queries
├── models/
│   └── blog.rs          # Blog post data models
├── services/
│   └── blog.rs          # Business logic layer
├── handlers/
│   ├── web.rs           # Web page handlers
│   └── api.rs           # API endpoint handlers
└── templates/
    └── mod.rs           # Template engine and filters

templates/
├── base.html            # Base template
├── home.html            # Homepage
├── blog_list.html       # Blog listing page
├── blog_post.html       # Individual blog post
└── contact.html         # Contact page

migrations/
└── 001_create_blog_posts.sql  # Database schema
```

## Setup

1. Install PostgreSQL and create a database
2. Copy `.env.example` to `.env` and configure your database URL
3. Run migrations: `sqlx migrate run`
4. Start the server: `cargo run`

The application will be available at `http://localhost:8080`

## API Endpoints

- `GET /` - Homepage
- `GET /blog` - Blog listing with pagination
- `GET /blog/{slug}` - Individual blog post
- `GET /contact` - Contact page
- `GET /api/blog` - Blog posts API (JSON)
- `GET /api/blog/{slug}` - Single blog post API (JSON)

## Development

The component rendering system is extensible - add new component types by:

1. Adding the variant to `BlogComponent` enum in `models/blog.rs`
2. Implementing rendering logic in `templates/mod.rs`
3. The system automatically handles the new component type

This architecture provides maximum flexibility for creating rich, interactive blog content while maintaining type safety and performance.
