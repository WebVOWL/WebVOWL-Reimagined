mod about_menu;
mod export_menu;
// mod filter_menu;
// mod modes_menu;
mod ontology_menu;
mod options_menu;
// mod search_menu;

use crate::components::lists::{ListDetails, ListElement};
use crate::components::menu::{mega_menu::MegaMenu, vertical_menu::VerticalMenu};
use leptos::prelude::*;
use options_menu::OptionsMenu;
use about_menu::AboutMenu;
use ontology_menu::OntologyMenu;
use options_menu::OptionsMenu;
use search_menu::SearchMenu;

#[component]
fn WorkbenchMenuItems(#[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <div class="flex justify-center flex-col w-[250px] p-2">
            <div class="workbench-menu-header">
                <h3>{title}</h3>
            </div>
            <div>
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn NewWorkbench() -> impl IntoView {
    view! {
        <VerticalMenu>
            <ListElement
                title="Load Ontology"
                icon=icondata::BiMenuRegular
            >
                <OntologyMenu/>
            </ListElement>

            <ListElement title="Search" icon=icondata::BiMenuRegular>
                <button />
            </ListElement>

            <ListElement title="Filter" icon=icondata::BiMenuRegular>
                <button />
            </ListElement>

            <ListElement
                title="Export"
                icon=icondata::BiMenuRegular
            >
                <ExportMenu/>
            </ListElement>

            <ListDetails title="Settings" icon=icondata::IoSettingsOutline>
                <ListElement title="Simulator">
                    <OptionsMenu />
                </ListElement>
            </ListDetails>

            <ListElement
                title="About"
                icon=icondata::BiMenuRegular
            >
                <AboutMenu/>
            </ListElement>
        </VerticalMenu>
    }
}
