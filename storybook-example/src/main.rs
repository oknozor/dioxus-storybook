use dioxus::prelude::*;
use storybook::{Stories, Story, StorybookConfig, storybook};

// Example Button component
#[storybook(tag = "Examples")]
#[component]
pub fn ExampleButton(label: String, #[props(default = false)] disabled: bool) -> Element {
    rsx! {
        button {
            padding: "8px 16px",
            border_radius: "4px",
            border: "1px solid #ccc",
            cursor: "pointer",
            disabled,
            "{label}"
        }
    }
}

impl Stories for ExampleButtonProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new(
                "Default",
                Self {
                    label: "Click me".to_string(),
                    disabled: false,
                },
            ),
            Story::with_description(
                "Disabled",
                "A disabled button that cannot be clicked",
                Self {
                    label: "Can't click".to_string(),
                    disabled: true,
                },
            ),
        ]
    }
}

// Example Card component
#[storybook(tag = "Examples")]
#[component]
pub fn ExampleCard(title: String, content: String) -> Element {
    rsx! {
        div { style: "border: 1px solid #ddd; border-radius: 8px; padding: 16px; max-width: 300px;",
            h3 { style: "margin: 0 0 8px 0;", "{title}" }
            p { style: "margin: 0; color: #666;", "{content}" }
        }
    }
}

impl Stories for ExampleCardProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new("Default", Self {
                title: "Card Title".to_string(),
                content: "This is the card content.".to_string(),
            }),
            Story::with_description(
                "Long Content",
                "A card with longer content text",
                Self {
                    title: "Featured Article".to_string(),
                    content: "This is a much longer content that demonstrates how the card handles more text.".to_string(),
                },
            ),
        ]
    }
}

storybook::storydoc!("Examples", "assets/getting-started.md");

fn main() {
    storybook::launch(StorybookConfig::default().with_title("Example Storybook"));
}
