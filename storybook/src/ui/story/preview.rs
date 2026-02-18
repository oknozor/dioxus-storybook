use crate::{RootSchema, StorybookConfig};
use dioxus::prelude::*;
use crate::StoryInfo;
use super::props_editor::{PropsEditor, PropsEditorHeader};
use crate::ui::story::apply_decorators;
use crate::ui::UiSettings;

/// Full-screen story view with fixed bottom props editor and viewport/zoom from UiSettings.
/// Used by StoryPage for the main story display.
#[component]
pub fn StoryPreview(
    story: StoryInfo,
    component_name: String,
    story_index: usize,
    render_fn: fn(&str) -> Element,
    prop_schema: RootSchema,
    #[props(default)] attribute: Vec<Attribute>,
) -> Element {
    let mut iframe_html = use_signal(String::new);
    let props_json = use_signal(|| story.props_json.clone());
    let props_expanded = use_signal(|| true);

    let container_id = format!(
        "fullscreen-render-{}-story-{}",
        component_name.replace(" ", "-").replace("::", "-"),
        story_index
    );

    let container_id_for_effect = container_id.clone();

    use_effect(move || {
        let _props_json_value = props_json();

        use web_sys::window;
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(container) = document.get_element_by_id(&container_id_for_effect) {
                    let html = container.inner_html();
                    iframe_html.set(html);
                }
            }
        }
    });

    let config = use_context::<StorybookConfig>();
    let ui_settings = use_context::<UiSettings>();
    let outline_enabled = (ui_settings.outline_enabled)();
    let grid_enabled = (ui_settings.grid_enabled)();
    let zoom_level = (ui_settings.zoom_level)();
    let viewport_size = (ui_settings.viewport_width)();

    let css_links = config
        .component_css
        .iter()
        .map(|css| format!(r#"<link rel="stylesheet" href="{}">"#, css))
        .collect::<Vec<_>>()
        .join("\n    ");

    let outline_css = if outline_enabled {
        "* { outline: 1px solid rgba(255, 0, 0, 0.3) !important; }"
    } else {
        ""
    };

    let grid_css = if grid_enabled {
        "body { background-size: 10px 10px; background-image: linear-gradient(to right, rgba(0,0,0,0.05) 1px, transparent 1px), linear-gradient(to bottom, rgba(0,0,0,0.05) 1px, transparent 1px); }"
    } else {
        ""
    };

    let srcdoc = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    {css_links}
    <style>
        body {{ margin: 0; padding: 16px; }}
        {outline_css}
        {grid_css}
    </style>
</head>
<body>
    {}
</body>
</html>"#,
        iframe_html()
    );

    rsx! {
        div { class: "fullscreen-story-view",
            // Hidden render container for HTML capture
            div {
                id: "{container_id}",
                position: "absolute",
                visibility: "hidden",
                pointer_events: "none",
                {apply_decorators((render_fn)(&props_json()), &story.decorators)}
            }

            div { class: "fullscreen-preview-area",
                div {
                    class: "fullscreen-iframe-container",
                    max_width: "{viewport_size.to_width()}",
                    margin: "auto",
                    iframe {
                        class: "preview-iframe",
                        srcdoc: "{srcdoc}",
                        transform: "scale({zoom_level as f64 / 100.0})",
                        transform_origin: "top left",
                        width: "{10000.0 / zoom_level as f64}%",
                        height: "auto",
                    }
                }
            }

            // Fixed bottom props editor
            div { class: "fullscreen-props-panel",
                PropsEditorHeader { expanded: props_expanded }
                if props_expanded() {
                    div { class: "fullscreen-props-scroll",
                        PropsEditor { props_json, schema: prop_schema.clone() }
                    }
                }
            }
        }
    }
}
