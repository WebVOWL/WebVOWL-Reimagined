use leptos::prelude::*;
use thaw::*;
use crate::pages::home::*;

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
    view! {
        <div class=move || {
            if show_ontology_menu.get() {
                "ontology-menu"
            } else {
                "ontology-menu menu-hidden"
            }
        }>
            <div class="ontology-menu-header">
                <h3>"Select Ontology"</h3>
            </div>
                <div class="ontology-menu-content">
                    <ConfigProvider>
                        <p class="ontology-input-label">"Select Ontology:"</p>
                        <Select class="ontology-dropdown" value=selected_ontology>
                            {move || {
                                vec![
                                    "Friend of a Friend (FOAF) vocabulary".to_string(),
                                    "GoodRelations Vocabulary for E-Commerce".to_string(),
                                    "Modular and Unified Tagging Ontology (MUTO)".to_string(),
                                    "Personas Ontology (PersonasOnto)".to_string(),
                                    "SIOC (Semantically-Interlinked Online Communities) Core Ontology"
                                        .to_string(),
                                    "Benchmark Graph for VOWL".to_string(),
                                ]
                                    .into_iter()
                                    .map(|ontology| {
                                        let ontology_value = ontology.clone();
                                        view! { <option value=ontology_value>{ontology}</option> }
                                    })
                                    .collect_view()
                            }}
                        </Select>
                    </ConfigProvider>
                    <div class="custom-ontology-section">
                        <h4>"Custom Ontology:"</h4>
                        <p class="ontology-input-label">"From URL:"</p>
                        <Input class="ontology-url-input" placeholder="Enter ontology IRI" />
                        <p class="ontology-input-label">"From File:"</p>
                        <Upload>
                            <Button class="ontology-menu-item">
                                "Select ontology file"
                            </Button>
                        </Upload>
                        <p class="ontology-input-label">"SPARQL Query:"</p>
                        <Textarea class="ontology-sparql-input" placeholder="Enter SPARQL query"/>
                    </div>
                </div>
            </div>
    }
}
