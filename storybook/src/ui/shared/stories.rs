use crate::ui::shared::{FullscreenButtonProps, GridButtonProps, OutlineButtonProps, ResetZoomButtonProps, ThemeToggleButtonProps, ZoomInButtonProps, ZoomOutButtonProps};
use crate::{Stories, Story};
use dioxus::prelude::*;

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

impl Stories for ZoomOutButtonProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new("Default", Self {
                zoom_level: Signal::new(100),
            }),
        ]
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

impl Stories for ResetZoomButtonProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new("Default", Self {
                zoom_level: Signal::new(100),
            }),
        ]
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



