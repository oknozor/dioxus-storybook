use dioxus::prelude::*;
use crate::ui::models::{ComponentInfo, Selection, NodeType};
use crate::ui::view::sidebar::node::ComponentNode;
use crate::ui::view::sidebar::tree::TreeNode;
use crate::ui::view::sidebar::search_input::SearchInput;
use crate::ui::services::category_builder::build_category_tree;

mod search_input;
mod tree;
mod node;

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


