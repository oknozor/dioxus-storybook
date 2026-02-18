use dioxus::prelude::*;
use lucide_dioxus::{Grid3X3, Maximize2, Minimize2, Moon, RotateCcw, Square, Sun, ZoomIn, ZoomOut};
use storybook_macro::storybook;
use crate::{self as storybook, Stories, Story};

#[storybook(tag = "Atoms")]
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

impl Stories for GridButtonProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new("Enabled", Self {
                grid_enabled: Signal::new(true),
            }),
            Story::new("Disabled", Self {
                grid_enabled: Signal::new(false),
            }),
        ]
    }
}

#[storybook(tag = "Atoms")]
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

impl Stories for OutlineButtonProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new("Enabled", Self {
                outline_enabled: Signal::new(true),
            }),
            Story::new("Disabled", Self {
                outline_enabled: Signal::new(false),
            }),
        ]
    }
}

#[storybook(tag = "Atoms")]
#[component]
pub fn ThemeToggleButton(is_dark_theme: Signal<bool>) -> Element {
    rsx! {
        button {
            class: if is_dark_theme() { "top-bar-btn active" } else { "top-bar-btn" },
            title: if is_dark_theme() { "Switch to light theme" } else { "Switch to dark theme" },
            onclick: move |_| is_dark_theme.toggle(),
            if is_dark_theme() {
                Sun {}
            } else {
                Moon {}
            }
        }
    }
}

impl Stories for ThemeToggleButtonProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new("Dark", Self {
                is_dark_theme: Signal::new(true),
            }),
            Story::new("Light", Self {
                is_dark_theme: Signal::new(false),
            }),
        ]
    }
}

#[storybook(tag = "Atoms")]
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

impl Stories for ZoomOutButtonProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new("Default", Self {
                zoom_level: Signal::new(100),
            }),
        ]
    }
}

#[storybook(tag = "Atoms")]
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

impl Stories for ZoomInButtonProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new("Default", Self {
                zoom_level: Signal::new(100),
            }),
        ]
    }
}

#[storybook(tag = "Atoms")]
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

impl Stories for ResetZoomButtonProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new("Default", Self {
                zoom_level: Signal::new(100),
            }),
        ]
    }
}

#[storybook(tag = "Atoms")]
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

impl Stories for FullscreenButtonProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new("Fullscreen", Self {
                fullscreen_on: Signal::new(true),
            }),
            Story::new("Not Fullscreen", Self {
                fullscreen_on: Signal::new(false),
            }),
        ]
    }
}
