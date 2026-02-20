use crate::ui::models::{CategoryTreeNode, NodeType, Selection};
use crate::ui::view::sidebar::node::ComponentNode;
use crate::ui::viewmodels::sidebar_vm::{get_story_titles, has_component_docs};
#[cfg(feature = "self-stories")]
use crate::{self as storybook};
use dioxus::prelude::*;
use lucide_dioxus::{ChevronRight, FileText, Folder, FolderOpen};

#[cfg(feature = "self-stories")]
use storybook_macro::storybook;

/// Recursive tree node for rendering categories and folders in the sidebar.
///
/// `TreeNode` is the backbone of the sidebar navigation. It renders a
/// collapsible header (with a folder icon and component count badge) and
/// recursively renders child folders, documentation links, and
/// [`ComponentNode`] entries.
///
/// # Props
///
/// | Prop | Type | Description |
/// |------|------|-------------|
/// | `name` | `String` | Display name of this tree level. |
/// | `node` | `CategoryTreeNode` | The tree data for this level (children, components, docs). |
/// | `selected` | `Signal<Option<Selection>>` | Currently selected item in the sidebar. |
/// | `node_type` | `NodeType` | Whether this node is a top-level `Category` or a nested `Folder`. |
///
/// @[story:Molecules/TreeNode/Category Node]
///
/// @[story:Molecules/TreeNode/Folder Node]
#[cfg_attr(feature = "self-stories", storybook(tag = "Molecules"))]
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
            TreeNodeHeader { expanded, name: name.clone(), component_count }
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
    }
}

#[component]
pub(crate) fn DocNode(
    selected: Signal<Option<Selection>>,
    path: String,
    #[props(default = String::from("Documentation"))] label: String,
) -> Element {
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
            span { class: "doc-name", "{label}" }
        }
    }
}

#[component]
fn TreeNodeHeader(expanded: Signal<bool>, name: String, component_count: usize) -> Element {
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
