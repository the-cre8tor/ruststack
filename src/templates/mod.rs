use crate::models::blog::{BlogComponent, BlogPost, BlogPostSummary};
use ammonia::Builder;
use chrono::{DateTime, Utc};
use minijinja::{Environment, Error, Value};
use pulldown_cmark::{html, Options, Parser};

pub struct TemplateEngine {
    env: Environment<'static>,
    markdown_parser: MarkdownParser,
}

pub struct MarkdownParser {
    options: Options,
    sanitizer: ammonia::Builder<'static>,
}

impl MarkdownParser {
    pub fn new() -> Self {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);

        // Configure HTML sanitizer for security
        let mut sanitizer = Builder::default();

        sanitizer
            .add_tags(&[
                "p",
                "br",
                "strong",
                "em",
                "code",
                "pre",
                "blockquote",
                "h1",
                "h2",
                "h3",
                "h4",
                "h5",
                "h6",
                "ul",
                "ol",
                "li",
                "table",
                "thead",
                "tbody",
                "tr",
                "td",
                "th",
                "a",
                "img",
                "del",
                "ins",
                "sup",
                "sub",
            ])
            .add_tag_attributes("a", &["href", "title"])
            .add_tag_attributes("img", &["src", "alt", "title", "width", "height"])
            .add_tag_attributes("code", &["class"])
            .add_tag_attributes("pre", &["class"])
            .add_generic_attributes(&["class", "id"])
            .link_rel(Some("noopener noreferrer"))
            .url_relative(ammonia::UrlRelative::PassThrough);

        Self { options, sanitizer }
    }

    pub fn parse(&self, markdown: &str) -> String {
        let parser = Parser::new_ext(markdown, self.options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        // Sanitize HTML for security
        let sanitized = self.sanitizer.clean(&html_output).to_string();

        // Add custom CSS classes for styling
        self.add_tailwind_classes(&sanitized)
    }

    fn add_tailwind_classes(&self, html: &str) -> String {
        html
            // Style code blocks
            .replace("<code>", "<code class=\"bg-gray-100 px-1 py-0.5 rounded text-sm font-mono\">")
            .replace("<pre><code", "<pre class=\"bg-gray-900 text-gray-100 p-4 rounded-lg overflow-x-auto\"><code")
            // Style links
            .replace("<a ", "<a class=\"text-blue-600 hover:text-blue-800 underline\" ")
            // Style blockquotes
            .replace("<blockquote>", "<blockquote class=\"border-l-4 border-gray-300 pl-4 py-2 italic text-gray-700\">")
            // Style tables
            .replace("<table>", "<table class=\"min-w-full divide-y divide-gray-200 my-4\">")
            .replace("<thead>", "<thead class=\"bg-gray-50\">")
            .replace("<th>", "<th class=\"px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider\">")
            .replace("<td>", "<td class=\"px-6 py-4 whitespace-nowrap text-sm text-gray-900\">")
            // Style lists
            .replace("<ul>", "<ul class=\"list-disc list-inside space-y-1 my-4\">")
            .replace("<ol>", "<ol class=\"list-decimal list-inside space-y-1 my-4\">")
            .replace("<li>", "<li class=\"text-gray-700\">")
    }
}

impl TemplateEngine {
    pub fn new() -> Result<Self, Error> {
        let mut env = Environment::new();

        // Load templates
        env.add_template("base.html", include_str!("./base.html"))?;
        env.add_template("home.html", include_str!("./home.html"))?;
        env.add_template("blog_list.html", include_str!("./blog_list.html"))?;
        env.add_template("blog_post.html", include_str!("./blog_post.html"))?;
        env.add_template("contact.html", include_str!("./contact.html"))?;

        let markdown_parser = MarkdownParser::new();

        // Add custom filters
        env.add_filter("render_component", render_component_filter);
        env.add_filter("markdown_to_html", markdown_to_html_filter);
        env.add_filter("date", date_filter);

        Ok(TemplateEngine {
            env,
            markdown_parser,
        })
    }

    pub fn render_home(&self) -> Result<String, Error> {
        let template = self.env.get_template("home.html")?;
        template.render(minijinja::context! {
            title => "ruststack - Master Rust Backend Development",
            description => "Learn Rust backend development with practical tutorials, courses, and resources."
        })
    }

    pub fn render_blog_list(
        &self,
        posts: &[BlogPostSummary],
        page: usize,
        total_pages: usize,
    ) -> Result<String, Error> {
        let template = self.env.get_template("blog_list.html")?;
        template.render(minijinja::context! {
            title => "Blog - ruststack",
            posts => posts,
            current_page => page,
            total_pages => total_pages,
            has_prev => page > 1,
            has_next => page < total_pages,
            prev_page => if page > 1 { page - 1 } else { 1 },
            next_page => if page < total_pages { page + 1 } else { total_pages }
        })
    }

    pub fn render_blog_post(&self, post: &BlogPost) -> Result<String, Error> {
        let template = self.env.get_template("blog_post.html")?;
        template.render(minijinja::context! {
            title => format!("{} - ruststack", post.title),
            post => post,
        })
    }

    pub fn render_contact(&self) -> Result<String, Error> {
        let template = self.env.get_template("contact.html")?;
        template.render(minijinja::context! {
            title => "Contact - ruststack"
        })
    }

    pub fn _parse_markdown(&self, markdown: &str) -> String {
        self.markdown_parser.parse(markdown)
    }
}

