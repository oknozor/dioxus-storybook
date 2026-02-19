use crate::ui::models::ViewportSize;
#[cfg(feature = "self-stories")]
use crate::{self as storybook};
#[cfg(feature = "self-stories")]
use crate::{Stories, Story, storybook};
use dioxus::prelude::*;
/// Dropdown selector for changing the story preview viewport width.
///
/// Renders a `<select>` element populated with all available
/// [`ViewportSize`] variants (e.g. Full Width, Mobile, Tablet, Desktop).
/// Selecting a different option updates the reactive `viewport_width`
/// signal, which in turn resizes the story preview iframe.
///
/// # Props
///
/// | Prop | Type | Description |
/// |------|------|-------------|
/// | `viewport_width` | `Signal<ViewportSize>` | The currently selected viewport size. |
///
/// @[story:Molecules/ViewPortSelector/Default]
#[cfg_attr(feature = "self-stories", storybook(tag = "Molecules"))]
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

#[cfg(feature = "self-stories")]
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
