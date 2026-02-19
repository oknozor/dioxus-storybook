use dioxus::prelude::*;
use lucide_dioxus::{Grid3X3, Maximize2, Minimize2, Moon, RotateCcw, Square, Sun, ZoomIn, ZoomOut};

#[cfg(feature = "self-stories")]
use crate::{self as storybook};

#[cfg(feature = "self-stories")]
use storybook_macro::storybook;

/// Toggle button for the grid overlay on the story preview.
///
/// Renders a toolbar button with a `Grid3×3` icon. When active the button
/// receives the `.active` CSS class, providing a visual indicator that the
/// grid overlay is currently visible.
///
/// # Props
///
/// | Prop | Type | Description |
/// |------|------|-------------|
/// | `grid_enabled` | `Signal<bool>` | Reactive flag — `true` shows the grid overlay. |
///
/// @[story:Atoms/GridButton/Enabled]
///
/// @[story:Atoms/GridButton/Disabled]
#[cfg_attr(feature = "self-stories", storybook(tag = "Atoms"))]
#[component]
pub fn GridButton(grid_enabled: Signal<bool>) -> Element {
    rsx! {
        button {
            class: if grid_enabled() { "top-bar-btn active" } else { "top-bar-btn" },
            title: if grid_enabled() { "Hide grid overlay" } else { "Show grid overlay" },
            onclick: move |_| grid_enabled.toggle(),
            Grid3X3 {}
        }
    }
}

/// Toggle button for element outline debugging on the story preview.
///
/// Renders a toolbar button with a `Square` icon. When active, all elements
/// in the story preview receive visible outlines, making it easy to inspect
/// layout and spacing.
///
/// # Props
///
/// | Prop | Type | Description |
/// |------|------|-------------|
/// | `outline_enabled` | `Signal<bool>` | Reactive flag — `true` shows element outlines. |
///
/// @[story:Atoms/OutlineButton/Enabled]
///
/// @[story:Atoms/OutlineButton/Disabled]
#[cfg_attr(feature = "self-stories", storybook(tag = "Atoms"))]
#[component]
pub fn OutlineButton(outline_enabled: Signal<bool>) -> Element {
    rsx! {
        button {
            class: if outline_enabled() { "top-bar-btn active" } else { "top-bar-btn" },
            title: if outline_enabled() { "Hide element outlines" } else { "Show element outlines" },
            onclick: move |_| outline_enabled.toggle(),
            Square {}
        }
    }
}

/// Toggle button for switching the story preview background between light
/// and dark.
///
/// Displays a `Moon` icon when the background is light and a `Sun` icon
/// when it is dark. This does **not** change the application theme — it
/// only affects the preview pane background color so you can test how
/// your component looks on different surfaces.
///
/// # Props
///
/// | Prop | Type | Description |
/// |------|------|-------------|
/// | `dark_preview_background` | `Signal<bool>` | `true` = dark background. |
///
/// @[story:Atoms/ThemeToggleButton/Dark Background]
///
/// @[story:Atoms/ThemeToggleButton/Light Background]
#[cfg_attr(feature = "self-stories", storybook(tag = "Atoms"))]
#[component]
pub fn ThemeToggleButton(dark_preview_background: Signal<bool>) -> Element {
    rsx! {
        button {
            class: if dark_preview_background() { "top-bar-btn active" } else { "top-bar-btn" },
            title: if dark_preview_background() { "Preview: Dark background" } else { "Preview: Light background" },
            onclick: move |_| dark_preview_background.toggle(),
            if dark_preview_background() {
                Sun {}
            } else {
                Moon {}
            }
        }
    }
}

/// Toolbar button that decreases the story preview zoom level.
///
/// Each click reduces `zoom_level` by 25 percentage points, with a minimum
/// of 25 %. Renders a `ZoomOut` icon.
///
/// # Props
///
/// | Prop | Type | Description |
/// |------|------|-------------|
/// | `zoom_level` | `Signal<i32>` | Current zoom percentage (mutated on click). |
///
/// @[story:Atoms/ZoomOutButton/Default]
#[cfg_attr(feature = "self-stories", storybook(tag = "Atoms"))]
#[component]
pub fn ZoomOutButton(zoom_level: Signal<i32>) -> Element {
    rsx! {
        button {
            class: "top-bar-btn",
            title: "Zoom Out",
            onclick: move |_| {
                let current = zoom_level();
                if current > 25 {
                    zoom_level.set(current - 25);
                }
            },
            ZoomOut {}
        }
    }
}

/// Toolbar button that increases the story preview zoom level.
///
/// Each click increases `zoom_level` by 25 percentage points, with a maximum
/// of 200 %. Renders a `ZoomIn` icon.
///
/// # Props
///
/// | Prop | Type | Description |
/// |------|------|-------------|
/// | `zoom_level` | `Signal<i32>` | Current zoom percentage (mutated on click). |
///
/// @[story:Atoms/ZoomInButton/Default]
#[cfg_attr(feature = "self-stories", storybook(tag = "Atoms"))]
#[component]
pub fn ZoomInButton(zoom_level: Signal<i32>) -> Element {
    rsx! {
        button {
            class: "top-bar-btn",
            title: "Zoom In",
            onclick: move |_| {
                let current = (zoom_level)();
                if current < 200 {
                    zoom_level.set(current + 25);
                }
            },
            ZoomIn {}
        }
    }
}

/// Toolbar button that resets the story preview zoom to 100 %.
///
/// Renders a `RotateCcw` icon. Clicking sets `zoom_level` back to `100`
/// regardless of its current value.
///
/// # Props
///
/// | Prop | Type | Description |
/// |------|------|-------------|
/// | `zoom_level` | `Signal<i32>` | Current zoom percentage (reset to 100 on click). |
///
/// @[story:Atoms/ResetZoomButton/Default]
#[cfg_attr(feature = "self-stories", storybook(tag = "Atoms"))]
#[component]
pub fn ResetZoomButton(zoom_level: Signal<i32>) -> Element {
    rsx! {
        button {
            class: "top-bar-btn",
            title: "Reset Zoom",
            onclick: move |_| zoom_level.set(100),
            RotateCcw {}
        }
    }
}

/// Toggle button for hiding or showing the sidebar.
///
/// When fullscreen mode is active the sidebar is hidden and the story
/// preview takes the full width. The icon switches between `Maximize2`
/// (enter fullscreen) and `Minimize2` (exit fullscreen).
///
/// # Props
///
/// | Prop | Type | Description |
/// |------|------|-------------|
/// | `fullscreen_on` | `Signal<bool>` | `true` = sidebar hidden. |
///
/// @[story:Atoms/FullscreenButton/Fullscreen]
///
/// @[story:Atoms/FullscreenButton/Not Fullscreen]
#[cfg_attr(feature = "self-stories", storybook(tag = "Atoms"))]
#[component]
pub fn FullscreenButton(fullscreen_on: Signal<bool>) -> Element {
    rsx! {
        button {
            class: if fullscreen_on() { "top-bar-btn active" } else { "top-bar-btn" },
            title: if fullscreen_on() { "Show sidebar" } else { "Hide sidebar" },
            onclick: move |_| fullscreen_on.toggle(),
            if fullscreen_on() {
                Minimize2 {}
            } else {
                Maximize2 {}
            }
        }
    }
}
