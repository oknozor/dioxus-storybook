use serde::{Deserialize, Serialize};

/// Represents the available viewport size presets for story preview.
#[derive(Clone, Copy, PartialEq, Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub enum ViewportSize {
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

