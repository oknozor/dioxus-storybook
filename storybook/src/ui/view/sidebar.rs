use crate::ui::models::{ComponentInfo, NodeType, Selection};
use crate::ui::services::category_builder::build_category_tree;
use crate::ui::view::sidebar::node::ComponentNode;
use crate::ui::view::sidebar::search_input::SearchInput;
use crate::ui::view::sidebar::tree::TreeNode;
use crate::ui::viewmodels::sidebar_vm::{get_story_titles, has_component_docs};
use dioxus::prelude::*;

mod node;
mod search_input;
mod tree;

#[cfg(feature = "self-stories")]
mod stories;

#[component]
pub fn Sidebar(
    search_query: Signal<String>,
    components: Vec<ComponentInfo>,
    selected: Signal<Option<Selection>>,
) -> Element {
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
                    let stories = get_story_titles(&component_name);
                    let has_docs = has_component_docs(&component_name);
                    let doc_path = format!("__component__/{component_name}");
                    let is_active = matches!(
                        selected(),
                        Some(Selection::Story(ref cn, _))
                        if cn == &component_name
                    ) || selected() == Some(Selection::DocPage(doc_path));
                    rsx! {
                        ComponentNode {
                            key: "{component_name}",
                            name: component_name.clone(),
                            selected,
                            stories,
                            is_active,
                            has_docs,
                        }
                    }
                }
            }
        }
    }
}
