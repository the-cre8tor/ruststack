-- Create blog_posts table
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE blog_posts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    published_at TIMESTAMP
    WITH
        TIME ZONE,
        cover_image VARCHAR(512),
        components JSONB NOT NULL DEFAULT '[]',
        created_at TIMESTAMP
    WITH
        TIME ZONE DEFAULT NOW (),
        updated_at TIMESTAMP
    WITH
        TIME ZONE DEFAULT NOW ()
);

-- Create indexes
CREATE INDEX idx_blog_posts_slug ON blog_posts (slug);

CREATE INDEX idx_blog_posts_published_at ON blog_posts (published_at)
WHERE
    published_at IS NOT NULL;

CREATE INDEX idx_blog_posts_components ON blog_posts USING GIN (components);

-- Create updated_at trigger
-- CREATE OR REPLACE FUNCTION update_updated_at_column()
-- RETURNS TRIGGER AS $
-- BEGIN
--     NEW.updated_at = NOW();
--     RETURN NEW;
-- END;
-- $ language 'plpgsql';
-- CREATE TRIGGER update_blog_posts_updated_at
--     BEFORE UPDATE ON blog_posts
--     FOR EACH ROW
--     EXECUTE FUNCTION update_updated_at_column();
-- Insert sample blog post
INSERT INTO
    blog_posts (
        title,
        slug,
        published_at,
        cover_image,
        components
    )
VALUES
    (
        'Understanding Lifetimes in Rust',
        'understanding-lifetimes',
        '2025-06-06T08:00:00Z',
        '/static/blog/lifetimes.png',
        '[
        {
            "type": "heading",
            "text": "What are Lifetimes?"
        },
        {
            "type": "paragraph",
            "markdown": "Lifetimes in Rust prevent dangling references by enforcing ownership rules at compile time. They ensure that references are valid for as long as they are used."
        },
        {
            "type": "code",
            "language": "rust",
            "code": "fn longest<''a>(x: &''a str, y: &''a str) -> &''a str {\n    if x.len() > y.len() {\n        x\n    } else {\n        y\n    }\n}"
        },
        {
            "type": "callout",
            "style": "warning",
            "markdown": "If you''re new to Rust, borrow checker warnings are common — don''t worry! They''re helping you write **safer code**."
        },
        {
            "type": "card",
            "title": "Further Reading",
            "description": "Check out the official Rust Book''s chapter on lifetimes for more detailed explanations.",
            "link": "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html"
        }
    ]'
    );
