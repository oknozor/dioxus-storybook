use crate::find_doc;
use dioxus::prelude::*;
use crate::ui::view::doc_page::embedded_story::EmbeddedStory;
use crate::ui::models::DocPart;
use crate::ui::services::doc_parser::parse_doc_content;
use crate::ui::viewmodels::doc_page_vm::{use_hljs_theme, HLJS_SCRIPT_URL};

mod embedded_story;

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
            document::Script { src: HLJS_SCRIPT_URL }
            DocContent { content_html: doc.content_html.to_string() }
        }
    }
}

/// Component to render documentation content with embedded stories
#[component]
fn DocContent(content_html: String) -> Element {
    // Delegate highlight.js theme management to the viewmodel hook
    use_hljs_theme();

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

