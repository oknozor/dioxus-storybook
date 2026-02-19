use crate::ui::view::story::header::StoryHeaderProps;
use crate::ui::view::story::docs::StoryDocsProps;
use crate::ui::view::story::toolbar::StoryZoomControlsProps;
use crate::ui::view::story::props_editor::PropsEditorHeaderProps;
use crate::{Stories, Story};
use dioxus::prelude::Signal;

impl Stories for StoryHeaderProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new(
                "Default",
                Self {
                    component_name: "ExampleButton".to_string(),
                    story_title: "Default".to_string(),
                },
            ),
            Story::new(
                "Long Names",
                Self {
                    component_name: "SuperLongComponentNameForTesting".to_string(),
                    story_title: "With Very Long Story Title Description".to_string(),
                },
            ),
        ]
    }
}

impl Stories for StoryDocsProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new(
                "With Content",
                Self {
                    docs: "<p>This is a <strong>documented</strong> component with rich HTML content.</p>".to_string(),
                },
            ),
            Story::with_description(
                "Empty",
                "Shows the docs area when no documentation is provided",
                Self {
                    docs: String::new(),
                },
            ),
        ]
    }
}

impl Stories for StoryZoomControlsProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new(
                "Default (100%)",
                Self {
                    zoom_level: Signal::new(100),
                },
            ),
            Story::new(
                "Zoomed In (150%)",
                Self {
                    zoom_level: Signal::new(150),
                },
            ),
            Story::new(
                "Zoomed Out (50%)",
                Self {
                    zoom_level: Signal::new(50),
                },
            ),
        ]
    }
}

impl Stories for PropsEditorHeaderProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new(
                "Expanded",
                Self {
                    expanded: Signal::new(true),
                },
            ),
            Story::new(
                "Collapsed",
                Self {
                    expanded: Signal::new(false),
                },
            ),
        ]
    }
}

