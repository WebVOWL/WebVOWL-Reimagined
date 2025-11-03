// mod about_menu;
// mod export_menu;
// mod filter_menu;
// mod modes_menu;
mod ontology_menu;
// mod options_menu;
// mod search_menu;

// use about_menu::AboutMenu;
// use export_menu::ExportMenu;
// use filter_menu::FilterMenu;
// use leptos::prelude::*;
// use modes_menu::ModesMenu;
use ontology_menu::OntologyMenu;
// use options_menu::OptionsMenu;
// use search_menu::SearchMenu;
use leptos::prelude::*;
use thaw::{Button, ConfigProvider, Popover, PopoverPosition, PopoverTrigger, PopoverTriggerType};

#[component]
pub fn Workbench() -> impl IntoView {
    view! {
        <div class="work-bench">
            <OntologyMenu />
        </div>
    }
}
