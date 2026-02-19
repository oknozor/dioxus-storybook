use dioxus::prelude::*;
#[cfg(feature = "self-stories")]
use storybook_macro::storybook;
#[cfg(feature = "self-stories")]
use crate::{self as storybook, Stories, Story};
use crate::ui::viewmodels::UiSettings;
use crate::ui::models::Selection;
use crate::ui::view::shared::{FullscreenButton, GridButton, OutlineButton, ThemeToggleButton, ViewPortSelector};
use crate::ui::view::story::StoryZoomControls;

/// Top navigation bar with theme, grid, outline, fullscreen toggles, and story-specific controls
#[cfg_attr(feature = "self-stories", storybook(tag = "Organisms"))]
#[component]
pub(crate) fn TopBar(selected: Signal<Option<Selection>>) -> Element {
    let ui_settings = use_context::<UiSettings>();
    let is_story_selected = matches!(selected(), Some(Selection::Story(_, _)));

    rsx! {
        div { class: "top-bar",
            div { class: "top-bar-left",
                ThemeToggleButton { is_dark_theme: ui_settings.is_dark_theme }
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
        vec![
            Story::new("Default", Self {
                selected: Signal::new(None),
            }),
        ]
    }
}

