use crate::Decorator;
use dioxus::prelude::*;

/// Apply decorators to an element.
/// Decorators are applied in order, with the first decorator being the outermost wrapper.
pub fn apply_decorators(element: Element, decorators: &[Decorator]) -> Element {
    decorators
        .iter()
        .rev()
        .fold(element, |acc, decorator| decorator(acc))
}
