use leptos::prelude::*;
use std::collections::HashMap;
use thaw::{Button, Theme};

pub fn ThemeSelection() -> impl IntoView {
    // WebVOWL brand colors
    // Designed with: https://storybooks.fluentui.dev/react/?path=/docs/theme-theme-designer--docs
    let brand_colors = RwSignal::new(HashMap::from([
        (10, "#020304"),
        (20, "#12171E"),
        (30, "#1E2630"),
        (40, "#2A313B"),
        (50, "#373D47"),
        (60, "#434A53"),
        (70, "#51575F"),
        (80, "#5E646C"),
        (90, "#6C7179"),
        (100, "#7B7F86"),
        (110, "#898D93"),
        (120, "#989BA1"),
        (130, "#A7AAAF"),
        (140, "#B6B9BD"),
        (150, "#C6C8CB"),
        (160, "#D5D7D9"),
    ]));

    let theme = Theme::use_rw_theme();
    let theme_name = Memo::new(move |_| {
        theme.with(|theme| {
            if theme.name == *"light" {
                "Dark".to_string()
            } else {
                "Light".to_string()
            }
        })
    });
    let change_theme = move |_| {
        if theme_name.get_untracked() == "Light" {
            theme.set(Theme::custom_light(&brand_colors.get()));
        } else {
            theme.set(Theme::custom_dark(&brand_colors.get()));
        }
    };

    view! {
        // <ConfigProvider theme>
        <Button
            icon=Memo::new(move |_| {
                theme
                    .with(|theme| {
                        if theme.name == "light" {
                            icondata::BiMoonRegular
                        } else {
                            icondata::BiSunRegular
                        }
                    })
            })
            on_click=change_theme
        >
            {move || theme_name.get()}
        </Button>
    }
}
