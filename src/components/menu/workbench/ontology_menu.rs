use crate::components::buttons::work_bench_buttons::OntologyButton;
use leptos::prelude::*;
use thaw::*;

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
        <Field
            label="Select Ontology:"
            >
            <Select class="ontology-dropdown" value=selected_ontology>
                {ontologies}
            </Select>
        </Field>
    }
}

#[component]
fn UploadInput() -> impl IntoView {
    view! {
       <div class="custom-ontology-section">
           <h4>"Custom Ontology:"</h4>
           <p class="workbench-input-label">"From URL:"</p>
           <Input class="workbench-url-input" placeholder="Enter ontology IRI" />
           <p class="workbench-input-label">"From File:"</p>
           <Upload>
               <Button class="ontology-upload-button">
                   "Select ontology file"
               </Button>
           </Upload>
           <p class="workbench-input-label">"SPARQL Query:"</p>
           <Textarea class="workbench-sparql-input" placeholder="Enter SPARQL query"/>
       </div>
    }
}

fn Sparql() -> impl IntoView {
    view! {
        <p class="workbench-input-label">"SPARQL Query:"</p>
        <Textarea class="workbench-sparql-input" placeholder="Enter SPARQL query"/>
    }
}

#[component]
fn OntologyMenuPart() -> impl IntoView {
    view! {
        <div class="workbench-menu-header">
            <h3>"Select Ontology"</h3>
        </div>
            <div class="workbench-menu-content">
                <UploadInput />
                <Sparql />
            </div>
    }
}

#[component]
pub fn OntologyMenu() -> impl IntoView {
    view! {
        <ConfigProvider>
            <Popover
                trigger_type=PopoverTriggerType::Click
                position=PopoverPosition::RightStart
            >
                <PopoverTrigger slot>
                    <OntologyButton />
                </PopoverTrigger>
                <OntologyMenuPart />
            </Popover>
        </ConfigProvider>
    }
}
