use crate::ui::settings::UiSettings;
use crate::find_doc;
use dioxus::prelude::*;
use crate::ui::doc_page::embedded_story::EmbeddedStory;
use crate::ui::doc_page::parser::{DocPart, parse_doc_content};

mod embedded_story;
pub(crate) mod parser;

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
            // Create or update the highlight.js theme link element
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

