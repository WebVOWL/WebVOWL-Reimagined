use super::WorkbenchMenuItems;
use crate::components::icon::Icon;
use leptos::prelude::*;
#[cfg(feature = "server")]
use webvowl_database::store::WebVOWLStore;

#[server]
pub async fn export_owl() -> Result<String, ServerFnError> {
    let store = WebVOWLStore::default();
    let owl_string = store.serialize_to_string().await?;
    Ok(owl_string)
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
    view! {
        <WorkbenchMenuItems title="Export Ontology">
            <div class="flex justify-center flex-wrap w-full">
                <ExportButton label="Json" icon=icondata::BiExportRegular />
                <ExportButton label="SVG" icon=icondata::BiExportRegular />
                <ExportButton label="TeX" icon=icondata::BiExportRegular />
                <ExportButton label="TTL" icon=icondata::BiExportRegular />
                <ExportButton label="URL" icon=icondata::BiExportRegular />
                <ExportButton label="RDF" icon=icondata::BiExportRegular />
                <ExportButton
                    label="OWL"
                    icon=icondata::BiExportRegular
                    on_click=Callback::new(move |_| {
                        leptos::task::spawn_local(async {
                            match export_owl().await {
                                Ok(data) => {
                                    leptos::logging::log!("Export success, data length: {}", data.len());
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        use web_sys::{Blob, Url, HtmlAnchorElement};
                                        use wasm_bindgen::JsCast;

                                        let window = web_sys::window().unwrap();
                                        let document = window.document().unwrap();
                                        let body = document.body().unwrap();

                                        let blob_parts = web_sys::js_sys::Array::new();
                                        blob_parts.push(&data.into());

                                        let mut blob_options = web_sys::BlobPropertyBag::new();
                                        blob_options.set_type("application/rdf+xml");

                                        let blob = Blob::new_with_str_sequence_and_options(&blob_parts, &blob_options).unwrap();
                                        let url = Url::create_object_url_with_blob(&blob).unwrap();

                                        let a = document.create_element("a").unwrap().unchecked_into::<HtmlAnchorElement>();
                                        a.set_href(&url);
                                        a.set_download("ontology.owl");
                                        a.set_attribute("style", "display: none").unwrap();

                                        body.append_child(&a).unwrap();
                                        a.click();
                                        body.remove_child(&a).unwrap();
                                        Url::revoke_object_url(&url).unwrap();
                                    }
                                }
                                Err(e) => {
                                    leptos::logging::error!("Export failed: {:?}", e);
                                }
                            }
                        });
                    })
                />
            </div>
        </WorkbenchMenuItems>
    }
}
