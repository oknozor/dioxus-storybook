use dioxus::prelude::*;

#[cfg(feature = "self-stories")]
use crate::{self as storybook};

#[cfg(feature = "self-stories")]
use storybook_macro::storybook;

/// Styled table row for the props editor table.
///
/// Wraps a native `<tr>` element with the `prop-row editable` CSS classes.
/// Accepts spread attributes and renders children (typically `Td` cells)
/// inside the row.
///
/// # Props
///
/// | Prop | Type | Description |
/// |------|------|-------------|
/// | `attributes` | `Vec<Attribute>` | Spread attributes forwarded to the `<tr>`. |
/// | `children` | `Element` | The table cells to render inside the row. |
///
/// @[story:Atoms/Tr/Default]
#[cfg_attr(feature = "self-stories", storybook(tag = "Atoms"))]
#[component]
pub fn Tr(
    #[props(extends = GlobalAttributes, extends = tr)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        tr { class: "prop-row editable", ..attributes, {children} }
    }
}

/// Styled table cell for the props editor table.
///
/// Wraps a native `<td>` element with the `prop-cell` CSS class. Accepts
/// spread attributes and renders arbitrary children.
///
/// # Props
///
/// | Prop | Type | Description |
/// |------|------|-------------|
/// | `attributes` | `Vec<Attribute>` | Spread attributes forwarded to the `<td>`. |
/// | `children` | `Element` | The content to render inside the cell. |
///
/// @[story:Atoms/Td/Default]
#[cfg_attr(feature = "self-stories", storybook(tag = "Atoms"))]
#[component]
pub fn Td(
    #[props(extends = GlobalAttributes, extends = td)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        td { class: "prop-cell", ..attributes, {children} }
    }
}
