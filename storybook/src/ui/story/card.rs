use dioxus::prelude::*;
use crate::{RootSchema, StorybookConfig};
use crate::StoryInfo;
use super::props_editor::{PropsEditor, PropsEditorHeader};
use super::iframe::{build_css_links, build_outline_css, build_srcdoc, capture_inner_html, make_container_id};
use crate::ui::story::apply_decorators;
use crate::ui::settings::UiSettings;
use crate::ui::story::toolbar::StoryZoomControls;

/// A single story card that renders one story with its own HTML capture and iframe.
/// Used for embedded story display in documentation pages.
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
    let props_expanded = use_signal(|| false);
    let zoom_level = use_signal(|| 100i32);

    let container_id = make_container_id("preview-render", &component_name, story_index);
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

    let css_links = build_css_links(&config);
    let outline_css = build_outline_css(outline_enabled);
    let srcdoc = build_srcdoc(&css_links, outline_css, &iframe_html());

    let preview_area_class = if grid_enabled {
        "story-preview-area grid-enabled"
    } else {
        "story-preview-area"
    };

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

            div { class: "{preview_area_class}",
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
                PropsEditorHeader { expanded: props_expanded }
                if props_expanded() {
                    PropsEditor { props_json, schema: prop_schema.clone() }
                }
            }
        }
    }
}
