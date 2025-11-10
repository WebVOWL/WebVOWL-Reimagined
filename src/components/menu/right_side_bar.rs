use crate::components::buttons::graph_interaction_buttons::GraphInteractionButtons;
use leptos::prelude::*;
use thaw::*;

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
        <ConfigProvider>
            <p class="sidebar-section">
                "Language(s):"
                <Select class="language-button">
                    {move || {
                        ontologylanguages
                            .get()
                            .into_iter()
                            .map(|lang| view! { <option>{lang}</option> })
                            .collect_view()
                    }}
                </Select>
            </p>
        </ConfigProvider>
    }
}

#[component]
pub fn Description() -> impl IntoView {
    let ontologydescription = RwSignal::new("The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language.".to_string());
    view! {
        <ConfigProvider>
            <Accordion class="accordion" collapsible=true multiple=true>
                <AccordionItem value="description">
                    <AccordionHeader slot>
                        <p class="accordion-header">"Description"</p>
                    </AccordionHeader>
                    <p class="accordion-section-content">
                        {move || ontologydescription.get()}
                    </p>
                </AccordionItem>
            </Accordion>
        </ConfigProvider>
    }
}

#[component]
pub fn MetaData() -> impl IntoView {
    let metadata = RwSignal::new("The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language.The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language.The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language.The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language.The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language.The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language.The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language.The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language.The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language.".to_string());
    view! {
        <ConfigProvider>
            <Accordion class="accordion" collapsible=true multiple=true>
                <AccordionItem value="metadata">
                    <AccordionHeader slot>
                        <p class="accordion-header">"Metadata"</p>
                    </AccordionHeader>
                    <p class="accordion-section-content">
                        {move || metadata.get()}
                    </p>
                </AccordionItem>
            </Accordion>
        </ConfigProvider>
    }
}

#[component]
pub fn SelectionDetails() -> impl IntoView {
    view! {
        <ConfigProvider>
            <Accordion class="accordion" collapsible=true multiple=true>
                <AccordionItem value="selection details">
                    <AccordionHeader slot>
                        <p class="accordion-header">"Selection Details"</p>
                    </AccordionHeader>
                    <p class="accordion-section-content">
                        "Select an element in the visualization."
                    </p>
                </AccordionItem>
            </Accordion>
        </ConfigProvider>
    }
}

#[component]
pub fn ToggleRightSidebarButton() -> impl IntoView {
    let open = use_context::<RwSignal<bool>>().unwrap();

    view! {
        <button
            class="toggle-sidebar-btn"
            class=(
                "toggle-sidebar-btn-collapsed",
                move || *open.read() == false,
            )
            on:click=move |_| {
                open.update(|value| *value = !*value);
            }
        >
            {move || if *open.read() { ">" } else { "<" }}
        </button>
    }
}

#[component]
pub fn RightSidebar() -> impl IntoView {
    let open = RwSignal::new(true);
    provide_context(open);

    view! {
        <ToggleRightSidebarButton />
        <div
            class:sidebar
            class=("sidebar-collapse", move || *open.read() == false)
            class=("sidebar-expand", move || *open.read())
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
        <GraphInteractionButtons />
    }
}
