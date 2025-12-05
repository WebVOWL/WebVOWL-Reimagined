mod about_menu;
mod export_menu;
// mod filter_menu;
mod ontology_menu;
mod options_menu;
// mod search_menu;

use crate::components::lists::{ListDetails, ListElement};
use crate::components::menu::vertical_menu::VerticalMenu;
use about_menu::AboutMenu;
use export_menu::ExportMenu;
// use filter_menu::FilterMenu;
use leptos::prelude::*;
use ontology_menu::OntologyMenu;
use options_menu::OptionsMenu;
// use search_menu::SearchMenu;

#[component]
fn WorkbenchMenuItems(#[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col justify-center p-2 w-[250px]">
            <div class="workbench-menu-header">
                <h3>{title}</h3>
            </div>
            <div>{children()}</div>
        </div>
    }
}

#[component]
pub fn NewWorkbench() -> impl IntoView {
    view! {
        <VerticalMenu>
            <ListElement title="Load Ontology" icon=icondata::BiMenuRegular>
                <OntologyMenu />
            </ListElement>

            // <ListElement title="Search" icon=icondata::BiMenuRegular>
            //     <SearchMenu />
            // </ListElement>

            // <ListElement title="Filter" icon=icondata::BiMenuRegular>
            //     <FilterMenu />
            // </ListElement>

            <ListElement title="Export" icon=icondata::BiMenuRegular>
                <ExportMenu />
            </ListElement>

            <ListDetails title="Settings" icon=icondata::IoSettingsOutline>
                <ListElement title="Simulator">
                    <OptionsMenu />
                </ListElement>
            </ListDetails>

            <ListElement title="About" icon=icondata::BiMenuRegular>
                <AboutMenu />
            </ListElement>
        </VerticalMenu>
    }
}
