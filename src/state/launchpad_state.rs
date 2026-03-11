use leptos::prelude::*;
use std::collections::HashSet;

#[derive(Clone)]
pub struct LaunchpadState {
    pub query: RwSignal<String>,
    pub active_filter: RwSignal<String>,
    pub collapsed: RwSignal<HashSet<String>>,
}

impl LaunchpadState {
    pub fn new() -> Self {
        Self {
            query: RwSignal::new(String::new()),
            active_filter: RwSignal::new("all".to_string()),
            collapsed: RwSignal::new(HashSet::new()),
        }
    }

    pub fn toggle_collapsed(&self, category: &str) {
        let category = category.to_string();
        self.collapsed.update(|set| {
            if set.contains(&category) {
                set.remove(&category);
            } else {
                set.insert(category);
            }
        });
    }

    pub fn is_collapsed(&self, category: &str) -> bool {
        self.collapsed.with(|set| set.contains(category))
    }
}
