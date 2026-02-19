use dioxus::prelude::*;
use crate::ui::models::DocPart;
use crate::ui::services::doc_parser::parse_doc_content;
use crate::ui::view::story::StoryCard;
use crate::ui::viewmodels::doc_page_vm::{use_hljs_theme, HLJS_SCRIPT_URL};
use crate::ui::viewmodels::embedded_story_vm::{resolve_embedded_story, EmbeddedStoryError};

/// Component to render a documentation page.
///
/// Pure presentational component — receives resolved `content_html` from the parent.
#[component]
pub fn DocPage(content_html: String) -> Element {
    rsx! {
        div { class: "doc-page",
            document::Script { src: HLJS_SCRIPT_URL }
            DocContent { content_html }
        }
    }
}

/// Component to render documentation content with embedded stories
#[component]
fn DocContent(content_html: String) -> Element {
    use_hljs_theme();

    let parts = parse_doc_content(&content_html);

    rsx! {
        div { class: "doc-content",
            for (index , part) in parts.iter().enumerate() {
                match part {
                    DocPart::Html(html) => rsx! {
                        div { key: "html-{index}", class: "doc-html", dangerous_inner_html: "{html}" }
                    },
                    DocPart::StoryEmbed { story_path, story_name } => {
                        match resolve_embedded_story(story_path, story_name) {
                            Ok(data) => rsx! {
                                div { class: "embedded-story", key: "story-{index}",
                                    StoryCard {
                                        story: data.story,
                                        component_name: data.component_name,
                                        story_index: data.story_index,
                                        render_fn: data.render_fn,
                                        prop_schema: data.prop_schema,
                                    }
                                }
                            },
                            Err(EmbeddedStoryError::InvalidPath(path)) => rsx! {
                                div { key: "story-{index}", class: "embedded-story-error", "Invalid story path: {path}" }
                            },
                            Err(EmbeddedStoryError::ComponentNotFound(name)) => rsx! {
                                div { key: "story-{index}", class: "embedded-story-error", "Component not found: {name}" }
                            },
                            Err(EmbeddedStoryError::StoryNotFound { component_name, story_name }) => {
                                rsx! {
                                    div { key: "story-{index}", class: "embedded-story-error",
                                        "Story not found: {story_name} in {component_name}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

