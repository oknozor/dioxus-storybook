use std::collections::HashMap;
use dioxus::prelude::*;
use crate::ui::sidebar::ComponentInfo;

#[derive(Store, PartialEq, Clone, Debug)]
pub(crate) struct ComponentStore {
    pub components: HashMap<String, ComponentInfo>,
}

impl ComponentStore {
    /// Filter components by search query (matches name or category, case-insensitive).
    pub(crate) fn search(&self, query: &str) -> Vec<ComponentInfo> {
        let query = query.to_lowercase();
        self.components
            .values()
            .filter(|c| {
                c.name.to_lowercase().contains(&query)
                    || c.category.to_lowercase().contains(&query)
            })
            .cloned()
            .collect()
    }
}

