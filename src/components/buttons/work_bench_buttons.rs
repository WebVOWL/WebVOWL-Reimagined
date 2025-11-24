// use leptos::prelude::*;
// use thaw::{Button, ButtonAppearance, ButtonShape, ConfigProvider};

// #[component]
// pub fn SearchButton() -> impl IntoView {
//     let ShowSearchMenu(show_search_menu) = use_context::<ShowSearchMenu>().expect("ShowSearchMenu should be provided");
//     let ShowOntologyMenu(show_ontology_menu) = use_context::<ShowOntologyMenu>().expect("ShowOntologyMenu should be provided");
//     let ShowFilterMenu(show_filter_menu) = use_context::<ShowFilterMenu>().expect("ShowFilterMenu should be provided");
//     let ShowExportMenu(show_export_menu) = use_context::<ShowExportMenu>().expect("ShowExportMenu should be provided");
//     let ShowOptionsMenu(show_options_menu) = use_context::<ShowOptionsMenu>().expect("ShowOptionsMenu should be provided");
//     let ShowAboutMenu(show_about_menu) = use_context::<ShowAboutMenu>().expect("ShowAboutMenu should be provided");
//     view! {
//         <ConfigProvider>
//             <Button
//                 class="work-bench-button"
//                 shape=ButtonShape::Square
//                 icon=icondata::AiSearchOutlined
//                 on_click=move |_| {
//                     let current_search_state = show_search_menu.get();
//                     show_ontology_menu.update(|val| *val = false);
//                     show_filter_menu.update(|val| *val = false);
//                     show_export_menu.update(|val| *val = false);
//                     show_options_menu.update(|val| *val = false);
//                     show_about_menu.update(|val| *val = false);
//                     show_search_menu.update(|val| *val = !current_search_state);
//                 }
//             ></Button>
//         </ConfigProvider>
//     }
// }

// #[component]
// pub fn FilterButton() -> impl IntoView {
//     let ShowSearchMenu(show_search_menu) = use_context::<ShowSearchMenu>().expect("ShowSearchMenu should be provided");
//     let ShowOntologyMenu(show_ontology_menu) = use_context::<ShowOntologyMenu>().expect("ShowOntologyMenu should be provided");
//     let ShowFilterMenu(show_filter_menu) = use_context::<ShowFilterMenu>().expect("ShowFilterMenu should be provided");
//     let ShowExportMenu(show_export_menu) = use_context::<ShowExportMenu>().expect("ShowExportMenu should be provided");
//     let ShowOptionsMenu(show_options_menu) = use_context::<ShowOptionsMenu>().expect("ShowOptionsMenu should be provided");
//     let ShowAboutMenu(show_about_menu) = use_context::<ShowAboutMenu>().expect("ShowAboutMenu should be provided");
//     view! {
//         <ConfigProvider>
//             <Button
//                 class="work-bench-button"
//                 shape=ButtonShape::Square
//                 icon=icondata::BiFilterAltRegular
//                 on_click=move |_| {
//                     let current_filter_state = show_filter_menu.get();
//                     show_ontology_menu.update(|val| *val = false);
//                     show_search_menu.update(|val| *val = false);
//                     show_export_menu.update(|val| *val = false);
//                     show_options_menu.update(|val| *val = false);
//                     show_about_menu.update(|val| *val = false);
//                     show_filter_menu.update(|val| *val = !current_filter_state);
//                 }

//             ></Button>
//         </ConfigProvider>
//     }
// }

// #[component]
// pub fn ExportButton() -> impl IntoView {
//     let ShowSearchMenu(show_search_menu) = use_context::<ShowSearchMenu>().expect("ShowSearchMenu should be provided");
//     let ShowOntologyMenu(show_ontology_menu) = use_context::<ShowOntologyMenu>().expect("ShowOntologyMenu should be provided");
//     let ShowFilterMenu(show_filter_menu) = use_context::<ShowFilterMenu>().expect("ShowFilterMenu should be provided");
//     let ShowExportMenu(show_export_menu) = use_context::<ShowExportMenu>().expect("ShowExportMenu should be provided");
//     let ShowOptionsMenu(show_options_menu) = use_context::<ShowOptionsMenu>().expect("ShowOptionsMenu should be provided");
//     let ShowAboutMenu(show_about_menu) = use_context::<ShowAboutMenu>().expect("ShowAboutMenu should be provided");
//     view! {
//         <ConfigProvider>
//             <Button
//                 class="work-bench-button"
//                 shape=ButtonShape::Square
//                 icon=icondata::BiExportRegular
//                 on_click=move |_| {
//                     let current_export_state = show_export_menu.get();
//                     show_ontology_menu.update(|val| *val = false);
//                     show_search_menu.update(|val| *val = false);
//                     show_filter_menu.update(|val| *val = false);
//                     show_options_menu.update(|val| *val = false);
//                     show_about_menu.update(|val| *val = false);
//                     show_export_menu.update(|val| *val = !current_export_state);
//                 }
//             ></Button>
//         </ConfigProvider>
//     }
// }

// #[component]
// pub fn PauseButton() -> impl IntoView {
//     view! {
//         <ConfigProvider>
//             <Button class="work-bench-button"
//             shape=ButtonShape::Square
//             icon=icondata::AiPauseOutlined></Button>
//         </ConfigProvider>
//     }
// }

// #[component]
// pub fn ResetButton() -> impl IntoView {
//     view! {
//         <ConfigProvider>
//             <Button class="work-bench-button" shape=ButtonShape::Square icon=icondata::VsDebugRestart></Button>
//         </ConfigProvider>
//     }
// }

// #[component]
// pub fn OptionsButton() -> impl IntoView {
//     let ShowSearchMenu(show_search_menu) = use_context::<ShowSearchMenu>().expect("ShowSearchMenu should be provided");
//     let ShowOntologyMenu(show_ontology_menu) = use_context::<ShowOntologyMenu>().expect("ShowOntologyMenu should be provided");
//     let ShowFilterMenu(show_filter_menu) = use_context::<ShowFilterMenu>().expect("ShowFilterMenu should be provided");
//     let ShowExportMenu(show_export_menu) = use_context::<ShowExportMenu>().expect("ShowExportMenu should be provided");
//     let ShowOptionsMenu(show_options_menu) = use_context::<ShowOptionsMenu>().expect("ShowOptionsMenu should be provided");
//     let ShowAboutMenu(show_about_menu) = use_context::<ShowAboutMenu>().expect("ShowAboutMenu should be provided");
//     view! {
//         <ConfigProvider>
//             <Button
//                 class="work-bench-button-bottom-1"
//                 shape=ButtonShape::Square
//                 icon=icondata::IoOptions
//                 on_click=move |_| {
//                     let current_options_state = show_options_menu.get();
//                     show_ontology_menu.update(|val| *val = false);
//                     show_search_menu.update(|val| *val = false);
//                     show_filter_menu.update(|val| *val = false);
//                     show_export_menu.update(|val| *val = false);
//                     show_about_menu.update(|val| *val = false);
//                     show_options_menu.update(|val| *val = !current_options_state);
//                 }
//             ></Button>
//         </ConfigProvider>
//     }
// }
