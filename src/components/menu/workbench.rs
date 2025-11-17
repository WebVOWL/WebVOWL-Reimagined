mod about_menu;
mod export_menu;
//mod filter_menu;
// mod modes_menu;
mod ontology_menu;
mod options_menu;
mod search_menu;
use about_menu::AboutMenu;
use export_menu::ExportMenu;
//use filter_menu::FilterMenu;
use leptos::prelude::*;
use thaw::*;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};
// use leptos::prelude::*;
// use modes_menu::ModesMenu;
use crate::components::theme::ThemeSelection;
use ontology_menu::OntologyMenu;
use options_menu::OptionsMenu;
use search_menu::SearchMenu;
use crate::components::lists::{ListChild, ListDetails, ListElement};
use crate::components::progress_bar::ProgressBar;
#[cfg(feature = "server")]
use crate::network::handle_local;

#[derive(Debug, Clone, Copy, PartialEq)]
enum MenuType {
    Export,
    //Filter,
    Search,
}

#[component]
pub fn WorkBenchButton(
    #[prop(default=ButtonShape::Rounded)] shape: ButtonShape,
    icon: icondata::Icon,
    #[prop(into)] text: String,
) -> impl IntoView {
    view! {
        <Button shape=shape icon=icon>
            {text}
        </Button>
    }
}

#[component]
fn WorkbenchMenuItems(#[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <div class="workbench-menu-header">
            <h3>{title}</h3>
        </div>
        <Flex vertical=true gap=FlexGap::Size(30)>
            {children()}
        </Flex>
    }
}

#[component]
pub fn Workbench() -> impl IntoView {
    view! {
        <Flex
            class="workbench"
            align=FlexAlign::Center
            justify=FlexJustify::SpaceBetween
            vertical=true
        >
            <Flex vertical=true gap=FlexGap::Large>
                <OntologyMenu />
                <OntologyMenu />
            </Flex>
            <Flex
                style="padding-bottom: 2rem;"
                vertical=true
                gap=FlexGap::Large
            >
                <ThemeSelection />
                <OptionsMenu />
                <AboutMenu />
            </Flex>
        </Flex>
    }
}

#[cfg(feature = "server")]
#[component]
pub fn Upload() -> impl IntoView {
    let upload_action = Action::new_local(|data: &FormData| handle_local(data.clone().into()));

    view! {
        <form on:submit=move |ev: SubmitEvent| {
            ev.prevent_default();
            let target = ev
                .target()
                .unwrap()
                .unchecked_into::<HtmlFormElement>();
            let form_data = FormData::new_with_form(&target).unwrap();
            upload_action.dispatch_local(form_data);
        }>
            <input type="file" name="file_to_upload" />
            <input type="submit" />
        </form>
        <ProgressBar />
    }
}

#[component]
pub fn NewWorkbench() -> impl IntoView {
    let (active_menu, set_active_menu) = signal::<Option<MenuType>>(None);

    view! {
        <div class="flex fixed inset-0 z-50 pointer-events-none">
            {}
            <div class="flex flex-col flex-shrink-0 justify-between bg-white border-gray-100 pointer-events-auto w-fit border-e">
                <div class="py-6 px-4">
                    <ul class="mt-6 space-y-1">
                        <ListElement
                            title="Load Ontology"
                            icon=icondata::BiMenuRegular
                        ></ListElement>

                        <div on:click=move |_| {
                            set_active_menu
                                .set(
                                    if active_menu.get() == Some(MenuType::Search) {
                                        None
                                    } else {
                                        Some(MenuType::Search)
                                    },
                                )
                        }>
                            <ListElement
                                title="Search"
                                icon=icondata::BiMenuRegular
                            ></ListElement>
                        </div>

                        <div>
                            <ListElement
                                title="Filter"
                                icon=icondata::BiMenuRegular
                            ></ListElement>
                        </div>

                        <div on:click=move |_| {
                            set_active_menu
                                .set(
                                    if active_menu.get() == Some(MenuType::Export) {
                                        None
                                    } else {
                                        Some(MenuType::Export)
                                    },
                                )
                        }>
                            <ListElement
                                title="Export"
                                icon=icondata::BiMenuRegular
                            ></ListElement>
                        </div>

                        <ListDetails
                            title="Settings"
                            icon=icondata::IoSettingsOutline
                        >
                            <ListChild title="Simulator"></ListChild>

                        </ListDetails>

                        <ListElement
                            title="About"
                            icon=icondata::BiMenuRegular
                        ></ListElement>
                    </ul>
                </div>
            </div>
            <div
                class="overflow-y-auto flex-shrink-0 bg-white border-gray-100 shadow-lg transition-all duration-500 pointer-events-auto border-e"
                style=move || {
                    if active_menu.get().is_some() {
                        "opacity: 1; width: 320px;"
                    } else {
                        "opacity: 0; width: 0;"
                    }
                }
            >
                <div class="p-4">
                    <div style=move || {
                        if active_menu.get() == Some(MenuType::Export) {
                            "display: block;"
                        } else {
                            "display: none;"
                        }
                    }>
                        <ExportMenu />
                    </div>
                    // <div style=move || {
                    //     if active_menu.get() == Some(MenuType::Filter) {
                    //         "display: block;"
                    //     } else {
                    //         "display: none;"
                    //     }
                    // }>
                    //     <FilterMenu />
                    // </div>
                    <div style=move || {
                        if active_menu.get() == Some(MenuType::Search) {
                            "display: block;"
                        } else {
                            "display: none;"
                        }
                    }>
                        <SearchMenu />
                    </div>
                </div>
            </div>
        </div>
    }
}
