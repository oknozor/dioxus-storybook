use super::props_editor::{PropsEditor, PropsEditorHeader};
use crate::{RenderFn, StoryInfo};
use schemars::Schema;
use crate::ui::services::decorators::apply_decorators;
use crate::ui::view::story::toolbar::StoryZoomControls;
use crate::ui::viewmodels::story_card_vm::use_story_card;
use dioxus::prelude::*;

/// A single story card that renders one story with its own HTML capture and iframe.
/// Used for embedded story display in documentation pages.
#[component]
pub fn StoryCard(
    story: StoryInfo,
    component_name: String,
    story_index: usize,
    render_fn: RenderFn,
    prop_schema: Schema,
    #[props(default)] attribute: Vec<Attribute>,
) -> Element {
    let state = use_story_card(&component_name, story_index, &story);

    rsx! {
        div { class: "story-card",
            h4 { class: "story-card-title", "{story.title}" }

            if let Some(desc) = &story.description {
                p { class: "story-card-description", "{desc}" }
            }

            div {
                id: "{state.container_id}",
                position: "absolute",
                visibility: "hidden",
                pointer_events: "none",
                {apply_decorators((render_fn.0)(&(state.props_json)()), &story.decorators)}
            }

            StoryZoomControls { zoom_level: state.zoom_level }

            div { class: "{state.preview_area_class}",
                iframe {
                    class: "preview-iframe",
                    srcdoc: "{state.srcdoc}",
                    transform: "scale({(state.zoom_level)() as f64 / 100.0})",
                    transform_origin: "top left",
                    width: "{10000.0 / (state.zoom_level)() as f64}%",
                    height: "auto",
                }
            }

            div { class: "props-editor-section",
                PropsEditorHeader { expanded: state.props_expanded }
                if (state.props_expanded)() {
                    PropsEditor {
                        props_json: state.props_json,
                        schema: prop_schema.clone(),
                    }
                }
            }
        }
    }
}
