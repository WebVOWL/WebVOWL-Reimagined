use leptos::prelude::*;
use leptos_meta::*;
use crate::components::menu::side_bar::*;
use crate::components::buttons::work_bench_buttons::*;

#[component]
pub fn Home() -> impl IntoView {
    let ontologytitle = RwSignal::new("Friend of a Friend (FOAF) vocabulary".to_string());
    provide_context(ontologytitle);
    let displayed_title = move || ontologytitle.get();
    let sidebar_open = RwSignal::new(true);
    let is_first_load = RwSignal::new(false);

    view! {
        <Title text="Leptos + Tailwindcss" />
        <main>
            <div class="min-h-screen bg-[rgba(201, 196, 196, 1)]">
                <button
                    class={move || {
                        if sidebar_open.get() {
                            "toggle-sidebar-btn"
                        } else {
                            "toggle-sidebar-btn toggle-sidebar-btn-collapsed"
                        }
                    }}
                    on:click=move |_| {
                        sidebar_open.update(|open| *open = !*open);
                        if !is_first_load.get() {
                            is_first_load.set(true);
                        }
                    }
                >
                    {move || if sidebar_open.get() { ">" } else { "<" }}
                </button>
                <div class="work-bench">
                    <OntologyButton />
                    <SearchButton />
                    <FilterButton />
                    <ExportButton />
                    <OptionsButton />
                    <AboutButton />
                </div>
                <div class={move || {
                    if is_first_load.get() {
                        if sidebar_open.get() {
                            "sidebar sidebar-expand"
                        } else {
                            "sidebar sidebar-collapse sidebar-collapsed"
                        }
                    } else {
                        if sidebar_open.get() {
                            "sidebar"
                        } else {
                            "sidebar sidebar-collapsed"
                        }
                    }
                }}>
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
            </div>
        </main>
    }
}
