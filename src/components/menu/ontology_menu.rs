use leptos::prelude::*;
use thaw::*;

#[component]
pub fn OntologyMenu() -> impl IntoView {
    let ontologytitle = use_context::<RwSignal<String>>().expect("ontologytitle should be provided");
    let on_select = Box::new(|value: &str| leptos::logging::warn!("{}", value));
    
    view! {
            <ConfigProvider>
                <Menu on_select=on_select trigger_type=MenuTriggerType::Hover position=MenuPosition::Top>
                    <MenuTrigger slot>
                        <Button shape=ButtonShape::Square icon=icondata::BiMenuRegular>"Ontology"</Button>
                    </MenuTrigger>
                    <Button on_click=move |_| ontologytitle.set("Friend of a Friend (FOAF) vocabulary".to_string())>
                        "Friend of a Friend (FOAF) vocabulary"
                    </Button> 
                    <Button on_click=move |_| ontologytitle.set("GoodRelations Vocabulary for E-Commerce".to_string())>
                        "GoodRelations Vocabulary for E-Commerce"
                    </Button>
                    <Button on_click=move |_| ontologytitle.set("Modular and Unified Tagging Ontology (MUTO)".to_string())>
                        "Modular and Unified Tagging Ontology (MUTO)"
                    </Button>
                    <Button on_click=move |_| ontologytitle.set("Personas Ontology (PersonasOnto)".to_string())>
                        "Personas Ontology (PersonasOnto)"
                    </Button>
                    <Button on_click=move |_| ontologytitle.set("SIOC (Semantically-Interlinked Online Communities) Core Ontology".to_string())>
                        "SIOC (Semantically-Interlinked Online Communities) Core Ontology"
                    </Button>
                    <Button on_click=move |_| ontologytitle.set("Benchmark Graph for VOWL".to_string())>
                        "Benchmark Graph for VOWL"
                    </Button>
                    <Label>"Custom Ontology"</Label>
                    <Input placeholder="Enter ontology IRI" />
                    <Upload>
                        <Button>
                            "Select ontology file"
                        </Button>
                    </Upload>
                </Menu>
            </ConfigProvider>
    }
}