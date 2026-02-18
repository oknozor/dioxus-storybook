use dioxus::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::ui::sidebar::node::ComponentNode;
use crate::ui::sidebar::tree::{build_category_tree, TreeNode};
use crate::ui::sidebar::search_input::SearchInput;

mod search_input;
mod tree;
mod node;

#[derive(Clone, PartialEq, Debug)]
pub struct ComponentInfo {
    pub name: String,
    pub category: String,
}

/// Selection type - a story, component, or doc page
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, JsonSchema)]
pub enum Selection {
    /// A specific story within a component (component_name, story_index)
    Story(String, usize),
    /// A documentation page
    DocPage(String),
}

/// The type of node in the hierarchy tree
#[derive(Clone, Copy, PartialEq, Debug)]
enum NodeType {
    /// Top-level category (first segment of the path)
    Category,
    /// Intermediate folder (middle segments of the path)
    Folder,
}

#[component]
pub fn Sidebar(search_query: Signal<String>, components: Vec<ComponentInfo>, selected: Signal<Option<Selection>>) -> Element {
    rsx! {
        div { class: "sidebar",
            SearchInput { search_query }
            ComponentTree { components, selected }
        }
    }
}

#[component]
pub fn ComponentTree(
    components: Vec<ComponentInfo>,
    selected: Signal<Option<Selection>>,
) -> Element {
    let tree = build_category_tree(&components);

    rsx! {
        div { class: "component-tree",
            // Render top-level categories (depth 0 = Category)
            for (category_name , node) in tree.children.iter() {
                TreeNode {
                    key: "{category_name}",
                    name: category_name.clone(),
                    node: node.clone(),
                    selected,
                    node_type: NodeType::Category,
                }
            }
            // Render any components at the root level (no category)
            for component_name in tree.components.iter() {
                {
                    let component_name = component_name.clone();
                    let stories: Vec<String> = crate::find_component(&component_name)
                        .map(|reg| (reg.get_stories)().into_iter().map(|s| s.title).collect())
                        .unwrap_or_default();

                    rsx! {
                        ComponentNode {
                            key: "{component_name}",
                            name: component_name.clone(),
                            selected,
                            stories,
                        }
                    }
                }
            }
        }
    }
}


