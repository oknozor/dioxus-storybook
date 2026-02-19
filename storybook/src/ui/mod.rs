use crate::ui::models::{ComponentInfo, Selection};
use crate::ui::view::doc_page::DocPage;
use crate::ui::view::sidebar::Sidebar;
use crate::{STORYBOOK_CSS, find_doc, get_components, take_config};
use dioxus::prelude::*;

// MVVM layers
pub mod models;
pub mod services;
pub mod viewmodels;

// View layer
pub mod view;

// Re-export commonly used items for the public API
pub(crate) use view::top_bar::TopBar;
pub use viewmodels::UiSettings;

use crate::ui::view::story::StoryPage;
use crate::ui::viewmodels::story_page_vm::{StoryPageError, resolve_story_page};

#[component]
pub(crate) fn App() -> Element {
    // Take the config from thread-local storage and provide it as context
    let _config = use_context_provider(take_config);

    // Provide UI settings as context
    let _ui_settings = use_context_provider(UiSettings::default);

    rsx! {
        Stylesheet { href: STORYBOOK_CSS }
        Storybook {}
    }
}

#[component]
fn Storybook() -> Element {
    let ui_settings = use_context::<UiSettings>();
    let search_query = use_signal(String::new);
    let selected = use_signal(|| Option::<Selection>::None);
    let components = use_store(|| viewmodels::ComponentStore {
        components: get_components()
            .map(|c| {
                (
                    c.name.to_string(),
                    ComponentInfo {
                        name: c.name.to_string(),
                        category: c.tag.to_string(),
                    },
                )
            })
            .collect(),
    });

    let filtered_components = use_memo(move || components().search(&search_query()));

    let container_class = use_memo(move || {
        let mut classes = vec!["storybook-container"];
        if (ui_settings.fullscreen)() {
            classes.push("fullscreen-mode");
        }
        classes.join(" ")
    });

    rsx! {
        div { class: "{container_class}",
            TopBar { selected }

            div { class: "content-wrapper",
                if !(ui_settings.fullscreen)() {
                    Sidebar {
                        search_query,
                        components: filtered_components(),
                        selected,
                    }
                }

                div { class: "main-content",
                    div { class: "component-preview",
                        match selected() {
                            Some(Selection::Story(component_name, story_index)) => {
                                match resolve_story_page(&component_name, story_index) {
                                    Ok(data) => rsx! {
                                        StoryPage {
                                            key: "{component_name}-{story_index}",
                                            component_name,
                                            story_index,
                                            story: data.story,
                                            story_title: data.story_title,
                                            render_fn: data.render_fn,
                                            prop_schema: data.prop_schema,
                                            description: data.description,
                                        }
                                    },
                                    Err(StoryPageError::ComponentNotFound(name)) => rsx! {
                                        div { class: "error", "Component not found: {name}" }
                                    },
                                    Err(StoryPageError::StoryNotFound { component_name, story_index }) => {
                                        rsx! {
                                            div { class: "error", "Story not found: index {story_index} for {component_name}" }
                                        }
                                    }
                                }
                            }
                            Some(Selection::DocPage(doc_path)) => {
                                match find_doc(&doc_path) {
                                    Some(doc) => rsx! {
                                        DocPage { key: "{doc_path}", content_html: doc.content_html.to_string() }
                                    },
                                    None => rsx! {
                                        div { class: "error", "Documentation not found: {doc_path}" }
                                    },
                                }
                            }
                            None => rsx! {
                                div { class: "empty-state",
                                    h2 { "Select a story" }
                                    p { "Choose a component and story from the sidebar to preview it" }
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
