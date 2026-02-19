use crate::ui::models::ViewportSize;
use dioxus::prelude::*;

/// Global UI settings shared via context.
///
/// This is the ViewModel for application-wide UI state — it holds reactive
/// signals that views can read and write.

#[derive(Clone, Copy, PartialEq)]
pub struct UiSettings {
    pub is_dark_theme: Signal<bool>,
    pub grid_enabled: Signal<bool>,
    pub outline_enabled: Signal<bool>,
    pub fullscreen: Signal<bool>,
    pub zoom_level: Signal<i32>,
    pub viewport_width: Signal<ViewportSize>,
}

impl Default for UiSettings {
    fn default() -> Self {
        UiSettings {
            is_dark_theme: Signal::new(false),
            grid_enabled: Signal::new(false),
            outline_enabled: Signal::new(false),
            fullscreen: Signal::new(false),
            zoom_level: Signal::new(100),
            viewport_width: Signal::new(ViewportSize::FullWidth),
        }
    }
}

