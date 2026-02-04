use storybook::StorybookConfig;

fn main() {
    storybook::launch(StorybookConfig::default()
        .with_title("Cadence UI")
        .with_css(cadence_ui::UI_CSS)
    );
}






