use std::collections::BTreeMap;
use dioxus::prelude::*;
use lucide_dioxus::{FolderOpen, Folder, ChevronRight, FileText};
use crate::ui::sidebar::{ComponentInfo, node::ComponentNode, NodeType, Selection};

/// Build a tree structure from flat component info
pub fn build_category_tree(components: &[ComponentInfo]) -> CategoryTreeNode {
    let mut root = CategoryTreeNode::default();

    for component in components {
        // Split the category path by forward slashes
        let path_segments: Vec<&str> = component.category.split('/').collect();
        root.insert(&path_segments, component.name.clone(), "");
    }

    root
}


/// A tree node that can contain either subcategories or components (or both)
#[derive(Clone, PartialEq, Debug, Default)]
pub struct CategoryTreeNode {
    /// Subcategories indexed by their name segment
    pub children: BTreeMap<String, CategoryTreeNode>,
    /// Components directly under this category
    pub components: Vec<String>,
    /// Full path to this node (e.g., "Category/Folder")
    pub full_path: String,
    /// Whether this node has an associated doc page
    pub has_doc: bool,
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
            let child =
                self.children
                    .entry(path[0].to_string())
                    .or_insert_with(|| CategoryTreeNode {
                        full_path: new_path.clone(),
                        has_doc: crate::find_doc(&new_path).is_some(),
                        ..Default::default()
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


/// Recursive component for rendering tree nodes (categories and folders)
#[component]
pub fn TreeNode(
    name: String,
    node: CategoryTreeNode,
    selected: Signal<Option<Selection>>,
    node_type: NodeType,
) -> Element {
    let expanded = use_signal(|| true);
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
            TreeNodeHeader {
                expanded: expanded.clone(),
                name: name.clone(),
                component_count,
            }
            if expanded() {
                div { class: "tree-children",
                    if has_doc {
                        DocNode { path: full_path.clone(), selected }
                    }

                    for (child_name , child_node) in node.children.iter() {
                        TreeNode {
                            key: "{child_name}",
                            name: child_name.clone(),
                            node: child_node.clone(),
                            selected,
                            node_type: NodeType::Folder,
                        }
                    }
                    // Then render components at this level
                    for component_name in node.components.iter() {
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
    }
}

#[component]
fn DocNode(selected: Signal<Option<Selection>>, path: String) -> Element {
    let doc_path = path.clone();
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

#[component]
fn TreeNodeHeader(expanded: Signal<bool>, name: String, component_count: usize) -> Element    {
    rsx! {
        div { class: "tree-header", onclick: move |_| expanded.toggle(),
            span { class: if expanded() { "arrow expanded" } else { "arrow" },
                ChevronRight { size: 14, stroke_width: 2 }
            }
            FolderIcon { expanded }
            span { class: "node-name", "{name}" }
            span { class: "category-count", "{component_count}" }
        }
    }
}

#[component]
fn FolderIcon(expanded: Signal<bool>) -> Element {
    rsx! {
        span { class: "node-icon",
            if expanded() {
                FolderOpen { size: 16, stroke_width: 2 }
            } else {
                Folder { size: 16, stroke_width: 2 }
            }
        }
    }
}
