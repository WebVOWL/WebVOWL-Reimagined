use super::WorkbenchMenuItems;
use leptos::prelude::*;
use web_sys::{FormData, HtmlInputElement, FileList};
use crate::components::user_input::file_upload::FileUpload;


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
fn UploadInput(upload: FileUpload) -> impl IntoView {
    let message = RwSignal::new(String::new());
    // TODO: This should be a loading widget until stuff has loaded
    // Maybe, if possible, also make some sort of progress bar or status update.
    let custom_request = move |file_list: FileList| {
        let len = file_list.length();
        message.set(format!("Number of uploaded files: {}", len));

        let form = FormData::new().unwrap();
        for i in 0..len {
            if let Some(file) = file_list.item(i) {
                form.append_with_blob("file_to_upload", &file).unwrap();
            }
        }

        upload.local_action.dispatch_local(form);
        upload.mode.set("local".into());
    };

    let handle_url = move |url: String| {
        upload.remote_action.dispatch(url);
        upload.mode.set("remote".into());
    };

    // TODO: Make accept formats a pointer to somewhere in network module as it should have definitions for accepted input.
    view! {
        <div class="mb-2">
            <label class="block mb-1">"From URL:"</label>
            <input
                class="w-full border-b-0 rounded p-1 bg-gray-200"
                placeholder="Enter input URL"
                on:change=move |ev| {
                    let target: HtmlInputElement = event_target::<HtmlInputElement>(&ev);
                    handle_url(target.value());
                }
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


#[component]
fn Sparql(upload: FileUpload) -> impl IntoView {
    let endpoint_signal = RwSignal::new(String::new());
    let query_signal = RwSignal::new(String::new());

    let textarea_ref = NodeRef::<leptos::html::Textarea>::new();

    let handle_input = move |_| {
        if let Some(el) = textarea_ref.get() {
            el.style("height: auto");

            let scroll = el.scroll_height();
            let new_height = scroll - 16;

            el.style(("height",format!("{}px", new_height)));
        }
    };

    let run_sparql = move || {
        upload.mode.set("sparql".into());
        upload.sparql_action.dispatch((
            endpoint_signal.get(),
            query_signal.get(),
            Some("json".to_string()),
        ));
    };

    view! {
        <fieldset>
            <legend>"SPARQL Query:"</legend>
            <div class="flex flex-col gap-2">
                <div>
                    <label class="block text-xs text-gray mb-1">"Query Endpoint"</label>
                    <input class="w-full text-xs border-b-0 rounded p-1 bg-gray-200" placeholder="Enter query endpoint"
                        on:input=move |ev| {
                            let t: HtmlInputElement = event_target(&ev);
                            endpoint_signal.set(t.value());
                        }
                    />
                </div>

                <div>
                    <label class="block text-xs text-gray mb-1">"Query"</label>
                    <textarea
                        node_ref=textarea_ref
                        class="w-full text-xs border-b-0 rounded p-1 resize-none overflow-hidden min-h-24 bg-gray-200"
                        rows=1
                        placeholder="Enter query"
                        on:input=move |ev| {
                            let t: HtmlInputElement = event_target(&ev);
                            query_signal.set(t.value());
                            handle_input(());
                        }
                    />
                </div>

                <button class="p-1 mt-1 rounded bg-blue-500 text-white text-xs"
                        on:click=move |_| run_sparql()
                >"Run query"</button>
            </div>
        </fieldset>
    }
}

#[component]
pub fn OntologyMenu() -> impl IntoView {
    let upload = FileUpload::new();
    view! {
        <WorkbenchMenuItems title="Load Ontology">
            <SelectStaticInput />
            <UploadInput upload=upload.clone()/>
            <Sparql upload=upload.clone()/>

            // TEST ONLY
            // {
            //     move || {
            //         upload.get_result().map(|result| {
            //             view! {<p class="text-xs mt-2">{format!("{:?}", result)}</p>}
            //         })
            //     }
            // }
        </WorkbenchMenuItems>
    }
}