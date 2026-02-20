use crate::ui::services::iframe::{
    build_css_links, build_grid_css, build_outline_css, build_srcdoc, build_zoom_css,
    capture_inner_html, make_container_id,
};
use crate::ui::viewmodels::ui_settings::UiSettings;
use crate::{StoryInfo, StorybookConfig};
use dioxus::prelude::*;

/// Docking position for the props editor panel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DockPosition {
    /// Panel spans full width at the bottom of the preview area.
    Bottom,
    /// Panel appears as a vertical column on the right side.
    Right,
}

/// Prepared state for a StoryPreview view.
pub struct StoryPreviewState {
    pub container_id: String,
    pub srcdoc: String,
    pub viewport_width: &'static str,
    pub props_json: Signal<String>,
    pub props_visible: Signal<bool>,
    pub props_dock_position: Signal<DockPosition>,
}

/// Custom hook that encapsulates all StoryPreview business logic.
///
/// Handles HTML capture, srcdoc building, context reading, and state management.
/// Returns a `StoryPreviewState` with all data the view needs to render.
pub fn use_story_preview(
    component_name: &str,
    story_index: usize,
    story: &StoryInfo,
) -> StoryPreviewState {
    let mut iframe_html = use_signal(String::new);
    let props_json = use_signal(|| story.props_json.clone());
    let props_visible = use_signal(|| true);
    let props_dock_position = use_signal(|| DockPosition::Bottom);

    let container_id = make_container_id("fullscreen-render", component_name, story_index);
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
    let dark_bg = (ui_settings.dark_preview_background)();

    let css_links = build_css_links(&config);
    let outline_css = build_outline_css(outline_enabled);
    let grid_css = build_grid_css(grid_enabled);
    let zoom_css = build_zoom_css(zoom_level);
    let background_color = if dark_bg { "#1e1e1e" } else { "#ffffff" };
    let srcdoc = build_srcdoc(
        &css_links,
        outline_css,
        grid_css,
        &zoom_css,
        &iframe_html(),
        background_color,
    );

    StoryPreviewState {
        container_id,
        srcdoc,
        viewport_width: viewport_size.to_width(),
        props_json,
        props_visible,
        props_dock_position,
    }
}
