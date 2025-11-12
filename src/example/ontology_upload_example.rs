use leptos::prelude::*;
use thaw::*;
use crate::pages::home::*;
use leptos::wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};
use server_fn::codec::{MultipartData, MultipartFormData};
#[server(
    input = MultipartFormData,
)]
pub async fn file_length(
    data: MultipartData,
) -> Result<usize, ServerFnError> {
    let mut data = data.into_inner().unwrap();
    println!("{:?}", data.values());
    let mut count = 0;
    while let Ok(Some(mut field)) = data.next_field().await {
        println!("\n[NEXT FIELD]\n");
        let name = field.name().unwrap_or_default().to_string();
        println!("  [NAME] {name}");
        while let Ok(Some(chunk)) = field.chunk().await {
            let len = chunk.len();
            count += len;
            println!("      [CHUNK] {len}");
            // in a real server function, you'd do something like saving the file here
        }
    }
    Ok(count)
}
#[component]
pub fn OntologyMenu() -> impl IntoView {
    let ontologytitle =
        use_context::<RwSignal<String>>().expect("ontologytitle should be provided");
    let ShowOntologyMenu(show_ontology_menu) = use_context::<ShowOntologyMenu>().expect("ShowOntologyMenu should be provided");
    let selected_ontology = RwSignal::new("Friend of a Friend (FOAF) vocabulary".to_string());
    Effect::new(move |_| {
        let selected = selected_ontology.get();
        ontologytitle.set(selected);
    });
    let upload_action = Action::new_local(|data: &FormData| {
    file_length(data.clone().into())
    });
    view! {
        <div class=move || {
            if show_ontology_menu.get() {
                "workbench-menu"
            } else {
                "workbench-menu menu-hidden"
            }
        }>
            <div class="workbench-menu-header">
                <h3>"Select Ontology"</h3>
            </div>
                <div class="workbench-menu-content">
                    <div class="custom-ontology-section">
                        <h4>"Custom Ontology:"</h4>
                        <p class="workbench-input-label">"From URL:"</p>
                        <p class="workbench-input-label">"From File:"</p>
                        <form on:submit=move |ev: SubmitEvent| {
                            ev.prevent_default();
                            let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
                            let form_data = FormData::new_with_form(&target).unwrap();
                            upload_action.dispatch_local(form_data);
                        }>
                            <input type="file" name="file_to_upload" 
                                oninput="this.form.requestSubmit()"/>
                        </form>
                        <p>
                        {move || {
                            if upload_action.input().read().is_none() && upload_action.value().read().is_none()
                            {
                                "Upload a file.".to_string()
                            } else if upload_action.pending().get() {
                                "Uploading...".to_string()
                            } else if let Some(Ok(value)) = upload_action.value().get() {
                                value.to_string()
                            } else {
                                format!("{:?}", upload_action.value().get())
                            }
                        }}
                        </p>
                    </div>
                </div>
            </div>
    }
}
