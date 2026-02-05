use dioxus::prelude::*;
use std::collections::BTreeMap;

#[derive(Clone, PartialEq, Debug)]
pub struct ComponentInfo {
    pub name: String,
    pub category: String,
}

/// A tree node that can contain either subcategories or components (or both)
#[derive(Clone, PartialEq, Debug, Default)]
struct CategoryTreeNode {
    /// Subcategories indexed by their name segment
    children: BTreeMap<String, CategoryTreeNode>,
    /// Components directly under this category
    components: Vec<String>,
}

impl CategoryTreeNode {
    /// Insert a component at the given path
    fn insert(&mut self, path: &[&str], component_name: String) {
        if path.is_empty() {
            self.components.push(component_name);
        } else {
            let child = self.children.entry(path[0].to_string()).or_default();
            child.insert(&path[1..], component_name);
        }
    }

    /// Count all components in this node and all its children recursively
    fn component_count(&self) -> usize {
        let direct_count = self.components.len();
        let children_count: usize = self.children.values().map(|c| c.component_count()).sum();
        direct_count + children_count
    }
}

/// Build a tree structure from flat component info
fn build_category_tree(components: &[ComponentInfo]) -> CategoryTreeNode {
    let mut root = CategoryTreeNode::default();

    for component in components {
        // Split the category path by forward slashes
        let path_segments: Vec<&str> = component.category.split('/').collect();
        root.insert(&path_segments, component.name.clone());
    }

    root
}

#[component]
pub fn ComponentTree(
    components: Vec<ComponentInfo>,
    selected_component: Signal<Option<String>>,
) -> Element {
    let tree = build_category_tree(&components);

    info!("ComponentTree: {:?} top-level categories", tree.children.len());

    rsx! {
        div { class: "tree",
            // Render top-level categories
            for (category_name , node) in tree.children.iter() {
                CategoryNode {
                    key: "{category_name}",
                    name: category_name.clone(),
                    node: node.clone(),
                    selected_component,
                    depth: 0
                }
            }
            // Render any components at the root level (no category)
            for component_name in tree.components.iter() {
                {
                    let component_name = component_name.clone();
                    rsx! {
                        ComponentNode {
                            key: "{component_name}",
                            name: component_name.clone(),
                            selected: selected_component() == Some(component_name.clone()),
                            onclick: move |_| {
                                selected_component.set(Some(component_name.clone()));
                            },
                        }
                    }
                }
            }
        }
    }
}

/// Recursive component for rendering category nodes with nested children
#[component]
fn CategoryNode(
    name: String,
    node: CategoryTreeNode,
    selected_component: Signal<Option<String>>,
    depth: usize,
) -> Element {
    let mut expanded = use_signal(|| true);
    let component_count = node.component_count();

    rsx! {
        div { class: "category-node",
            div {
                class: "category-header",
                onclick: move |_| expanded.set(!expanded()),
                span { class: if expanded() { "arrow expanded" } else { "arrow" }, "▶" }
                span { class: "category-name", "{name}" }
                span { class: "category-count", "{component_count}" }
            }
            if expanded() {
                div { class: "category-children",
                    // Render nested subcategories first
                    for (child_name , child_node) in node.children.iter() {
                        CategoryNode {
                            key: "{child_name}",
                            name: child_name.clone(),
                            node: child_node.clone(),
                            selected_component,
                            depth: depth + 1
                        }
                    }
                    // Then render components at this level
                    for component_name in node.components.iter() {
                        {
                            let component_name = component_name.clone();
                            rsx! {
                                ComponentNode {
                                    key: "{component_name}",
                                    name: component_name.clone(),
                                    selected: selected_component() == Some(component_name.clone()),
                                    onclick: move |_| {
                                        selected_component.set(Some(component_name.clone()));
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ComponentNode(name: String, selected: bool, onclick: EventHandler<()>) -> Element {
    rsx! {
        div { class: if selected { "component-node selected" } else { "component-node" }, onclick: move |_| onclick.call(()),
            span { class: "component-icon", "📦" }
            span { class: "component-name", "{name}" }
        }
    }
}
