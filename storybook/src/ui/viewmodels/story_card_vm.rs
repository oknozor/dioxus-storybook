use crate::ui::services::iframe::{
    build_css_links, build_outline_css, build_srcdoc, capture_inner_html, make_container_id,
};
use crate::ui::viewmodels::ui_settings::UiSettings;
use crate::{StoryInfo, StorybookConfig};
use dioxus::prelude::*;

/// Prepared state for a StoryCard view.
pub struct StoryCardState {
    pub container_id: String,
    pub srcdoc: String,
    pub preview_area_class: &'static str,
    pub zoom_level: Signal<i32>,
    pub props_json: Signal<String>,
    pub props_expanded: Signal<bool>,
}

/// Custom hook that encapsulates all StoryCard business logic.
///
/// Handles HTML capture, srcdoc building, context reading, and state management.
/// Returns a `StoryCardState` with all data the view needs to render.
pub fn use_story_card(
    component_name: &str,
    story_index: usize,
    story: &StoryInfo,
) -> StoryCardState {
    let mut iframe_html = use_signal(String::new);
    let props_json = use_signal(|| story.props_json.clone());
    let props_expanded = use_signal(|| false);
    let zoom_level = use_signal(|| 100i32);

    let container_id = make_container_id("preview-render", component_name, story_index);
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

    StoryCardState {
        container_id,
        srcdoc,
        preview_area_class,
        zoom_level,
        props_json,
        props_expanded,
    }
}
