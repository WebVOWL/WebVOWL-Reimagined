use crate::components::buttons::work_bench_buttons::*;
use crate::components::menu::ontology_menu::*;
use crate::components::menu::search_menu::*;
use crate::components::menu::filter_menu::*;
use crate::components::menu::export_menu::*;
use crate::components::menu::options_menu::*;
use crate::components::menu::about_menu::*;
use crate::components::menu::side_bar::*;
use crate::components::zoom_slider::*;
use crate::components::buttons::center_graph_button::*;
use crate::components::buttons::zoom_buttons::*;
use leptos::prelude::*;
use leptos_meta::*;

#[derive(Clone, Copy)]
pub struct ShowOntologyMenu(pub RwSignal<bool>);

#[derive(Clone, Copy)]  
pub struct ShowSearchMenu(pub RwSignal<bool>);

#[derive(Clone, Copy)]
pub struct ShowFilterMenu(pub RwSignal<bool>);

#[derive(Clone, Copy)]
pub struct ShowExportMenu(pub RwSignal<bool>);

#[derive(Clone, Copy)]
pub struct ShowOptionsMenu(pub RwSignal<bool>);

#[derive(Clone, Copy)]
pub struct ShowAboutMenu(pub RwSignal<bool>);

#[derive(Clone, Copy)]
pub struct SidebarOpen(pub RwSignal<bool>);

#[derive(Clone, Copy)]
pub struct IsFirstLoad(pub RwSignal<bool>);

#[component]
pub fn Home() -> impl IntoView {
    let ontologytitle = RwSignal::new("Friend of a Friend (FOAF) vocabulary".to_string());
    let show_ontology_menu = ShowOntologyMenu(RwSignal::new(false));
    let show_search_menu = ShowSearchMenu(RwSignal::new(false));
    let show_filter_menu = ShowFilterMenu(RwSignal::new(false));
    let show_export_menu = ShowExportMenu(RwSignal::new(false));
    let show_options_menu = ShowOptionsMenu(RwSignal::new(false));
    let show_about_menu = ShowAboutMenu(RwSignal::new(false));
    let displayed_title = move || ontologytitle.get();
    let sidebar_open = SidebarOpen(RwSignal::new(true));
    let is_first_load = IsFirstLoad(RwSignal::new(false));
    provide_context(show_ontology_menu);
    provide_context(show_search_menu);
    provide_context(show_filter_menu);
    provide_context(show_export_menu);
    provide_context(show_options_menu);
    provide_context(show_about_menu);
    provide_context(ontologytitle);
    provide_context(sidebar_open);
    provide_context(is_first_load);

    view! {
        <Title text="Leptos + Tailwindcss" />
        <main>
            <canvas id="canvas"/>
            <div class="min-h-screen bg-[rgba(201, 196, 196, 1)]">
                <button
                    class=move || {
                        if sidebar_open.0.get() {
                            "toggle-sidebar-btn"
                        } else {
                            "toggle-sidebar-btn toggle-sidebar-btn-collapsed"
                        }
                    }
                    on:click=move |_| {
                        sidebar_open.0.update(|open| *open = !*open);
                        if !is_first_load.0.get() {
                            is_first_load.0.set(true);
                        }
                    }
                >
                    {move || if sidebar_open.0.get() { ">" } else { "<" }}
                </button>
                <div class="work-bench">
                    <OntologyButton />
                    <SearchButton />
                    <FilterButton />
                    <ExportButton />
                    <OptionsButton />
                    <AboutButton />
                </div>            
                <OntologyMenu />
                <SearchMenu />
                <FilterMenu />
                <ExportMenu />
                <OptionsMenu />
                <AboutMenu />
                <div class=move || {
                    if is_first_load.0.get() {
                        if sidebar_open.0.get() {
                            "sidebar sidebar-expand"
                        } else {
                            "sidebar sidebar-collapse sidebar-collapsed"
                        }
                    } else {
                        if sidebar_open.0.get() {
                            "sidebar"
                        } else {
                            "sidebar sidebar-collapsed"
                        }
                    }   
                }>
                    <div class="sidebar-content">
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
                <CenterGraphButton />
                <ZoomInButton />
                <ZoomOutButton />
            </div>
        </main>
    }
}
