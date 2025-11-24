use crate::components::buttons::graph_interaction_buttons::GraphInteractionButtons;
use leptos::prelude::*;

#[component]
pub fn Accordion(#[prop(into)] title: String, children: Children) -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    view! {
        <div class="border-b border-gray-200">
            <button
                class="flex justify-between items-center py-3 px-4 w-full font-medium text-left text-gray-700 transition-colors hover:bg-gray-50"
                on:click=move |_| set_is_open.update(|v| *v = !*v)
            >
                <span>{title.clone()}</span>
                <span
                    class="text-gray-500 transition-transform"
                    class=("rotate-180", move || is_open.get())
                >
                    "▼"
                </span>
            </button>
            <div
                class="overflow-hidden transition-all duration-300"
                style=move || {
                    if is_open.get() {
                        "max-height: 1000px; opacity: 1;"
                    } else {
                        "max-height: 0px; opacity: 0;"
                    }
                }
            >
                <div class="py-3 px-4 text-gray-700 bg-white">{children()}</div>
            </div>
        </div>
    }
}

#[component]
pub fn OntologyIri() -> impl IntoView {
    let ontologyiri = RwSignal::new("http://xmlns.com/foaf/0.1/".to_string());
    view! {
        <p class="sidebar-section">
            <a
                href=move || ontologyiri.get()
                target="_blank"
                class="ontology-link"
            >
                {move || ontologyiri.get()}
            </a>
        </p>
    }
}

#[component]
pub fn Version() -> impl IntoView {
    let ontologyversion = RwSignal::new("0.99".to_string());
    view! {
        <p class="sidebar-section">
            "Version: "{move || ontologyversion.get()}
        </p>
    }
}

#[component]
pub fn Author() -> impl IntoView {
    let ontologyauthors = RwSignal::new("Alice, Bob, Charlie".to_string());
    view! {
        <p class="sidebar-section">
            Author(s): {move || ontologyauthors.get()}
        </p>
    }
}

#[component]
pub fn Language() -> impl IntoView {
    let ontologylanguages = RwSignal::new(vec![
        "english".to_string(),
        "german".to_string(),
        "french".to_string(),
    ]);
    view! {
        <p class="sidebar-section">
            "Language(s):"
            <select class="py-2 px-3 mt-2 text-sm rounded-md border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none">
                {move || {
                    ontologylanguages
                        .get()
                        .into_iter()
                        .map(|lang| view! { <option>{lang}</option> })
                        .collect_view()
                }}
            </select>
        </p>
    }
}

#[component]
pub fn Description() -> impl IntoView {
    let ontologydescription = RwSignal::new("The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language.".to_string());
    view! {
        <Accordion title="Description">
            <p>{move || ontologydescription.get()}</p>
        </Accordion>
    }
}

#[component]
pub fn MetaData() -> impl IntoView {
    let metadata = RwSignal::new("The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language.The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language.The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language.The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language.The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language.The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language.The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language.The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language.The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language.".to_string());
    view! {
        <Accordion title="Metadata">
            <p>{move || metadata.get()}</p>
        </Accordion>
    }
}

#[component]
pub fn SelectionDetails() -> impl IntoView {
    let selection_details = RwSignal::new("Select an element in the visualization.".to_string());
    view! {
        <Accordion title="Selection Details">
            <p>{move || selection_details.get()}</p>
        </Accordion>
    }
}

#[component]
pub fn ToggleRightSidebarButton() -> impl IntoView {
    let is_open = RwSignal::new(true);
    view! {
        <div data-sidebar-open=move || is_open.get().to_string()>
            <button
                class="toggle-sidebar-btn"
                class=("toggle-sidebar-btn-collapsed", move || !is_open.get())
                on:click=move |_| {
                    is_open.update(|value| *value = !*value);
                }
            >
                {move || if is_open.get() { ">" } else { "<" }}
            </button>
            <div
                class="sidebar"
                class=("sidebar-collapse", move || !is_open.get())
                class=("sidebar-expand", move || is_open.get())
            >
                <div class="sidebar-content">
                    <p class="ontology-title">
                        "Friend of a Friend (FOAF) vocabulary"
                    </p>
                    <OntologyIri />
                    <Version />
                    <Author />
                    <Language />
                    <Description />
                    <MetaData />
                    <SelectionDetails />
                </div>
            </div>
            <GraphInteractionButtons is_sidebar_open=is_open />
        </div>
    }
}

#[component]
pub fn RightSidebar() -> impl IntoView {
    view! { <ToggleRightSidebarButton /> }
}
