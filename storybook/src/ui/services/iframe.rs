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

/// Build the grid overlay CSS applied to the iframe body when the grid is enabled.
pub fn build_grid_css(enabled: bool) -> &'static str {
    if enabled {
        concat!(
            "body { ",
            "background-size: 100px 100px, 100px 100px, 20px 20px, 20px 20px; ",
            "background-position: 0 0, 0 0, 0 0, 0 0; ",
            "background-blend-mode: difference; ",
            "background-image: ",
            "linear-gradient(rgba(130,130,130,0.5) 1px, transparent 1px), ",
            "linear-gradient(90deg, rgba(130,130,130,0.5) 1px, transparent 1px), ",
            "linear-gradient(rgba(130,130,130,0.25) 1px, transparent 1px), ",
            "linear-gradient(90deg, rgba(130,130,130,0.25) 1px, transparent 1px); ",
            "}"
        )
    } else {
        ""
    }
}

/// Build the CSS zoom rule applied to the iframe body.
pub fn build_zoom_css(zoom_level: i32) -> String {
    if zoom_level == 100 {
        String::new()
    } else {
        let scale = zoom_level as f64 / 100.0;
        format!("body {{ zoom: {scale}; }}")
    }
}

/// Build the full srcdoc HTML for an iframe preview.
pub fn build_srcdoc(
    css_links: &str,
    outline_css: &str,
    grid_css: &str,
    zoom_css: &str,
    body_html: &str,
    background_color: &str,
) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    {css_links}
    <style>
        body {{ margin: 0; padding: 16px; background: {background_color}; }}
        {outline_css}
        {grid_css}
        {zoom_css}
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
