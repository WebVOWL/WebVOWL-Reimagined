use leptos::prelude::*;
use leptos_meta::*;
use crate::components::menu::ontology_menu::OntologyMenu;
use crate::components::buttons::search::SearchButton;
use crate::components::buttons::locate_button::LocateButton;
use crate::components::menu::export_menu::ExportMenu;
use crate::components::menu::filter_menu::FilterMenu;
use crate::components::menu::options_menu::OptionsMenu;
use crate::components::menu::modes_menu::ModesMenu;
use crate::components::buttons::reset_button::ResetButton;
use crate::components::buttons::pause_button::PauseButton;
use crate::components::menu::about_menu::AboutMenu;
use crate::components::menu::side_bar::*;


#[component]
pub fn Home() -> impl IntoView {
    let ontologytitle = RwSignal::new("Friend of a Friend (FOAF) vocabulary".to_string());
    provide_context(ontologytitle);
    let displayed_title = move || ontologytitle.get();
    
    view! {
        <Title text="Leptos + Tailwindcss" />
            <main>
                <div class="min-h-screen bg-[rgba(201, 196, 196, 1)]">
                    <div class="bottom-bar">
                        <SearchButton />
                        <LocateButton />
                        <OntologyMenu />
                        <ExportMenu />
                        <FilterMenu />
                        <OptionsMenu />
                        <ModesMenu />
                        <ResetButton />
                        <PauseButton />
                        <AboutMenu />
                    </div>
                    <div class="sidebar">
                        <p class="ontology-title">{displayed_title}</p>
                        <OntologyIri />
                        <Version />
                        <Author />
                        <Language />
                        <Description />
                        <MetaData />
                        <Statistics />
                        <SelectionDetails />
                    </div>
                </div>
            </main>
    }
}