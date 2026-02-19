use crate::ui::models::ViewportSize;
use crate::{self as storybook};
use crate::{Stories, Story, storybook};
use dioxus::prelude::*;
#[storybook(tag = "Molecules")]
#[component]
pub fn ViewPortSelector(viewport_width: Signal<ViewportSize>) -> Element {
    rsx! {
        select {
            class: "top-bar-viewport-select",
            title: "Viewport Size",
            value: "{viewport_width().value()}",
            onchange: move |e: Event<FormData>| {
                let size = ViewportSize::from_value(&e.value());
                viewport_width.set(size);
            },
            for size in ViewportSize::all() {
                option {
                    value: "{size.value()}",
                    selected: viewport_width() == *size,
                    "{size.label()}"
                }
            }
        }
    }
}

impl Stories for ViewPortSelectorProps {
    fn stories() -> Vec<Story<Self>> {
        vec![Story::new(
            "Default",
            Self {
                viewport_width: Signal::new(ViewportSize::FullWidth),
            },
        )]
    }
}
