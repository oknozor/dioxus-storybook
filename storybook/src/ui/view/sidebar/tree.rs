use dioxus::prelude::*;
use lucide_dioxus::{FolderOpen, Folder, ChevronRight, FileText};
use crate::ui::view::sidebar::node::ComponentNode;
use crate::ui::models::{CategoryTreeNode, NodeType, Selection};

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