fn render_component_filter(component: minijinja::Value) -> Result<Value, Error> {
    let convert_to = serde_json::to_value(&component).map_err(|e| {
        Error::new(
            minijinja::ErrorKind::InvalidOperation,
            format!("serialization error: {}", e),
        )
    })?;

    let component: BlogComponent = serde_json::from_value(convert_to).map_err(|e| {
        Error::new(
            minijinja::ErrorKind::InvalidOperation,
            format!("deserialization error: {}", e),
        )
    })?;

    let html_content = match component {
        BlogComponent::Heading { text } => format!(
            r#"<h2 class="text-2xl font-bold text-gray-900 mb-4">{}</h2>"#,
            html_escape(&text)
        ),
        BlogComponent::Paragraph { markdown } => {
            // Use the proper markdown parser
            let parser = MarkdownParser::new();
            let html = parser.parse(&markdown);
            format!(
                r#"<div class="prose prose-lg max-w-none mb-6">{}</div>"#,
                html
            )
        }
        BlogComponent::Code { language, code } => format!(
            r#"<div class="bg-gray-900 rounded-lg p-4 mb-6 overflow-x-auto">
                    <pre><code class="language-{} text-gray-100 font-mono text-sm">{}</code></pre>
                   </div>"#,
            html_escape(&language),
            html_escape(&code)
        ),
        BlogComponent::Callout { style, markdown } => {
            let (bg_class, border_class, text_class, icon) = match style.as_str() {
                "warning" => ("bg-yellow-50", "border-yellow-200", "text-yellow-800", "⚠️"),
                "info" => ("bg-blue-50", "border-blue-200", "text-blue-800", "ℹ️"),
                "success" => ("bg-green-50", "border-green-200", "text-green-800", "✅"),
                "error" => ("bg-red-50", "border-red-200", "text-red-800", "❌"),
                _ => ("bg-gray-50", "border-gray-200", "text-gray-800", "📝"),
            };

            let parser = MarkdownParser::new();
            let html = parser.parse(&markdown);
            format!(
                r#"<div class="border-l-4 {} {} {} p-4 mb-6 rounded-r-lg">
                    <div class="flex items-start">
                        <span class="text-lg mr-3 flex-shrink-0">{}</span>
                        <div class="prose prose-sm max-w-none">{}</div>
                    </div>
                   </div>"#,
                border_class, bg_class, text_class, icon, html
            )
        }
        BlogComponent::Card {
            title,
            description,
            link,
        } => format!(
            r#"<div class="bg-white border border-gray-200 rounded-lg p-6 mb-6 hover:shadow-md transition-shadow">
                    <h3 class="text-lg font-semibold text-gray-900 mb-2">{}</h3>
                    <p class="text-gray-600 mb-4">{}</p>
                    <a href="{}" class="inline-flex items-center text-blue-600 hover:text-blue-800 font-medium" target="_blank" rel="noopener noreferrer">
                        Read more →
                    </a>
                   </div>"#,
            html_escape(&title),
            html_escape(&description),
            html_escape(&link)
        ),
        BlogComponent::Image { src, alt, caption } => {
            let caption_html = caption.as_ref()
                .map(|c| format!(r#"<figcaption class="text-center text-gray-600 text-sm mt-2">{}</figcaption>"#, html_escape(c)))
                .unwrap_or_default();

            format!(
                r#"<figure class="mb-6">
                    <img src="{}" alt="{}" class="w-full rounded-lg shadow-sm" loading="lazy">
                    {}
                   </figure>"#,
                html_escape(&src),
                html_escape(&alt),
                caption_html
            )
        }
        BlogComponent::Quote { text, author } => {
            let author_html = author
                .as_ref()
                .map(|a| {
                    format!(
                        r#"<cite class="text-gray-600 text-sm font-medium">— {}</cite>"#,
                        html_escape(a)
                    )
                })
                .unwrap_or_default();

            format!(
                r#"<blockquote class="border-l-4 border-gray-300 pl-6 py-4 mb-6 italic text-gray-700 bg-gray-50 rounded-r-lg">
                    <p class="text-lg mb-2">"{}"</p>
                    {}
                   </blockquote>"#,
                html_escape(&text),
                author_html
            )
        }
    };

    Ok(Value::from_safe_string(html_content))
}

fn markdown_to_html_filter(value: minijinja::Value) -> Result<String, Error> {
    let text = value
        .as_str()
        .ok_or_else(|| Error::new(minijinja::ErrorKind::InvalidOperation, "expected string"))?;
    let parser = MarkdownParser::new();
    Ok(parser.parse(text))
}

fn date_filter(value: minijinja::Value, format: Option<&str>) -> Result<String, Error> {
    let format_str = format.unwrap_or("%Y-%m-%d %H:%M:%S");

    // Try to get as string first (ISO format)
    if let Some(datetime_str) = value.as_str() {
        let datetime: DateTime<Utc> = datetime_str.parse().map_err(|_| {
            Error::new(
                minijinja::ErrorKind::InvalidOperation,
                "invalid datetime format",
            )
        })?;

        return Ok(datetime.format(format_str).to_string());
    }

    // If it's a structured object (from serde), try to extract it
    if let Ok(serde_value) = serde_json::to_value(&value) {
        if let Ok(datetime) = serde_json::from_value::<DateTime<Utc>>(serde_value) {
            return Ok(datetime.format(format_str).to_string());
        }
    }

    Err(Error::new(
        minijinja::ErrorKind::InvalidOperation,
        "value is not a valid datetime",
    ))
}

fn html_escape(text: &str) -> String {
    text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#x27;")
}
