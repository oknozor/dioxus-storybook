use crate::ui::doc_page::DocPage;
use crate::ui::sidebar::{ComponentInfo, Selection, Sidebar};
use crate::{STORYBOOK_CSS, get_components, take_config, Stories, Story};
use dioxus::prelude::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use storybook_macro::storybook;
use crate::ui::shared::{FullscreenButton, GridButton, OutlineButton, ResetZoomButton, ThemeToggleButton, ViewPortSelector, ZoomInButton, ZoomOutButton};
use crate::{self as storybook};
use crate::ui::story::{StoryPage, StoryZoomControls};

pub mod doc_page;
pub mod sidebar;
pub mod story;
pub mod shared;
/// Represents the available viewport size presets for story preview.
#[derive(Clone, Copy, PartialEq, Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub(crate) enum ViewportSize {
    FullWidth,
    SmallMobile,
    LargeMobile,
    Tablet,
}

impl ViewportSize {
    /// Returns the pixel width constraint, or `None` for full width.
    pub fn to_width(self) -> &'static str {
        match self {
            ViewportSize::FullWidth => "100%",
            ViewportSize::SmallMobile => "375px",
            ViewportSize::LargeMobile => "428px",
            ViewportSize::Tablet => "768px",
        }
    }

    /// Returns a human-readable label for display in the dropdown.
    pub fn label(self) -> &'static str {
        match self {
            ViewportSize::FullWidth => "Full Width",
            ViewportSize::SmallMobile => "Small Mobile (375px)",
            ViewportSize::LargeMobile => "Large Mobile (428px)",
            ViewportSize::Tablet => "Tablet (768px)",
        }
    }

    /// Returns a short string value used as the `<option>` value attribute.
    pub fn value(self) -> &'static str {
        match self {
            ViewportSize::FullWidth => "full",
            ViewportSize::SmallMobile => "375",
            ViewportSize::LargeMobile => "428",
            ViewportSize::Tablet => "768",
        }
    }

    /// Parse from the `<option>` value string.
    pub fn from_value(s: &str) -> Self {
        match s {
            "375" => ViewportSize::SmallMobile,
            "428" => ViewportSize::LargeMobile,
            "768" => ViewportSize::Tablet,
            _ => ViewportSize::FullWidth,
        }
    }

    /// All variants in display order.
    pub fn all() -> &'static [ViewportSize] {
        &[
            ViewportSize::FullWidth,
            ViewportSize::SmallMobile,
            ViewportSize::LargeMobile,
            ViewportSize::Tablet,
        ]
    }
}

/// Global UI settings shared via context
#[derive(Clone, Copy)]
pub struct UiSettings {
    pub is_dark_theme: Signal<bool>,
    pub grid_enabled: Signal<bool>,
    pub outline_enabled: Signal<bool>,
    pub fullscreen: Signal<bool>,
    pub zoom_level: Signal<i32>,
    pub viewport_width: Signal<ViewportSize>,
}

impl Default for UiSettings {
    fn default() -> Self {
        UiSettings {
            is_dark_theme: Signal::new(false),
            grid_enabled: Signal::new(false),
            outline_enabled: Signal::new(false),
            fullscreen: Signal::new(false),
            zoom_level: Signal::new(100),
            viewport_width: Signal::new(ViewportSize::FullWidth),
        }
    }
}

#[component]
pub(crate) fn App() -> Element {
    // Take the config from thread-local storage and provide it as context
    let _config = use_context_provider(take_config);

    // Provide UI settings as context
    let _ui_settings = use_context_provider(|| UiSettings::default());

    rsx! {
        Stylesheet { href: STORYBOOK_CSS }
        Storybook {}
    }
}

/// Top navigation bar with theme, grid, outline, fullscreen toggles, and story-specific controls
#[storybook(tag = "Organisms")]
#[component]
fn TopBar(selected: Signal<Option<Selection>>) -> Element {
    let ui_settings = use_context::<UiSettings>();
    let zoom = (ui_settings.zoom_level)();
    let is_story_selected = matches!(selected(), Some(Selection::Story(_, _)));

    rsx! {
        div { class: "top-bar",
            div { class: "top-bar-left",
                ThemeToggleButton { is_dark_theme: ui_settings.is_dark_theme }
                GridButton { grid_enabled: ui_settings.grid_enabled }
                OutlineButton { outline_enabled: ui_settings.outline_enabled }

                if is_story_selected {
                    div { class: "top-bar-divider" }
                    StoryZoomControls { zoom_level: ui_settings.zoom_level }
                    div { class: "top-bar-divider" }
                    ViewPortSelector { viewport_width: ui_settings.viewport_width }
                }
            }

            div { class: "top-bar-right",
                FullscreenButton { fullscreen_on: ui_settings.fullscreen }
            }
        }
    }
}

impl Stories for TopBarProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new("Default", Self {
                selected: Signal::new(None),
            }),
        ]
    }
}

#[component]
fn Storybook() -> Element {
    let ui_settings = use_context::<UiSettings>();
    let search_query = use_signal(String::new);
    let selected = use_signal(|| Option::<Selection>::None);
    let components = use_store(|| ComponentStore {
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

    let filtered_components = use_memo(move || components.search(&search_query()));

    let container_class = use_memo(move || {
        let mut classes = vec!["storybook-container"];
        if (ui_settings.is_dark_theme)() {
            classes.push("dark-theme");
        }
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
                            Some(Selection::Story(component_name, story_index)) => rsx! {
                                StoryPage {
                                    key: "{component_name}-{story_index}",
                                    component_name,
                                    story_index,
                                }
                            },
                            Some(Selection::DocPage(doc_path)) => rsx! {
                                DocPage { key: "{doc_path}", path: doc_path }
                            },
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

#[derive(Store, PartialEq, Clone, Debug)]
pub(crate) struct ComponentStore {
    components: HashMap<String, ComponentInfo>,
}

#[store]
impl<Lens> Store<ComponentStore, Lens> {
    fn search(&self, query: &str) -> Vec<ComponentInfo> {
        self.components()
            .values()
            .filter(|c| {
                c.read().name.to_lowercase().contains(query)
                    || c.read().category.to_lowercase().contains(query)
            })
            .map(|c| c())
            .collect()
    }
}
