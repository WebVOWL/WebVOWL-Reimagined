use leptos::prelude::*;
use thaw::*;
use leptos::logging::log;
use crate::pages::home::*;

#[component]
pub fn OntologyButton() -> impl IntoView {
    let ShowSearchMenu(show_search_menu) = use_context::<ShowSearchMenu>().expect("ShowSearchMenu should be provided");
    let ShowOntologyMenu(show_ontology_menu) = use_context::<ShowOntologyMenu>().expect("ShowOntologyMenu should be provided");
    view! {
        <ConfigProvider>
            <Button
                class="work-bench-button"
                shape=ButtonShape::Square
                icon=icondata::BiMenuRegular
                on_click=move |_| show_ontology_menu.update(|val| *val = !*val)
            ></Button>
        </ConfigProvider>
    }
}

pub fn SearchButton() -> impl IntoView {
    let ShowSearchMenu(show_search_menu) = use_context::<ShowSearchMenu>().expect("ShowSearchMenu should be provided");
    let ShowOntologyMenu(show_ontology_menu) = use_context::<ShowOntologyMenu>().expect("ShowOntologyMenu should be provided");
    view! {
        <ConfigProvider>
            <Button
                class="work-bench-button"
                shape=ButtonShape::Square
                icon=icondata::AiSearchOutlined
                on_click=move |_| show_search_menu.update(|val| *val = !*val)
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
