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
        <Field label="Select Ontology:">
            <Select
                // class="ontology-dropdown"
                value=selected_ontology
                size=SelectSize::Large
            >
                {ontologies}
            </Select>
        </Field>
    }
}

#[component]
fn UploadInput() -> impl IntoView {
    let toaster = ToasterInjection::expect_context();

    // TODO: This should be a loading widget until stuff has loaded
    // Maybe, if possible, also make some sort of progress bar or status update.
    let custom_request = move |file_list: FileList| {
        let len = file_list.length();
        toaster.dispatch_toast(
            move || {
                view! {
                    <Toast>
                        <ToastBody>
                            {format!("Number of uploaded files: {len}")}
                        </ToastBody>
                    </Toast>
                }
            },
            Default::default(),
        );
    };

    // TODO: Make accept formats a pointer to somewhere in network module as it should have definitions for accepted input.
    view! {
        // <div class="custom-ontology-section">
        // <h4>"Custom Ontology:"</h4>
        <Field label="From URL:">
            <Input placeholder="Enter ontology IRI" />
        </Field>
        <Field label="From File:">
            <Upload
                accept=".owl,.owx,.xml,.json,.ttl"
                multiple=false
                name="upload_file"
                custom_request
            >
                <Button>
                    // class="ontology-upload-button"
                    "Select ontology file"
                </Button>
            </Upload>
        </Field>
    }
}

fn Sparql() -> impl IntoView {
    view! {
        <Field label="SPARQL Query:">
            <Textarea // class="workbench-sparql-input"
            placeholder="Enter SPARQL query" />
        </Field>
    }
}

#[component]
fn OntologyMenuPart() -> impl IntoView {
    view! {
        <div>
            // class="workbench-menu"
            <div>
                // class="workbench-menu-header"
                <h3>"Select Ontology"</h3>
            </div>
            <div>
                // class="workbench-menu-content"
                <SelectStaticInput />
                <UploadInput />
                <Sparql />
            </div>
        </div>
    }
}

#[component]
pub fn OntologyButton() -> impl IntoView {
    view! {
        <Button
            // class="work-bench-button"
            shape=ButtonShape::Square
            icon=icondata::BiMenuRegular
        ></Button>
    }
}

#[component]
pub fn OntologyMenu() -> impl IntoView {
    view! {
        <Popover
            trigger_type=PopoverTriggerType::Click
            position=PopoverPosition::RightStart
        >
            <PopoverTrigger slot>
                <OntologyButton />
            </PopoverTrigger>
            <OntologyMenuPart />
        </Popover>
    }
}
