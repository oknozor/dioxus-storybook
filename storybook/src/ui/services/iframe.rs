use crate::StorybookConfig;

/// Build the CSS `<link>` tags for component stylesheets.
pub fn build_css_links(config: &StorybookConfig) -> String {
    config
        .component_css
        .iter()
        .map(|css| format!(r#"<link rel="stylesheet" href="{}">"#, css))
        .collect::<Vec<_>>()
        .join("\n    ")
}

/// Build the outline CSS rule if outlines are enabled.
pub fn build_outline_css(enabled: bool) -> &'static str {
    if enabled {
        "* { outline: 1px solid rgba(255, 0, 0, 0.3) !important; }"
    } else {
        ""
    }
}

/// Build the full srcdoc HTML for an iframe preview.
pub fn build_srcdoc(css_links: &str, outline_css: &str, body_html: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    {css_links}
    <style>
        body {{ margin: 0; padding: 16px; }}
        {outline_css}
    </style>
</head>
<body>
    {body_html}
</body>
</html>"#
    )
}

/// Capture the innerHTML from a hidden render container via web_sys.
pub fn capture_inner_html(container_id: &str) -> Option<String> {
    use web_sys::window;
    let window = window()?;
    let document = window.document()?;
    let container = document.get_element_by_id(container_id)?;
    Some(container.inner_html())
}

/// Generate a unique container ID for HTML capture.
pub fn make_container_id(prefix: &str, component_name: &str, story_index: usize) -> String {
    format!(
        "{}-{}-story-{}",
        prefix,
        component_name.replace(" ", "-").replace("::", "-"),
        story_index
    )
}

