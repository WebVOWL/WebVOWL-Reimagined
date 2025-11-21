use super::WorkbenchMenuItems;
use leptos::prelude::*;
use web_sys::{HtmlInputElement, FileList};

#[component]
fn SelectStaticInput() -> impl IntoView {
    let selected_ontology = RwSignal::new("Friend of a Friend (FOAF) vocabulary".to_string());

    let ontologies = move || {
        vec![
            "Friend of a Friend (FOAF) vocabulary".to_string(),
            "GoodRelations Vocabulary for E-Commerce".to_string(),
            "Modular and Unified Tagging Ontology (MUTO)".to_string(),
            "Personas Ontology (PersonasOnto)".to_string(),
            "SIOC (Semantically-Interlinked Online Communities) Core Ontology".to_string(),
            "Benchmark Graph for VOWL".to_string(),
        ]
        .into_iter()
        .map(|ontology| {
            let ontology_value = ontology.clone();
            view! { <option value=ontology_value>{ontology}</option> }
        })
        .collect_view()
    };

    view! {
        <div class="mb-2">
            <label class="block mb-1">"Premade Ontology:"</label>
            <select 
                class="w-full border-b-0 rounded p-1 text-sm bg-gray-200"
                prop:value=selected_ontology
                on:change=move |ev| {
                    let target: HtmlInputElement = event_target::<HtmlInputElement>(&ev);
                    selected_ontology.set(target.value());
                }
            >
                {ontologies()}
            </select>
        </div>
    }
}

#[component]
fn UploadInput() -> impl IntoView {
    let message = RwSignal::new(String::new());
    // TODO: This should be a loading widget until stuff has loaded
    // Maybe, if possible, also make some sort of progress bar or status update.
    let custom_request = move |file_list: FileList| {
        let len = file_list.length();
        message.set(format!("Number of uploaded files: {}", len));
    };

    // TODO: Make accept formats a pointer to somewhere in network module as it should have definitions for accepted input.
    view! {
        <div class="mb-2">
            <label class="block mb-1">"From URL:"</label>
            <input
                class="w-full border-b-0 rounded p-1 bg-gray-200"
                placeholder="Enter input URL"
            />
        </div>
        <div class="mb-2">
            <label class="block mb-1">"From File:"</label>
            <div class="relative">
                <input
                    id="file-upload"
                    type="file"
                    class="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
                    multiple=""
                    accept=".owl,.owx,.xml,.json,.ttl"
                    on:change=move |ev| {
                        let input: HtmlInputElement =event_target::<HtmlInputElement>(&ev);
                        if let Some(files) = input.files() {
                            custom_request(files);
                        }
                    }
                />
                <label for="file-upload"
                    class="block w-full border-b-0 rounded p-1 bg-gray-200"
                    >
                    "Select ontology file(s)"
                </label>
            </div>
            {move || {
                let msg = message.get();
                (!msg.is_empty()).then(|| view! {<p class="mt-1 text-green">{msg}</p>})
            }}
        </div>
    }
}

fn Sparql() -> impl IntoView {

    let textarea_ref = NodeRef::<leptos::html::Textarea>::new();

    let handle_input = move |_| {
        if let Some(el) = textarea_ref.get() {
            el.style("height: auto");

            let scroll = el.scroll_height();
            let new_height = scroll - 16;

            el.style(("height",format!("{}px", new_height)));
        }
    };

    view! {
        <fieldset>
            <legend>"SPARQL Query:"</legend>
            <div class="flex flex-col gap-2">
                <div>
                    <label class="block text-xs text-gray mb-1">"Query Endpoint"</label>
                    <input class="w-full text-xs border-b-0 rounded p-1 bg-gray-200" placeholder="Enter query endpoint"/>
                </div>

                <div>
                    <label class="block text-xs text-gray mb-1">"Query"</label>
                    <textarea
                        node_ref=textarea_ref
                        class="w-full text-xs border-b-0 rounded p-1 resize-none overflow-hidden min-h-24 bg-gray-200"
                        rows=1
                        placeholder="Enter SPARQL query"
                        on:input=handle_input
                        
                    />
                </div>
            </div>
        </fieldset>
    }
}

#[component]
pub fn OntologyMenu() -> impl IntoView {
    view! {
        <WorkbenchMenuItems title="Load Ontology">
            <SelectStaticInput />
            <UploadInput />
            <Sparql />
        </WorkbenchMenuItems>
    }
}
