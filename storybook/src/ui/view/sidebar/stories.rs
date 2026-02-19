use dioxus::prelude::Signal;
use crate::{Stories, Story};
use crate::ui::models::NodeType;
use crate::ui::view::sidebar::search_input::SearchInputProps;
use crate::ui::view::sidebar::tree::TreeNodeProps;

impl Stories for SearchInputProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new("Default", Self {
                search_query: Signal::new("Placeholder".to_string()),
            }),
        ]
    }
}

impl Stories for TreeNodeProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new("Default", Self {
                name: "Component".to_string(),
                node: Default::default(),
                selected: Signal::new(None),
                node_type: NodeType::Category,
            }),
        ]
    }
}
