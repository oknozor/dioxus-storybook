use super::props_editor::PropsEditor;
use crate::ui::services::decorators::apply_decorators;
use crate::ui::viewmodels::story_preview_vm::{DockPosition, use_story_preview};
use crate::{RenderFn, StoryInfo};
use dioxus::prelude::*;
use lucide_dioxus::{PanelBottom, PanelRight, X};
use schemars::Schema;

/// Full-screen story view with dockable props editor and viewport/zoom from UiSettings.
/// Used by StoryPage for the main story display.
#[component]
pub fn StoryPreview(
    story: StoryInfo,
    component_name: String,
    story_index: usize,
    render_fn: RenderFn,
    prop_schema: Schema,
    #[props(default)] attribute: Vec<Attribute>,
) -> Element {
    let state = use_story_preview(&component_name, story_index, &story);
    let mut props_visible = state.props_visible;
    let mut props_dock_position = state.props_dock_position;

    let visible = props_visible();
    let dock = props_dock_position();

    let container_class = match (visible, dock) {
        (true, DockPosition::Bottom) => "fullscreen-story-view dock-bottom",
        (true, DockPosition::Right) => "fullscreen-story-view dock-right",
        _ => "fullscreen-story-view",
    };

    let panel_class = match dock {
        DockPosition::Bottom => "fullscreen-props-panel props-dock-bottom",
        DockPosition::Right => "fullscreen-props-panel props-dock-right",
    };

    rsx! {
        div { class: "{container_class}",
            // Hidden render container for HTML capture
            div {
                id: "{state.container_id}",
                position: "absolute",
                visibility: "hidden",
                pointer_events: "none",
                {apply_decorators((render_fn.0)(&(state.props_json)()), &story.decorators)}
            }

            div { class: "fullscreen-preview-area",
                div {
                    class: "fullscreen-iframe-container",
                    max_width: "{state.viewport_width}",
                    margin: "auto",
                    iframe {
                        class: "preview-iframe",
                        srcdoc: "{state.srcdoc}",
                    }
                }
            }

            if visible {
                div { class: "{panel_class}",
                    // Panel header with dock controls and close button
                    div { class: "props-panel-header",
                        span { class: "props-panel-title", "Props Editor" }
                        div { class: "props-panel-controls",
                            button {
                                class: if dock == DockPosition::Bottom { "props-panel-btn active" } else { "props-panel-btn" },
                                title: "Dock to bottom",
                                onclick: move |_| props_dock_position.set(DockPosition::Bottom),
                                PanelBottom { size: 16, stroke_width: 2 }
                            }
                            button {
                                class: if dock == DockPosition::Right { "props-panel-btn active" } else { "props-panel-btn" },
                                title: "Dock to right",
                                onclick: move |_| props_dock_position.set(DockPosition::Right),
                                PanelRight { size: 16, stroke_width: 2 }
                            }
                            button {
                                class: "props-panel-btn props-panel-close",
                                title: "Close props editor",
                                onclick: move |_| props_visible.set(false),
                                X { size: 16, stroke_width: 2 }
                            }
                        }
                    }
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
