use super::{WorkBenchButton, WorkbenchMenuItems};
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
        <Field label="Premade Ontology:">
            <Select value=selected_ontology size=SelectSize::Large>
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
        <Field label="From URL:">
            <Input placeholder="Enter input URL" />
        </Field>
        <Field label="From File:">
            <Upload
                accept=".owl,.owx,.xml,.json,.ttl"
                multiple=false
                name="upload_file"
                custom_request
            >
                <Button>"Select ontology file"</Button>
            </Upload>
        </Field>
    }
}

fn Sparql() -> impl IntoView {
    view! {
        <fieldset>
            <legend>"SPARQL Query"</legend>
            <Flex gap=FlexGap::Size(20) vertical=true>
                <Field label="Query Endpoint">
                    <Input placeholder="Enter query endpoint" />
                </Field>
                <Field label="Query">
                    <Textarea
                        resize=TextareaResize::Vertical
                        size=TextareaSize::Large
                        placeholder="Enter SPARQL query"
                    />
                </Field>
            </Flex>
        </fieldset>
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
                <WorkBenchButton
                    text="Load"
                    icon=icondata::BiMenuRegular
                ></WorkBenchButton>
            </PopoverTrigger>
            <WorkbenchMenuItems title="Load Ontology">
                <SelectStaticInput />
                <UploadInput />
                <Sparql />
            </WorkbenchMenuItems>
        </Popover>
    }
}
