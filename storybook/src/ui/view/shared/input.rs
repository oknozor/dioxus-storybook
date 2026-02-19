use dioxus::prelude::*;

#[cfg(feature = "self-stories")]
use crate::{self as storybook};

#[cfg(feature = "self-stories")]
use storybook_macro::storybook;

/// Styled checkbox input used in the props editor.
///
/// Wraps a native `<input type="checkbox">` with the `prop-input prop-checkbox`
/// CSS classes. Accepts spread attributes so callers can set `checked`,
/// `disabled`, or any other HTML input attribute.
///
/// # Props
///
/// | Prop | Type | Description |
/// |------|------|-------------|
/// | `attributes` | `Vec<Attribute>` | Spread attributes forwarded to the inner `<input>`. |
/// | `onchange` | `EventHandler<bool>` | Fires with the new checked state on toggle. |
///
/// @[story:Atoms/Checkbox/Checked]
///
/// @[story:Atoms/Checkbox/Unchecked]
#[cfg_attr(feature = "self-stories", storybook(tag = "Atoms"))]
#[component]
pub fn Checkbox(
    #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
    onchange: EventHandler<bool>,
) -> Element {
    rsx! {
        input {
            class: "prop-input prop-checkbox",
            r#type: "checkbox",
            onchange: move |e| onchange.call(e.checked()),
            ..attributes,
        }
    }
}

/// Styled text input used in the props editor.
///
/// Wraps a native `<input>` with the `prop-input` CSS class. The `type`
/// attribute can be overridden via spread attributes (e.g. `type: "number"`).
///
/// # Props
///
/// | Prop | Type | Description |
/// |------|------|-------------|
/// | `oninput` | `EventHandler<String>` | Fires with the current input value on every keystroke. |
/// | `attributes` | `Vec<Attribute>` | Spread attributes forwarded to the inner `<input>`. |
///
/// @[story:Atoms/TextInput/Default]
#[cfg_attr(feature = "self-stories", storybook(tag = "Atoms"))]
#[component]
pub fn TextInput(
    oninput: EventHandler<String>,
    #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        input {
            class: "prop-input",
            oninput: move |e| oninput.call(e.value()),
            ..attributes,
        }
    }
}
