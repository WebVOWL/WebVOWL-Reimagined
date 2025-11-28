use super::WorkbenchMenuItems;
use leptos::prelude::*;
use leptos::server_fn::codec::{MultipartData, MultipartFormData};
use std::future::Future;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{FileList, HtmlInputElement};
use web_sys::{FormData, HtmlFormElement, SubmitEvent};
#[cfg(feature = "server")]
use webvowl_database::store::WebVOWLStore;

#[server(
    input = MultipartFormData,
)]
pub async fn load_ontology(data: MultipartData) -> Result<usize, ServerFnError> {
    let mut session = WebVOWLStore::default();
    let mut data = data.into_inner().unwrap();

    let mut count = 0;
    while let Ok(Some(mut field)) = data.next_field().await {
        println!("\n[NEXT FIELD]\n");
        let name = field.name().unwrap_or_default().to_string();
        println!("  [NAME] {name}");

        if let Some(filename) = field.file_name() {
            let filename = filename.to_string();
            let _ = session.start_upload(&filename).await;
        }

        while let Ok(Some(chunk)) = field.chunk().await {
            let len = chunk.len();
            count += len;
            println!("      [CHUNK] {len}");
            let _ = session.upload_chunk(&chunk).await;
        }
    }
    let _ = session.complete_upload().await;
    println!("Upload done. Total bytes uploaded: {count}");
    Ok(count)
}

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
    let upload_action = Action::new_local(|data: &FormData| load_ontology(data.clone().into()));

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

            <form on:submit=move |ev: SubmitEvent| {
                ev.prevent_default();
                let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
                let form_data = FormData::new_with_form(&target).unwrap();
                upload_action.dispatch_local(form_data);
            }>
                <input type="file" name="file_to_upload" oninput="this.form.requestSubmit()" />
                <input type="submit" />
            </form>
                // <label for="file-upload"
                //     class="block w-full border-b-0 rounded p-1 bg-gray-200"
                //     >
                //     "Select ontology file(s)"
                // </label>
            </div>
            // {move || {
            //     let msg = message.get();
            //     (!msg.is_empty()).then(|| view! {<p class="mt-1 text-green">{msg}</p>})
            // }}
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

            el.style(("height", format!("{}px", new_height)));
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
