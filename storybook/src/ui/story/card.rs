use dioxus::prelude::*;
use lucide_dioxus::{ChevronDown, ChevronRight};
use crate::{RootSchema, StorybookConfig};
use crate::StoryInfo;
use super::props_editor::PropsEditor;
use crate::ui::story::apply_decorators;
use crate::ui::UiSettings;
use web_sys::window;
use crate::ui::story::toolbar::StoryZoomControls;

/// A single story card that renders one story with its own HTML capture and iframe
#[component]
pub fn StoryCard(
    story: StoryInfo,
    component_name: String,
    story_index: usize,
    render_fn: fn(&str) -> Element,
    prop_schema: RootSchema,
    #[props(default)] attribute: Vec<Attribute>,
) -> Element {
    let mut iframe_html = use_signal(String::new);
    let props_json = use_signal(|| story.props_json.clone());
    let mut props_expanded = use_signal(|| false);
    let zoom_level = use_signal(|| 100i32);

    let container_id = format!(
        "preview-render-{}-story-{}",
        component_name.replace(" ", "-").replace("::", "-"),
        story_index
    );

    let container_id_for_effect = container_id.clone();

    use_effect(move || {
        let _props_json_value = props_json();

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
        div { class: "story-card",
            h4 { class: "story-card-title", "{story.title}" }

            if let Some(desc) = &story.description {
                p { class: "story-card-description", "{desc}" }
            }

            div {
                id: "{container_id}",
                position: "absolute",
                visibility: "hidden",
                pointer_events: "none",
                {apply_decorators(render_fn(&props_json()), &story.decorators)}
            }

            StoryZoomControls { zoom_level }

            div { class: "story-preview-area",
                iframe {
                    class: "preview-iframe",
                    srcdoc: "{srcdoc}",
                    transform: "scale({zoom_level() as f64 / 100.0})",
                    transform_origin: "top left",
                    width: "{10000.0 / zoom_level() as f64}%",
                    height: "auto",
                }
            }

            div { class: "props-editor-section",
                div {
                    class: "props-editor-header",
                    onclick: move |_| props_expanded.toggle(),
                    span { class: "collapse-icon",
                        if props_expanded() {
                            ChevronDown { size: 14, stroke_width: 2 }
                        } else {
                            ChevronRight { size: 14, stroke_width: 2 }
                        }
                    }
                    "Props Editor"
                }
                if props_expanded() {
                    PropsEditor { props_json, schema: prop_schema.clone() }
                }
            }
        }
    }
}
