use crate::ui::models::DocPart;

/// Parse documentation content and extract story embed markers.
///
/// Story embeds are marked as: `<div class="storybook-embed" data-story-path="..." data-story-name="..."></div>`
pub fn parse_doc_content(content: &str) -> Vec<DocPart> {
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

/// Extract an attribute value from an HTML element string.
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
