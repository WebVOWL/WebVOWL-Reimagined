use super::WorkbenchMenuItems;
use crate::components::icon::Icon;
use futures::StreamExt;
use leptos::prelude::*;
use leptos::server_fn::codec::{ByteStream, Streaming};
#[cfg(feature = "server")]
use webvowl_database::store::WebVOWLStore;

#[server(output = Streaming)]
pub async fn export_owl() -> Result<ByteStream<ServerFnError>, ServerFnError> {
    let store = WebVOWLStore::default();
    let stream = store.serialize_stream().await?;
    Ok(ByteStream::new(stream.map(|chunk| {
        chunk
            .map_err(|e| ServerFnError::new(e.to_string()))
            .map(|bytes| bytes::Bytes::from(bytes))
    })))
}

#[component]
pub fn ExportButton(
    #[prop(into)] label: String,
    #[prop(into)] icon: icondata::Icon,
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let onclick_handler = move |_| {
        if let Some(cb) = &on_click {
            cb.run(());
        }
    };

    view! {
        <button
            class="relative flex items-center justify-center gap-1 h-10 w-40 m-2 bg-gray-200 text-[#000000] rounded-sm font-semibold hover:bg-[#dd9900] transition-colors cursor-pointer"
            on:click=onclick_handler
        >
            <Icon icon=icon />
            {label}
        </button>
    }
}

#[component]
pub fn ExportMenu() -> impl IntoView {
    let progress_message = RwSignal::new(String::new());

    view! {
        <WorkbenchMenuItems title="Export Ontology">
            <div class="flex justify-center flex-wrap w-full">
                //<ExportButton label="Json" icon=icondata::BiExportRegular />
                //<ExportButton label="SVG" icon=icondata::BiExportRegular />
                //<ExportButton label="TeX" icon=icondata::BiExportRegular />
                //<ExportButton label="TTL" icon=icondata::BiExportRegular />
                //<ExportButton label="URL" icon=icondata::BiExportRegular />
                <ExportButton label="RDF" 
                icon=icondata::BiExportRegular
                on_click=Callback::new(move |_| {
                    leptos::task::spawn_local(async move {
                        match export_owl().await {
                            Ok(byte_stream) => {
                                progress_message.set("Downloaded: 0 MB".to_string());
                                let mut stream = byte_stream.into_inner();
                                let mut data = Vec::new();
                                let mut downloaded = 0;
                                while let Some(chunk) = stream.next().await {
                                    match chunk {
                                        Ok(bytes) => {
                                            downloaded += bytes.len();
                                            let mb = downloaded as f64 / 1_024.0 / 1_024.0;
                                            progress_message.set(format!("Downloaded: {:.2} MB", mb));
                                            data.extend(bytes);
                                        },
                                        Err(e) => leptos::logging::error!("Error in stream: {:?}", e),
                                    }
                                }
                                progress_message.set("Processing...".to_string());
                                leptos::logging::log!("Export success, data length: {}", data.len());
                                #[cfg(target_arch = "wasm32")]
                                {
                                    use web_sys::{Blob, Url, HtmlAnchorElement};
                                    use web_sys::wasm_bindgen::JsCast;

                                    let window = web_sys::window().unwrap();
                                    let document = window.document().unwrap();
                                    let body = document.body().unwrap();

                                    let blob_parts = web_sys::js_sys::Array::new();
                                    let uint8_array = web_sys::js_sys::Uint8Array::from(data.as_slice());
                                    blob_parts.push(&uint8_array.into());

                                    let mut blob_options = web_sys::BlobPropertyBag::new();
                                    blob_options.set_type("application/rdf+xml");

                                    let blob = Blob::new_with_str_sequence_and_options(&blob_parts, &blob_options).unwrap();
                                    let url = Url::create_object_url_with_blob(&blob).unwrap();
                                    leptos::logging::log!("URL: {}", url);
                                    let a = document.create_element("a").unwrap().unchecked_into::<HtmlAnchorElement>();
                                    a.set_href(&url);
                                    a.set_download("ontology.rdf");
                                    a.set_attribute("style", "display: none").unwrap();

                                    body.append_child(&a).unwrap();
                                    a.click();
                                    body.remove_child(&a).unwrap();
                                    Url::revoke_object_url(&url).unwrap();
                                    progress_message.set("Download complete".to_string());
                                }
                            }
                            Err(e) => {
                                leptos::logging::error!("Export failed: {:?}", e);
                                progress_message.set(format!("Export failed: {:?}", e));
                            }
                        }
                    });
                })
                 />
                //<ExportButton label="OWL" icon=icondata::BiExportRegular/>
            </div>
            {move || {
                let msg = progress_message.get();
                (!msg.is_empty()).then(|| view! {
                    <div class="w-full text-center text-sm mt-2 text-gray-600">
                        {msg}
                    </div>
                })
            }}
        </WorkbenchMenuItems>
    }
}
