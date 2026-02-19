use crate::ui::models::{CategoryTreeNode, ComponentInfo};

/// Build a tree structure from flat component info.
pub fn build_category_tree(components: &[ComponentInfo]) -> CategoryTreeNode {
    let mut root = CategoryTreeNode::default();

    for component in components {
        let path_segments: Vec<&str> = component.category.split('/').collect();
        root.insert(&path_segments, component.name.clone(), "");
    }

    root
}
