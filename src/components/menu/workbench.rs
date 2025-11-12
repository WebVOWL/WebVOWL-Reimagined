mod about_menu;
// mod export_menu;
// mod filter_menu;
// mod modes_menu;
mod ontology_menu;
mod options_menu;
// mod search_menu;

use about_menu::AboutMenu;
use leptos::prelude::*;
use thaw::*;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};
// use export_menu::ExportMenu;
// use filter_menu::FilterMenu;
// use leptos::prelude::*;
// use modes_menu::ModesMenu;
use crate::components::theme::ThemeSelection;
use ontology_menu::OntologyMenu;
use options_menu::OptionsMenu;
// use search_menu::SearchMenu;

use crate::components::lists::{ListChild, ListDetails, ListElement};
use crate::components::progress_bar::ProgressBar;
use crate::network::handle_local;

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

#[component]
pub fn Upload() -> impl IntoView {
    let upload_action = Action::new_local(|data: &FormData| handle_local(data.clone().into()));

    view! {
        <form on:submit=move |ev: SubmitEvent| {
            ev.prevent_default();
            let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
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
    view! {
    <div class="flex h-screen flex-col justify-between w-fit border-e border-gray-100 bg-white">
      <div class="px-4 py-6">
        <ul class="mt-6 space-y-1">
            <ListElement
                title="Load Ontology"
                icon=icondata::BiMenuRegular>
            </ListElement>

            <ListElement
                title="Search"
                icon=icondata::BiMenuRegular>
            </ListElement>

            <ListElement
                title="Filter"
                icon=icondata::BiMenuRegular>
            </ListElement>

            <ListElement
                title="Export"
                icon=icondata::BiMenuRegular>
            </ListElement>

            <ListDetails
                title="Settings"
                icon=icondata::IoSettingsOutline
                >
                <ListChild title="Simulator"></ListChild>
                <>


            </ListDetails>

            <ListElement
                title="About"
                icon=icondata::BiMenuRegular>
            </ListElement>
        </ul>
      </div>
    </div>
        }
}
