use crate::ui::models::{CategoryTreeNode, ComponentInfo};

/// Build a tree structure from flat component info **and** doc registrations.
///
/// After inserting every component, the builder iterates over all
/// [`DocRegistration`](crate::DocRegistration) entries and ensures that a tree
/// node exists for each doc path — even when no components share that path.
/// This allows root-level (or otherwise orphan) doc pages to appear in the
/// sidebar.
pub fn build_category_tree(components: &[ComponentInfo]) -> CategoryTreeNode {
    let mut root = CategoryTreeNode::default();

    // 1. Insert components
    for component in components {
        let path_segments: Vec<&str> = component.category.split('/').collect();
        root.insert(&path_segments, component.name.clone(), "");
    }

    // 2. Ensure tree nodes exist for every doc registration path
    for doc in crate::get_docs() {
        let path_segments: Vec<&str> = doc.path.split('/').collect();
        root.insert_doc_path(&path_segments, "");
    }

    root
}
