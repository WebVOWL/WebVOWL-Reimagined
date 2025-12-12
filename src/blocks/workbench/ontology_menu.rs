use super::WorkbenchMenuItems;
use crate::components::user_input::file_upload::{FileUpload, handle_internal_sparql};
use crate::sparql_queries::testing::TESTING_QUERY;
use grapher::prelude::GraphDisplayData;
use grapher::prelude::{EVENT_DISPATCHER, RenderEvent};
use leptos::prelude::*;
use leptos::task::spawn_local;
use log::{error, info};
use web_sys::HtmlInputElement;

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
fn UploadInput(
    graph_data: RwSignal<GraphDisplayData>,
    total_graph_data: RwSignal<GraphDisplayData>,
) -> impl IntoView {
    let upload = FileUpload::new(graph_data.clone());
    let loading_done = upload.local_action.value();
    let upload_progress = upload.tracker.upload_progress.clone();
    let parsing_status = upload.tracker.parsing_status.clone();
    let parsing_done = upload.tracker.parsing_done.clone();
    let tracker_url = upload.tracker.clone();
    let tracker_file = upload.tracker.clone();

    Effect::new(move || {
        if let Some(value) = loading_done.get() {
            match value {
                Ok(_) => spawn_local(async move {
                    let output_result = handle_internal_sparql(TESTING_QUERY.to_string()).await;
                    match output_result {
                        Ok(new_graph_data) => {
                            graph_data.set(new_graph_data.clone());
                            total_graph_data.set(new_graph_data.clone());
                            EVENT_DISPATCHER
                                .rend_write_chan
                                .send(RenderEvent::LoadGraph(new_graph_data));
                        }
                        Err(e) => error!("{}", e),
                    }
                }),
                Err(e) => error!("{}", e),
            }
        }
    });

    view! {
         <div class="mb-2">
            <label class="block mb-1">"From URL:"</label>
            <input
                class="w-full border-b-0 rounded p-1 bg-gray-200"
                placeholder="Enter input URL"
                on:input=move |ev| {
                    let target: HtmlInputElement = event_target(&ev);
                    let url = target.value();

                    tracker_url.upload_url(url.clone(), move |u| {
                        upload.remote_action.dispatch(u);
                        upload.mode.set("remote".to_string());
                    });
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
                    accept=".owl,.ofn,.owx,.xml,.json,.ttl,.rdf,.nt,.nq,.trig,.jsonld,.n3,.srj,.srx,.json,.xml,.csv,.tsv"
                    on:input=move |ev| {
                        let input: HtmlInputElement = event_target(&ev);
                        if let Some(files) = input.files() {
                            tracker_file.upload_files(files, move |form|{
                                info!("Uploading files");
                                upload.local_action.dispatch_local(form);
                                upload.mode.set("local".to_string());
                            });
                        } else {
                            info!("Found no files to upload");
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
                // let msg = message.get();
                // (!msg.is_empty()).then(|| view! {<p class="mt-1 text-green">{msg}</p>})
                let progress = upload_progress.get();
                let parsing = parsing_status.get();
                let done = parsing_done.get();

                if progress > 0 {
                    view! {
                        <div class="mt-2">
                            <div class="w-full bg-gray-200 rounded-full h-2.5 mt-2 dark:bg-gray-700">
                                <div class="bg-blue-500 h-2.5 rounded-full transition-all duration-300" style=format!("width: {}%", std::cmp::min(progress, 100))></div>
                            </div>
                            {if progress >= 100 {
                                view! {
                                    <div class="text-sm mt-1 text-center font-bold">"Upload done"</div>
                                    {if !done {
                                        view! { <div class="text-sm mt-1 text-center">{parsing}</div> }.into_any()
                                    } else {
                                        view! { <div class="text-sm mt-1 text-center font-bold">"Parsing done"</div> }.into_any()
                                    }}
                                }.into_any()
                            } else {
                                view! { <></> }.into_any()
                            }}
                        </div>
                    }.into_any()
                } else {
                    view! { <></> }.into_any()
                }
            }}
        </div>
    }
}

#[component]
fn FetchData() -> impl IntoView {
    view! {
        <button on:click=move |_| {
            spawn_local(async {
                let output_result = handle_internal_sparql(TESTING_QUERY.to_string()).await;
                match output_result {
                    Ok(graph_data) => {
                        EVENT_DISPATCHER.rend_write_chan.send(RenderEvent::LoadGraph(graph_data));},
                    Err(e) => error!("{}", e),
                }})
        }>"reload data"</button>
    }
}

#[component]
fn Sparql(
    graph_data: RwSignal<GraphDisplayData>,
    total_graph_data: RwSignal<GraphDisplayData>,
) -> impl IntoView {
    let upload = FileUpload::new(graph_data.clone());
    let upload_progress = upload.tracker.upload_progress.clone();
    let parsing_status = upload.tracker.parsing_status.clone();
    let parsing_done = upload.tracker.parsing_done.clone();
    let tracker_sparql = upload.tracker.clone();

    let endpoint_signal = RwSignal::new(String::new());
    let query_signal = RwSignal::new(String::new());

    let textarea_ref = NodeRef::<leptos::html::Textarea>::new();

    let handle_input = move |_| {
        if let Some(el) = textarea_ref.get() {
            el.style("height: auto");

            let scroll = el.scroll_height();
            let new_height = scroll - 16;

            el.style(("height", format!("{}px", new_height)));
        }
    };

    let run_sparql = move || {
        tracker_sparql.upload_sparql(
            endpoint_signal.get(),
            query_signal.get(),
            move |(ep, q, fmt)| {
                upload.sparql_action.dispatch((ep, q, fmt));
                upload.mode.set("sparql".to_string());
            },
        );
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

                {move || {
                    let progress = upload_progress.get();
                    let parsing = parsing_status.get();
                    let done = parsing_done.get();

                    if progress > 0 {
                        view! {
                            <div class="mt-2">
                                <div class="w-full bg-gray-200 rounded-full h-2.5 mt-2 dark:bg-gray-700">
                                    <div class="bg-blue-500 h-2.5 rounded-full transition-all duration-300" style=format!("width: {}%", std::cmp::min(progress, 100))></div>
                                </div>
                                {if progress >= 100 {
                                    view! {
                                        <div class="text-sm mt-1 text-center font-bold">"Upload done"</div>
                                        {if !done {
                                            view! { <div class="text-sm mt-1 text-center">{parsing}</div> }.into_any()
                                        } else {
                                            view! { <div class="text-sm mt-1 text-center font-bold">"Parsing done"</div> }.into_any()
                                        }}
                                    }.into_any()
                                } else {
                                    view! { <></> }.into_any()
                                }}
                            </div>
                        }.into_any()
                    } else {
                        view! { <></> }.into_any()
                    }
                }}
            </div>
        </fieldset>
    }
}

#[component]
pub fn OntologyMenu(
    graph_data: RwSignal<GraphDisplayData>,
    total_graph_data: RwSignal<GraphDisplayData>,
) -> impl IntoView {
    view! {
        <WorkbenchMenuItems title="Load Ontology">
            <SelectStaticInput />
            <UploadInput
                graph_data=graph_data.clone()
                total_graph_data=total_graph_data.clone()
            />
            <Sparql
                graph_data=graph_data.clone()
                total_graph_data=total_graph_data.clone()
            />
            <FetchData />
        </WorkbenchMenuItems>
    }
}
