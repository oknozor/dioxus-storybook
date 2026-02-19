use dioxus::prelude::*;

const HLJS_VERSION: &str = "11.11.1";
const HLJS_THEME: &str = "github";

/// The highlight.js script URL (loaded once per doc page).
pub const HLJS_SCRIPT_URL: &str = concat!(
    "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/",
    "11.11.1",
    "/highlight.min.js"
);

/// Custom hook that manages the highlight.js theme stylesheet.
///
/// Injects the highlight.js light theme stylesheet and highlights all
/// code blocks on mount.
pub fn use_hljs_theme() {
    use_effect(move || {
        let css_url = format!(
            "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/{HLJS_VERSION}/styles/{HLJS_THEME}.min.css"
        );
        // Create or update the highlight.js theme stylesheet and highlight all code blocks
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
