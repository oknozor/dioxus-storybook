use crate::ui::preview::StoryCard;
use crate::{find_component, find_doc};
use dioxus::prelude::*;

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
            // Render the HTML content with embedded stories
            DocContent { content_html: doc.content_html.to_string() }
        }
    }
}

/// Component to render documentation content with embedded stories
#[component]
fn DocContent(content_html: String) -> Element {
    // Parse the HTML content and find story embed markers
    // Story embeds are marked as: <div class="storybook-embed" data-story-path="..."></div>

    // Split content by story embed markers and render each part
    let parts = parse_doc_content(&content_html);

    rsx! {
        div { class: "doc-content",
            for (index, part) in parts.iter().enumerate() {
                match part {
                    DocPart::Html(html) => rsx! {
                        div {
                            key: "html-{index}",
                            class: "doc-html",
                            dangerous_inner_html: "{html}"
                        }
                    },
                    DocPart::StoryEmbed { story_path, story_name } => rsx! {
                        EmbeddedStory {
                            key: "story-{index}",
                            story_path: story_path.clone(),
                            story_name: story_name.clone()
                        }
                    }
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

/// Component to render an embedded story within a doc page
#[component]
fn EmbeddedStory(story_path: String, story_name: String) -> Element {
    // Parse the story path: "Category/Component/StoryName"
    let path_parts: Vec<&str> = story_path.split('/').collect();

    if path_parts.len() < 2 {
        return rsx! {
            div { class: "embedded-story-error", "Invalid story path: {story_path}" }
        };
    }

    // The component name is the second-to-last part, story name is the last
    let component_name = path_parts[path_parts.len() - 2];

    let Some(registration) = find_component(component_name) else {
        return rsx! {
            div { class: "embedded-story-error", "Component not found: {component_name}" }
        };
    };

    // Find the specific story and its index
    let stories = (registration.get_stories)();
    let story_with_index = stories
        .iter()
        .enumerate()
        .find(|(_, s)| s.title == story_name);

    let Some((story_index, story)) = story_with_index else {
        return rsx! {
            div { class: "embedded-story-error", "Story not found: {story_name} in {component_name}" }
        };
    };

    let render_fn = registration.render_with_props;
    let prop_schema = (registration.get_prop_schema)();

    rsx! {
        div { class: "embedded-story",
            StoryCard {
                story: story.clone(),
                component_name: component_name.to_string(),
                story_index,
                render_fn,
                prop_schema
            }
        }
    }
}
