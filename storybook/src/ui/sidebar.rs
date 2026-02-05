use dioxus::prelude::*;
use std::collections::BTreeMap;
use crate::find_doc;
use lucide_dioxus::{ChevronRight, FileText, Folder, FolderOpen, Component};

#[derive(Clone, PartialEq, Debug)]
pub struct ComponentInfo {
    pub name: String,
    pub category: String,
}

/// Selection type - either a component or a doc page
#[derive(Clone, PartialEq, Debug)]
pub enum Selection {
    Component(String),
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

/// A tree node that can contain either subcategories or components (or both)
#[derive(Clone, PartialEq, Debug, Default)]
struct CategoryTreeNode {
    /// Subcategories indexed by their name segment
    children: BTreeMap<String, CategoryTreeNode>,
    /// Components directly under this category
    components: Vec<String>,
    /// Full path to this node (e.g., "Category/Folder")
    full_path: String,
    /// Whether this node has an associated doc page
    has_doc: bool,
}

impl CategoryTreeNode {
    /// Insert a component at the given path
    fn insert(&mut self, path: &[&str], component_name: String, current_path: &str) {
        if path.is_empty() {
            self.components.push(component_name);
        } else {
            let new_path = if current_path.is_empty() {
                path[0].to_string()
            } else {
                format!("{}/{}", current_path, path[0])
            };
            let child = self.children.entry(path[0].to_string()).or_insert_with(|| {
                CategoryTreeNode {
                    full_path: new_path.clone(),
                    has_doc: crate::find_doc(&new_path).is_some(),
                    ..Default::default()
                }
            });
            child.insert(&path[1..], component_name, &new_path);
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
        root.insert(&path_segments, component.name.clone(), "");
    }

    root
}

#[component]
pub fn ComponentTree(
    components: Vec<ComponentInfo>,
    selected: Signal<Option<Selection>>,
) -> Element {
    let tree = build_category_tree(&components);

    info!("ComponentTree: {:?} top-level categories", tree.children.len());

    rsx! {
        div { class: "tree",
            // Render top-level categories (depth 0 = Category)
            for (category_name , node) in tree.children.iter() {
                TreeNode {
                    key: "{category_name}",
                    name: category_name.clone(),
                    node: node.clone(),
                    selected,
                    node_type: NodeType::Category
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
                            selected: selected() == Some(Selection::Component(component_name.clone())),
                            onclick: move |_| {
                                selected.set(Some(Selection::Component(component_name.clone())));
                            },
                        }
                    }
                }
            }
        }
    }
}

/// Recursive component for rendering tree nodes (categories and folders)
#[component]
fn TreeNode(
    name: String,
    node: CategoryTreeNode,
    selected: Signal<Option<Selection>>,
    node_type: NodeType,
) -> Element {
    let mut expanded = use_signal(|| true);
    let component_count = node.component_count();
    let has_doc = node.has_doc;
    let full_path = node.full_path.clone();

    // Determine CSS class based on node type
    let node_class = match node_type {
        NodeType::Category => "tree-node category-node",
        NodeType::Folder => "tree-node folder-node",
    };

    rsx! {
        div { class: "{node_class}",
            div {
                class: "tree-header",
                onclick: move |_| expanded.set(!expanded()),
                span { class: if expanded() { "arrow expanded" } else { "arrow" },
                    ChevronRight { size: 14, stroke_width: 2 }
                }
                span { class: "node-icon",
                    if expanded() {
                        FolderOpen { size: 16, stroke_width: 2 }
                    } else {
                        Folder { size: 16, stroke_width: 2 }
                    }
                }
                span { class: "node-name", "{name}" }
                span { class: "category-count", "{component_count}" }
            }
            if expanded() {
                div { class: "tree-children",
                    // Render doc page link first if this node has documentation
                    if has_doc {
                        {
                            let doc_path = full_path.clone();
                            let is_selected = selected() == Some(Selection::DocPage(doc_path.clone()));
                            rsx! {
                                div {
                                    class: if is_selected { "doc-node selected" } else { "doc-node" },
                                    onclick: move |_| {
                                        selected.set(Some(Selection::DocPage(doc_path.clone())));
                                    },
                                    span { class: "doc-icon",
                                        FileText { size: 16, stroke_width: 2 }
                                    }
                                    span { class: "doc-name", "Documentation" }
                                }
                            }
                        }
                    }
                    // Render nested subcategories/folders
                    for (child_name , child_node) in node.children.iter() {
                        TreeNode {
                            key: "{child_name}",
                            name: child_name.clone(),
                            node: child_node.clone(),
                            selected,
                            node_type: NodeType::Folder
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
                                    selected: selected() == Some(Selection::Component(component_name.clone())),
                                    onclick: move |_| {
                                        selected.set(Some(Selection::Component(component_name.clone())));
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
            span { class: "component-icon",
                Component { size: 16, stroke_width: 2 }
            }
            span { class: "component-name", "{name}" }
        }
    }
}
