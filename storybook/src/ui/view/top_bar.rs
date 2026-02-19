use crate::ui::models::Selection;
use crate::ui::view::shared::{
    FullscreenButton, GridButton, OutlineButton, ThemeToggleButton, ViewPortSelector,
};
use crate::ui::view::story::StoryZoomControls;
use crate::ui::viewmodels::UiSettings;
#[cfg(feature = "self-stories")]
use crate::{self as storybook, Stories, Story};
use dioxus::prelude::*;
#[cfg(feature = "self-stories")]
use storybook_macro::storybook;

/// Top navigation bar with global and story-specific controls.
///
/// The left section always shows the theme toggle, grid overlay, and
/// outline buttons. When a story is selected, it additionally renders
/// the zoom controls toolbar and the viewport size selector, separated
/// by dividers. The right section contains the fullscreen toggle.
///
/// All UI settings are read from the `UiSettings` context rather than
/// being passed as props, because `UiSettings` contains `Signal` fields
/// that cannot satisfy the serialization requirements of the
/// `#[storybook]` macro.
///
/// # Props
///
/// | Prop | Type | Description |
/// |------|------|-------------|
/// | `selected` | `Signal<Option<Selection>>` | The currently selected sidebar item. |
///
/// @[story:Organisms/TopBar/Default]
#[cfg_attr(feature = "self-stories", storybook(tag = "Organisms"))]
#[component]
pub(crate) fn TopBar(selected: Signal<Option<Selection>>) -> Element {
    let ui_settings = use_context::<UiSettings>();
    let is_story_selected = matches!(selected(), Some(Selection::Story(_, _)));

    rsx! {
        div { class: "top-bar",
            div { class: "top-bar-left",
                ThemeToggleButton { dark_preview_background: ui_settings.dark_preview_background }
                GridButton { grid_enabled: ui_settings.grid_enabled }
                OutlineButton { outline_enabled: ui_settings.outline_enabled }

                if is_story_selected {
                    div { class: "top-bar-divider" }
                    StoryZoomControls { zoom_level: ui_settings.zoom_level }
                    div { class: "top-bar-divider" }
                    ViewPortSelector { viewport_width: ui_settings.viewport_width }
                }
            }

            div { class: "top-bar-right",
                FullscreenButton { fullscreen_on: ui_settings.fullscreen }
            }
        }
    }
}

#[cfg(feature = "self-stories")]
impl Stories for TopBarProps {
    fn stories() -> Vec<Story<Self>> {
        vec![Story::new(
            "Default",
            Self {
                selected: Signal::new(None),
            },
        )]
    }
}
