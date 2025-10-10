use leptos::prelude::*;
use thaw::*;

#[component]
pub fn OntologyButton() -> impl IntoView {
    view! {
        <ConfigProvider>
            <Button
                class="work-bench-button"
                shape=ButtonShape::Square
                icon=icondata::BiMenuRegular
            ></Button>
        </ConfigProvider>
    }
}

pub fn SearchButton() -> impl IntoView {
    view! {
        <ConfigProvider>
            <Button
                class="work-bench-button"
                shape=ButtonShape::Square
                icon=icondata::AiSearchOutlined
            ></Button>
        </ConfigProvider>
    }
}

pub fn FilterButton() -> impl IntoView {
    view! {
        <ConfigProvider>
            <Button
                class="work-bench-button"
                shape=ButtonShape::Square
                icon=icondata::BiFilterAltRegular
            ></Button>
        </ConfigProvider>
    }
}

pub fn ExportButton() -> impl IntoView {
    view! {
        <ConfigProvider>
            <Button
                class="work-bench-button"
                shape=ButtonShape::Square
                icon=icondata::BiExportRegular
            ></Button>
        </ConfigProvider>
    }
}

pub fn OptionsButton() -> impl IntoView {
    view! {
        <ConfigProvider>
            <Button
                class="work-bench-button-bottom-1"
                shape=ButtonShape::Square
                icon=icondata::IoOptions
            ></Button>
        </ConfigProvider>
    }
}

pub fn AboutButton() -> impl IntoView {
    view! {
        <ConfigProvider>
            <Button
                class="work-bench-button-bottom-2"
                shape=ButtonShape::Square
                icon=icondata::AiCopyrightOutlined
            ></Button>
        </ConfigProvider>
    }
}
