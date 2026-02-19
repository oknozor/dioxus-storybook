use crate::ui::models::Selection;
use dioxus::prelude::*;
use lucide_dioxus::{BookOpen, ChevronRight, Component};

/// Expandable component node that shows individual stories as children
#[component]
pub fn ComponentNode(
    name: String,
    selected: Signal<Option<Selection>>,
    stories: Vec<String>,
) -> Element {
    let component_name = name.clone();
    let current_name = name.clone();

    let is_component_active = use_memo(move || match selected() {
        Some(Selection::Story(ref cn, _)) => cn == &current_name,
        _ => false,
    });

    rsx! {
        div { class: "component-node-group",
            RootNode {
                name: name.clone(),
                expanded: is_component_active(),
                selected,
            }
            if is_component_active() {
                div { class: "story-children",
                    for (index , story_title) in stories.iter().enumerate() {
                        {
                            let component_name = component_name.clone();
                            let is_selected = selected()
                                == Some(Selection::Story(component_name.clone(), index));
                            let story_title = story_title.clone();
                            rsx! {
                                StoryNode {
                                    key: "{component_name}-story-{index}",
                                    is_selected,
                                    onclick: move |_| selected.set(Some(Selection::Story(component_name.clone(), index))),
                                    story_title,
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
fn RootNode(name: String, expanded: bool, selected: Signal<Option<Selection>>) -> Element {
    rsx! {
        div {
            class: if expanded { "component-node active" } else { "component-node" },
            onclick: move |_| {
                selected.set(Some(Selection::Story(name.clone(), 0)));
            },
            span { class: if expanded { "arrow expanded" } else { "arrow" },
                ChevronRight { size: 12, stroke_width: 2 }
            }
            span { class: "component-icon",
                Component { size: 14, stroke_width: 2 }
            }
            span { class: "component-name", "{name}" }
        }
    }
}
#[component]
fn StoryNode(
    is_selected: bool,
    story_title: String,
    #[props(extends = GlobalAttributes, extends = tr)]
    attributes: Vec<Attribute>,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div {
            class: if is_selected { "story-node selected" } else { "story-node" },
            onclick: move |e| {
                onclick.call(e);
            },
            ..attributes,
            span { class: "story-icon",
                BookOpen { size: 12, stroke_width: 2 }
            }
            span { class: "story-name", "{story_title}" }
        }
    }
}
