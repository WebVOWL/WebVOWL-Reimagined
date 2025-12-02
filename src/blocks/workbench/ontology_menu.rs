use super::WorkbenchMenuItems;
use futures::StreamExt;
use std::cell::RefCell;
use std::rc::Rc;
use gloo_timers::callback::Interval;
use leptos::prelude::*;
use leptos::server_fn::codec::{MultipartData, MultipartFormData, StreamingText, TextStream};
use leptos::task::spawn_local;
use web_sys::HtmlInputElement;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};
#[cfg(feature = "server")]
use webvowl_database::store::WebVOWLStore;

#[cfg(feature = "ssr")]
mod progress {
    use async_broadcast::{Receiver, Sender, broadcast};
    use dashmap::DashMap;
    use futures::Stream;
    use std::sync::LazyLock;

    struct File {
        total: usize,
        tx: Sender<usize>,
        rx: Receiver<usize>,
    }

    static FILES: LazyLock<DashMap<String, File>> = LazyLock::new(DashMap::new);

    pub async fn add_chunk(filename: &str, len: usize) {
        let mut entry = FILES.entry(filename.to_string()).or_insert_with(|| {
            let (mut tx, rx) = broadcast(128);
            tx.set_overflow(true);
            File { total: 0, tx, rx }
        });
        entry.total += len;
        let new_total = entry.total;

        let tx = entry.tx.clone();
        drop(entry);

        let _ = tx.broadcast(new_total).await;
    }

    pub fn reset(filename: &str) {
        if let Some(mut entry) = FILES.get_mut(filename) {
            // println!(
            //     "Resetting progress for '{}'. Old total: {}",
            //     filename, entry.total
            // );
            entry.total = 0;
        } else {
            // println!(
            //     "Reset called for '{}' but no entry found in FILES map",
            //     filename
            // );
        }
    }

    pub fn remove(filename: &str) {
        if FILES.remove(filename).is_some() {
            // println!("Removed progress entry for '{}'", filename);
        }
    }

    pub fn for_file(filename: String) -> impl Stream<Item = usize> {
        let entry = FILES.entry(filename).or_insert_with(|| {
            let (mut tx, rx) = broadcast(2048);
            tx.set_overflow(true);
            File { total: 0, tx, rx }
        });
        entry.rx.clone()
    }
}

#[server(output = StreamingText)]
pub async fn ontology_progress(filename: String) -> Result<TextStream, ServerFnError> {
    // println!("ontology_progress called for: {}", filename);
    let progress = progress::for_file(filename);
    let progress = progress.map(|bytes| Ok(format!("{bytes}\n")));
    Ok(TextStream::new(progress))
}

