use dioxus::prelude::*;
use crate::ui::viewmodels::ui_settings::UiSettings;

const HLJS_VERSION: &str = "11.11.1";
const HLJS_LIGHT_THEME: &str = "github";
const HLJS_DARK_THEME: &str = "github-dark";

/// The highlight.js script URL (loaded once per doc page).
pub const HLJS_SCRIPT_URL: &str = concat!(
    "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/",
    "11.11.1",
    "/highlight.min.js"
);

/// Custom hook that manages highlight.js theme switching based on dark/light mode.
///
/// Injects or updates the highlight.js theme stylesheet and re-highlights all
/// code blocks whenever the theme changes.
pub fn use_hljs_theme() {
    let ui_settings = use_context::<UiSettings>();
    let is_dark = (ui_settings.is_dark_theme)();

    use_effect(move || {
        let theme = if is_dark { HLJS_DARK_THEME } else { HLJS_LIGHT_THEME };
        let css_url = format!(
            "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/{HLJS_VERSION}/styles/{theme}.min.css"
        );
        // Create or update the highlight.js theme stylesheet and re-highlight all code blocks
        document::eval(&format!(
            r#"
            // Create or update the highlight.js theme link element
            var link = document.getElementById('hljs-theme');
            if (!link) {{
                link = document.createElement('link');
                link.id = 'hljs-theme';
                link.rel = 'stylesheet';
                document.head.appendChild(link);
            }}
            link.href = "{css_url}";
            // Wait for the DOM to update and script to load, then highlight
            setTimeout(function() {{
                if (typeof hljs !== 'undefined') {{
                    // Remove previous highlighting so hljs re-processes the blocks
                    document.querySelectorAll('pre code[data-highlighted]').forEach(function(el) {{
                        el.removeAttribute('data-highlighted');
                    }});
                    hljs.highlightAll();
                }}
            }}, 100);
            "#
        ));
    });
}

