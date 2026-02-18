use crate::ui::UiSettings;
use crate::{find_component, find_doc};
use dioxus::prelude::*;
use crate::ui::doc_page::embedded_story::EmbeddedStory;

mod embedded_story;

const HLJS_VERSION: &str = "11.11.1";
const HLJS_LIGHT_THEME: &str = "github";
const HLJS_DARK_THEME: &str = "github-dark";

/// Component to render a documentation page
#[component]

pub fn DocPage(path: String) -> Element {
    let Some(doc) = find_doc(&path) else {
        return rsx! {
            div { class: "error", "Documentation not found: {path}" }
        };
    };

    rsx! {
        div { class: "doc-page",
            document::Script { src: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/{HLJS_VERSION}/highlight.min.js" }
            DocContent { content_html: doc.content_html.to_string() }
        }
    }
}

/// Component to render documentation content with embedded stories
#[component]
fn DocContent(content_html: String) -> Element {
    let ui_settings = use_context::<UiSettings>();
    let is_dark = (ui_settings.is_dark_theme)();

    // Trigger highlight.js after content renders and when theme changes
    use_effect(move || {
        let theme = if is_dark { HLJS_DARK_THEME } else { HLJS_LIGHT_THEME };
        let css_url = format!(
            "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/{HLJS_VERSION}/styles/{theme}.min.css"
        );
        // Create or update the highlight.js theme stylesheet and re-highlight all code blocks
        document::eval(&format!(
            r#"
            // Create or update the highlight.js th
eme link element
            var link = document.getElementById('hljs-theme');
            if (!link) {{
                link = document.createElement('link');
                link.id = 'hljs-theme';
                link.rel = 'stylesheet';
                document.head.appendChild(link);
            }}
            link.href = "{css_url}";
            // Wait for the DOM to update and script to load, then highlight
            setTimeout(function() {{
                if (typeof hljs !== 'undefined') {{
                    // Remove previous highlighting so hljs re-processes the blocks
                    document.querySelectorAll('pre code[data-highlighted]').forEach(function(el) {{
                        el.removeAttribute('data-highlighted');
                    }});
                    hljs.highlightAll();
                }}
            }}, 100);
            "#
        ));
    });

    // Parse the HTML content and find story embed markers
    // Story embeds are marked as: <div class="storybook-embed" data-story-path="..."></div>

    // Split content by story embed markers and render each part
    let parts = parse_doc_content(&content_html);

    rsx! {
        div { class: "doc-content",
            for (index , part) in parts.iter().enumerate() {
                match part {
                    DocPart::Html(html) => rsx! {
                        div { key: "html-{index}", class: "doc-html", dangerous_inner_html: "{html}" }
                    },
                    DocPart::StoryEmbed { story_path, story_name } => rsx! {
                        EmbeddedStory {
                            key: "story-{index}",
                            story_path: story_path.clone(),
                            story_name: story_name.clone(),
                        }
                    },
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
enum DocPart {
    Html(String),
    StoryEmbed {
        story_path: String,
        story_name: String,
    },
}

/// Parse documentation content and extract story embed markers
fn parse_doc_content(content: &str) -> Vec<DocPart> {
    let mut parts = Vec::new();
    let mut remaining = content;

    while let Some(start_idx) = remaining.find(r#"<div class="storybook-embed""#) {
        // Add HTML before the embed marker
        if start_idx > 0 {
            parts.push(DocPart::Html(remaining[..start_idx].to_string()));
        }

        // Find the end of this div
        if let Some(end_idx) = remaining[start_idx..].find("</div>") {
            let embed_div = &remaining[start_idx..start_idx + end_idx + 6];

            // Extract story path and name from data attributes
            if let (Some(path), Some(name)) = (
                extract_attr(embed_div, "data-story-path"),
                extract_attr(embed_div, "data-story-name"),
            ) {
                parts.push(DocPart::StoryEmbed {
                    story_path: path,
                    story_name: name,
                });
            }

            remaining = &remaining[start_idx + end_idx + 6..];
        } else {
            break;
        }
    }

    // Add any remaining HTML
    if !remaining.is_empty() {
        parts.push(DocPart::Html(remaining.to_string()));
    }

    parts
}

/// Extract an attribute value from an HTML element string
fn extract_attr(element: &str, attr_name: &str) -> Option<String> {
    let pattern = format!(r#"{}=""#, attr_name);
    if let Some(start) = element.find(&pattern) {
        let value_start = start + pattern.len();
        if let Some(end) = element[value_start..].find('"') {
            return Some(element[value_start..value_start + end].to_string());
        }
    }
    None
}

