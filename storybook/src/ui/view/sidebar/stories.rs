use crate::ui::models::{NodeType, Selection};
use crate::ui::view::sidebar::node::ComponentNodeProps;
use crate::ui::view::sidebar::search_input::SearchInputProps;
use crate::ui::view::sidebar::tree::TreeNodeProps;
use crate::{Stories, Story};
use dioxus::prelude::Signal;

impl Stories for SearchInputProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new(
                "Empty",
                Self {
                    search_query: Signal::new(String::new()),
                },
            ),
            Story::new(
                "With Query",
                Self {
                    search_query: Signal::new("Button".to_string()),
                },
            ),
        ]
    }
}

impl Stories for TreeNodeProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new(
                "Category Node",
                Self {
                    name: "Components".to_string(),
                    node: Default::default(),
                    selected: Signal::new(None),
                    node_type: NodeType::Category,
                },
            ),
            Story::new(
                "Folder Node",
                Self {
                    name: "Buttons".to_string(),
                    node: Default::default(),
                    selected: Signal::new(None),
                    node_type: NodeType::Folder,
                },
            ),
        ]
    }
}

impl Stories for ComponentNodeProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new(
                "Collapsed",
                Self {
                    name: "ExampleButton".to_string(),
                    selected: Signal::new(None),
                    stories: vec!["Default".to_string(), "Disabled".to_string()],
                    is_active: false,
                    has_docs: false,
                },
            ),
            Story::new(
                "Expanded",
                Self {
                    name: "ExampleButton".to_string(),
                    selected: Signal::new(Some(Selection::Story("ExampleButton".to_string(), 0))),
                    stories: vec!["Default".to_string(), "Disabled".to_string()],
                    is_active: true,
                    has_docs: false,
                },
            ),
            Story::new(
                "Expanded with Docs",
                Self {
                    name: "ExampleButton".to_string(),
                    selected: Signal::new(Some(Selection::Story("ExampleButton".to_string(), 0))),
                    stories: vec!["Default".to_string(), "Disabled".to_string()],
                    is_active: true,
                    has_docs: true,
                },
            ),
            Story::with_description(
                "Single Story",
                "A component with only one story variant",
                Self {
                    name: "IconButton".to_string(),
                    selected: Signal::new(None),
                    stories: vec!["Default".to_string()],
                    is_active: false,
                    has_docs: false,
                },
            ),
        ]
    }
}
