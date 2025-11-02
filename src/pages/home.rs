// use crate::components::buttons::work_bench_buttons::*;
// use crate::components::menu::about_menu::*;
// use crate::components::menu::export_menu::*;
// use crate::components::menu::filter_menu::*;
// use crate::components::menu::ontology_menu::*;
// use crate::components::menu::options_menu::*;
use crate::components::buttons::graph_interaction_buttons::GraphInteractionButtons;
use crate::components::menu::right_side_bar::RightSidebar;
// use crate::components::menu::search_menu::*;
use crate::signals::menu_signals::SidebarOpen;
use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn Home() -> impl IntoView {
    // let show_ontology_menu = ShowOntologyMenu(RwSignal::new(false));
    // let show_search_menu = ShowSearchMenu(RwSignal::new(false));
    // let show_filter_menu = ShowFilterMenu(RwSignal::new(false));
    // let show_export_menu = ShowExportMenu(RwSignal::new(false));
    // let show_options_menu = ShowOptionsMenu(RwSignal::new(false));
    // let show_about_menu = ShowAboutMenu(RwSignal::new(false));
    let sidebar_open = SidebarOpen(RwSignal::new(true));

    // provide_context(show_ontology_menu);
    // provide_context(show_search_menu);
    // provide_context(show_filter_menu);
    // provide_context(show_export_menu);
    // provide_context(show_options_menu);
    // provide_context(show_about_menu);
    // provide_context(ontologytitle);
    provide_context(sidebar_open);

    view! {
        <Title text="Leptos + Tailwindcss" />
        <main>
            <canvas id="canvas" />
            <div class="min-h-screen bg-[rgba(201, 196, 196, 1)]">
                <RightSidebar />
                <GraphInteractionButtons />
            </div>
        </main>
    }
}
