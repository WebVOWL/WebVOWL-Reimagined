use leptos::prelude::*;
use thaw::*;

#[component]
pub fn OntologyIri() -> impl IntoView{
    let ontologyiri = RwSignal::new("http://xmlns.com/foaf/0.1/".to_string());
    view! {
        <p class="info-section">
            <a 
                href={move || ontologyiri.get()} 
                target="_blank" 
                class="ontology-link"
            >
                {move || ontologyiri.get()}
            </a>
        </p>
    }
}

#[component]
pub fn Version() -> impl IntoView{
    let ontologyversion = RwSignal::new("0.99".to_string());
    view! {
        <p class="info-section">"Version: "{move || ontologyversion.get()}</p>
    }
}

#[component]
pub fn Author() -> impl IntoView{
    let ontologyauthors = RwSignal::new("Alice, Bob, Charlie".to_string());
    view! {
        <ConfigProvider>
            <p class="info-section"><Caption1Strong>Author(s): {move || ontologyauthors.get()}</Caption1Strong></p>
        </ConfigProvider>
    }
}

#[component]
pub fn Language() -> impl IntoView{
    let ontologylanguages = RwSignal::new(vec!["english".to_string(), "german".to_string(), "french".to_string()]);
    view! {
        <ConfigProvider>
            <p class="info-section">
                <Caption1Strong>Language(s): </Caption1Strong>
                <Select class="language-button">
                    {move || ontologylanguages.get().into_iter().map(|lang| view! {
                        <option>{lang}</option> 
                    }).collect_view()}
                </Select>
            </p>
        </ConfigProvider>
    }
}


#[component]
pub fn Description() -> impl IntoView{
    let ontologydescription = RwSignal::new("The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language.".to_string());
    view! {
        <ConfigProvider>
                    <div class="accordion-header">
                        <Accordion collapsible=true multiple=true>
                            <AccordionItem value="description">
                                <AccordionHeader slot>
                                    "Description"
                                </AccordionHeader>
                                <p class="info-section-content">{move || ontologydescription.get()}</p>
                            </AccordionItem>
                        </Accordion>
                        </div>
        </ConfigProvider>
    }
}


#[component]
pub fn MetaData() -> impl IntoView{
    let metadata = RwSignal::new("The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language.".to_string());
    view! {
        <ConfigProvider>
            <Accordion collapsible=true multiple=true>
                <AccordionItem value="setadata">
                        <AccordionHeader slot>
                            "Metadata"
                        </AccordionHeader>
                        <p class="info-section-content">{move || metadata.get()}</p>
                </AccordionItem>
            </Accordion>
        </ConfigProvider>
    }
}

#[component]
pub fn Statistics() -> impl IntoView{
    let classcount = RwSignal::new(20);
    let objectpropertycount = RwSignal::new(0);
    let datatypepropertycount = RwSignal::new(0);
    let individualcount = RwSignal::new(19);
    let nodecount = RwSignal::new(0);
    let edgecount = RwSignal::new(0);
    view! {
        <ConfigProvider>
        <Accordion collapsible=true multiple=true>
            <AccordionItem value="statistics">
                <AccordionHeader slot>
                    "Statistics"
                </AccordionHeader>
                    <p class="info-section-content">
                    "Classes: "{move || classcount.get()}
                    "Object Properties: " {move || objectpropertycount.get()}
                    "Datatype Properties: " {move || datatypepropertycount.get()}
                    "Individuals: " {move || individualcount.get()}
                    "Nodes: " {move || nodecount.get()}
                    "Edges: " {move || edgecount.get()}
                    </p>
            </AccordionItem>
        </Accordion>
        </ConfigProvider>
    }
}


#[component]
pub fn SelectionDetails() -> impl IntoView{
    view! {
        <ConfigProvider>
            <Accordion collapsible=true multiple=true>
                <AccordionItem value="selection details">
                    <AccordionHeader slot>
                        "Selection Details"
                    </AccordionHeader>
                    <p class="info-section-content">"Select an element in the visualization."</p>
                </AccordionItem>
            </Accordion>
        </ConfigProvider>
    }
}