#[server(
    input = MultipartFormData,
)]
pub async fn load_ontology(data: MultipartData) -> Result<usize, ServerFnError> {
    let mut session = WebVOWLStore::default();
    let mut data = data.into_inner().unwrap();
    let mut count = 0;
    while let Ok(Some(mut field)) = data.next_field().await {
        let name = field.file_name().unwrap_or_default().to_string();

        if !name.is_empty() {
            progress::reset(&name);
            let _ = session.start_upload(&name).await;
        }

        while let Ok(Some(chunk)) = field.chunk().await {
            let len = chunk.len();
            count += len;
            let _ = session.upload_chunk(&chunk).await;
            progress::add_chunk(&name, len).await;
        }

        if !name.is_empty() {
            progress::remove(&name);
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
    let (filename, set_filename) = signal("Select File".to_string());
    let (file_size, set_file_size) = signal(0usize);
    let (upload_progress, set_upload_progress) = signal(0);
    let (parsing_status, set_parsing_status) = signal(String::new());
    let (parsing_done, set_parsing_done) = signal(false);
    let interval_handle: Rc<RefCell<Option<Interval>>> = Rc::new(RefCell::new(None));
    let upload_action = Action::new_local(|data: &FormData| load_ontology(data.clone().into()));
    let upload_result = upload_action.value();

    Effect::new({
        let interval_handle = Rc::clone(&interval_handle);
        move |_| {
            if let Some(result) = upload_result.get() {
                match result {
                    Ok(_) => {
                        if let Some(interval) = interval_handle.borrow_mut().take() {
                            interval.cancel();
                        }
                        set_parsing_status.set("Parsing complete".to_string());
                        set_parsing_done.set(true);
                    }
                    Err(err) => {
                        if let Some(interval) = interval_handle.borrow_mut().take() {
                            interval.cancel();
                        }
                        set_parsing_status.set(format!("Parsing failed: {err}"));
                        set_parsing_done.set(false);
                    }
                }
            }
        }
    });
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

                // Clear the file input to allow re-selecting the same file
                if let Some(file_input) = target.elements().named_item("file_to_upload") {
                     if let Some(input) = file_input.dyn_ref::<HtmlInputElement>() {
                         input.set_value("");
                     }
                }

                let fname = filename.get_untracked();
                let fsize = file_size.get_untracked();

                set_upload_progress.set(0);
                set_parsing_status.set(String::new());
                set_parsing_done.set(false);

                let interval_handle = Rc::clone(&interval_handle);
                spawn_local(async move {
                    match ontology_progress(fname).await {
                        Ok(stream_result) => {
                            // Dispatch upload AFTER connecting to stream
                            upload_action.dispatch_local(form_data);

                            let mut stream = stream_result.into_inner();
                            while let Some(result) = stream.next().await {
                                match result {
                                    Ok(chunk) => {
                                        if let Ok(bytes) = chunk.trim().parse::<usize>() {
                                            if fsize > 0 {
                                                let percent = (bytes as f64 / fsize as f64) * 100.0;
                                                set_upload_progress.set(percent as i32);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        leptos::logging::error!("Stream error: {:?}", e);
                                    }
                                }
                            }
                            set_upload_progress.set(100);
                            set_parsing_status.set("Parsing".to_string());
                            let interval = Interval::new(1500, move || {
                                set_parsing_status.update(|s| {
                                    if s.ends_with("......") {
                                        *s = "Parsing".to_string();
                                    } else {
                                        s.push('.');
                                    }
                                });
                            });
                            {
                                let mut handle = interval_handle.borrow_mut();
                                if let Some(existing) = handle.take() {
                                    existing.cancel();
                                }
                                *handle = Some(interval);
                            }
                        },
                        Err(e) => {
                            leptos::logging::error!("Failed to connect to progress stream: {:?}", e);
                            upload_action.dispatch_local(form_data);
                        }
                    }
                });
            }>
                <label class="cursor-pointer block w-full border-b-0 rounded p-1 bg-gray-200">
                    {filename}
                    <input type="file" name="file_to_upload" class="hidden"
                        on:change=move |ev| {
                            let target = event_target::<HtmlInputElement>(&ev);
                            if let Some(files) = target.files() {
                                if let Some(file) = files.get(0) {
                                    set_filename.set(file.name());
                                    set_file_size.set(file.size() as usize);
                                }
                            }
                            target.form().unwrap().request_submit().unwrap();
                        }
                    />
                </label>
                {move || {
                    let progress = upload_progress.get();
                    if progress > 0 {
                        view! {
                            <div class="w-full bg-gray-200 rounded-full h-2.5 mt-2 dark:bg-gray-700">
                                <div class="bg-blue-500 h-2.5 rounded-full transition-all duration-300" style=format!("width: {}%", std::cmp::min(progress, 100))></div>
                            </div>
                            {if progress >= 100 {
                                view! {
                                    <div class="text-sm mt-1 text-center font-bold">"Upload done"</div>
                                    {if !parsing_done.get() {
                                        view! { <div class="text-sm mt-1 text-center">{parsing_status}</div> }.into_any()
                                    } else {
                                        view! { <div class="text-sm mt-1 text-center font-bold">"Parsing done"</div> }.into_any()
                                    }}
                                }.into_any()
                            } else {
                                view! { <></> }.into_any()
                            }}
                        }.into_any()
                    } else {
                        view! { <></> }.into_any()
                    }
                }}
                <input type="submit" class="hidden" />
            </form>
            </div>
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
