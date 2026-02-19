use crate::{RootSchema, StorybookConfig};
use dioxus::prelude::*;
use crate::StoryInfo;
use super::props_editor::{PropsEditor, PropsEditorHeader};
use super::iframe::{build_css_links, build_outline_css, build_srcdoc, capture_inner_html, make_container_id};
use crate::ui::story::apply_decorators;
use crate::ui::settings::UiSettings;

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

    let container_id = make_container_id("fullscreen-render", &component_name, story_index);
    let container_id_for_effect = container_id.clone();

    use_effect(move || {
        let _props_json_value = props_json();
        if let Some(html) = capture_inner_html(&container_id_for_effect) {
            iframe_html.set(html);
        }
    });

    let config = use_context::<StorybookConfig>();
    let ui_settings = use_context::<UiSettings>();
    let outline_enabled = (ui_settings.outline_enabled)();
    let grid_enabled = (ui_settings.grid_enabled)();
    let zoom_level = (ui_settings.zoom_level)();
    let viewport_size = (ui_settings.viewport_width)();

    let css_links = build_css_links(&config);
    let outline_css = build_outline_css(outline_enabled);
    let srcdoc = build_srcdoc(&css_links, outline_css, &iframe_html());

    let preview_area_class = if grid_enabled {
        "fullscreen-preview-area grid-enabled"
    } else {
        "fullscreen-preview-area"
    };

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

            div { class: "{preview_area_class}",
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
