use crate::RootSchema;
use dioxus::prelude::*;
use crate::StoryInfo;
use super::props_editor::{PropsEditor, PropsEditorHeader};
use crate::ui::services::decorators::apply_decorators;
use crate::ui::viewmodels::story_preview_vm::use_story_preview;

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
    let state = use_story_preview(&component_name, story_index, &story);

    rsx! {
        div { class: "fullscreen-story-view",
            // Hidden render container for HTML capture
            div {
                id: "{state.container_id}",
                position: "absolute",
                visibility: "hidden",
                pointer_events: "none",
                {apply_decorators((render_fn)(&(state.props_json)()), &story.decorators)}
            }

            div { class: "{state.preview_area_class}",
                div {
                    class: "fullscreen-iframe-container",
                    max_width: "{state.viewport_width}",
                    margin: "auto",
                    iframe {
                        class: "preview-iframe",
                        srcdoc: "{state.srcdoc}",
                        transform: "scale({state.zoom_level as f64 / 100.0})",
                        transform_origin: "top left",
                        width: "{10000.0 / state.zoom_level as f64}%",
                        height: "auto",
                    }
                }
            }

            // Fixed bottom props editor
            div { class: "fullscreen-props-panel",
                PropsEditorHeader { expanded: state.props_expanded }
                if (state.props_expanded)() {
                    div { class: "fullscreen-props-scroll",
                        PropsEditor {
                            props_json: state.props_json,
                            schema: prop_schema.clone(),
                        }
                    }
                }
            }
        }
    }
}
