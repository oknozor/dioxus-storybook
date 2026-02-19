use std::collections::BTreeMap;

/// A tree node that can contain either subcategories or components (or both).
///
/// This is a pure data model — it has no UI rendering logic.
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
    pub(crate) fn insert(&mut self, path: &[&str], component_name: String, current_path: &str) {
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
    pub fn component_count(&self) -> usize {
        let direct_count = self.components.len();
        let children_count: usize = self.children.values().map(|c| c.component_count()).sum();
        direct_count + children_count
    }
}

