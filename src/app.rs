use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{FlatRoutes, Route, Router},
};
use thaw::*;
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="webvowl" href="/pkg/webvowl.css" />
        //<Link rel="shortcut icon" type_="image/ico" href="/favicon.ico" />
        <Router>
            <FlatRoutes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home />
            </FlatRoutes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let on_select = move |value: &str| leptos::logging::warn!("{}", value);
    //search bar
    let searchvalue = RwSignal::new(String::new());
    //filter menu    
    let datatypepropertycheck = RwSignal::new(false);
    let objectpropertycheck = RwSignal::new(false);
    let solitarypropertycheck = RwSignal::new(false);
    let classdisjointnesscheck = RwSignal::new(false);
    let setoperatorscheck = RwSignal::new(false);
    let degreeofcollapsingcheck = RwSignal::new(0.0);
    //options menu
    let zoomcheck = RwSignal::new(false);
    let dynamiclabelcheck = RwSignal::new(false);
    let nodescalingcheck = RwSignal::new(false);
    let compactnotationcheck = RwSignal::new(false);
    let colorexternalscheck = RwSignal::new(false);
    let classdistancevalue = RwSignal::new(0.0);
    let datatypedistancevalue = RwSignal::new(0.0);
    let maxlabelwidthvalue = RwSignal::new(0.0);
    //right info bar
    let ontologytitle = RwSignal::new("Friend of a Friend (FOAF) vocabulary".to_string());
    let ontologyiri = RwSignal::new("http://xmlns.com/foaf/0.1/".to_string());
    let ontologyversion = RwSignal::new("0.99".to_string());
    let ontologyauthors = RwSignal::new("Alice, Bob, Charlie".to_string());
    let ontologylanguages = RwSignal::new(vec!["en".to_string()]);
    view! {
        <Title text="Leptos + Tailwindcss" />
        <main>
            <div class="min-h-screen bg-[rgba(201, 196, 196, 1)]">
                <div class="fixed bottom-0 left-0 flex flex-row flex-wrap p-0 font-mono text-white">
                    <ConfigProvider>
                        <Input placeholder="Search" value=searchvalue />
                        <Button class="locateButton">"⌖"</Button>
                        //ontology menu
                        <Menu on_select=on_select position=MenuPosition::Top>
                            <MenuTrigger slot>
                                <Button icon=icondata::BiMenuRegular>"Ontology"</Button>
                            </MenuTrigger>
                            <Button>"Friend of a Friend (FOAF) vocabulary"</Button> 
                            <Button>"GoodRelations Vocabulary for E-Commerce"</Button>
                            <Button>"Modular and Unified Tagging Ontology (MUTO)"</Button>
                            <Button>"Personas Ontology (PersonasOnto)"</Button>  
                            <Button>"SIOC (Semantically-Interlinked Online Communities) Core Ontology"</Button>
                            <Button>"Benchmark Graph for VOWL"</Button>
                            <Label>"Custom Ontology"</Label>
                            <Input placeholder="Enter ontology IRI"/>
                            <Upload>
                                <Button>
                                    "Select ontology file"
                                </Button>
                            </Upload>
                        </Menu>
                        //export menu
                        <Menu on_select position=MenuPosition::Top>
                            <MenuTrigger slot>
                                <Button icon=icondata::BiExportRegular>"Export"</Button>
                            </MenuTrigger>
                            <Button>"Export as JSON"</Button>
                            <Button>"Export as SVG"</Button>
                            <Button>"Export as TeX"</Button>
                            <Button>"Export as TTL"</Button>
                            <Button>"Export as URL"</Button>
                        </Menu>
                        //filter menu
                        <Menu on_select position=MenuPosition::Top>
                            <MenuTrigger slot>
                                <Button icon=icondata::BiFilterAltRegular>"Filter"</Button>
                            </MenuTrigger>
                            <Checkbox checked=datatypepropertycheck label="Datatype properties" />
                            <Checkbox checked=objectpropertycheck label="Object properties" />
                            <Checkbox checked=solitarypropertycheck label="Solitary properties" />
                            <Checkbox checked=classdisjointnesscheck label="Class disjointness" />
                            <Checkbox checked=setoperatorscheck label="Set operators" />
                            <Slider value=degreeofcollapsingcheck max=16.0 step=1.0 show_stops=false />
                            <Label>"Degree of collapsing"</Label>
                        </Menu>

                        //options menu
                        <Menu on_select position=MenuPosition::Top>
                            <MenuTrigger slot>
                                <Button icon=icondata::IoOptions>"Options"</Button>
                            </MenuTrigger>
                            <Checkbox checked=zoomcheck label="Zoom controls" />
                            <Slider value=classdistancevalue max=600.0 step=10.0 show_stops=false />
                            <Label>"Class Distance"</Label>
                            <Slider value=datatypedistancevalue max=600.0 step=10.0 show_stops=false />
                            <Label>"Datatype Distance"</Label>
                            <Checkbox checked=dynamiclabelcheck label="Dynamic labels" />
                            <Slider value=maxlabelwidthvalue max=600.0 step=10.0 show_stops=false />
                            <Label>"Max label width"</Label>
                            <Checkbox checked=nodescalingcheck label="Node scaling" />
                            <Checkbox checked=compactnotationcheck label= "Compact notation" />
                            <Checkbox checked=colorexternalscheck label="Color externals" />
                        </Menu>
                        //modes menu
                        <Menu on_select position=MenuPosition::Top>
                            <MenuTrigger slot>
                                <Button icon=icondata::AiStarOutlined>"Modes"</Button>
                            </MenuTrigger>
                            <Button>"Editing (experimental)"</Button>
                            <Button>"Pick & pin"</Button>
                        </Menu>
                        <Button icon=icondata::VsDebugRestart>"Reset"</Button>
                        <Button icon=icondata::AiPauseOutlined>"Pause"</Button>
                        <Menu on_select position=MenuPosition::Top>
                            <MenuTrigger slot>
                                <Button icon=icondata::AiCopyrightOutlined>"About"</Button>
                            </MenuTrigger>
                            <Button>"MIT License © 2014-2019"</Button>
                            <Caption1Strong>"WebVOWL Developers:"</Caption1Strong>
                            <Caption1Strong>"Vincent Link, Steffen Lohmann, Eduard Marbach, Stefan Negru, Vitalis Wiens"</Caption1Strong>
                        </Menu>
                    </ConfigProvider>
                </div>
                //right info bar
                <div class="info-bar">
                    <div class="ontology-title">{move || ontologytitle.get()}</div>
                    <p>{{move || ontologyiri.get()}}</p>
                    <p>{move || ontologyversion.get()}</p>
                    <Caption1Strong>"Author(s): "</Caption1Strong>
                    <Caption1Strong>{move || ontologyauthors.get()}</Caption1Strong>
                    <Caption1Strong>"Language(s): "</Caption1Strong>
                </div>
                <div class="accordion-header">
                    <Accordion collapsible=true multiple=true>
                        <AccordionItem value="description">
                            <AccordionHeader slot>
                                "Description"
                            </AccordionHeader>
                            <p id="description" class="info-section-content">"The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language."</p>
                        </AccordionItem>
                    </Accordion>
                    <Accordion collapsible=true multiple=true>
                        <AccordionItem value="setadata">
                                <AccordionHeader slot>
                                    "Metadata"
                                </AccordionHeader>
                                <p id="metadata" class="info-section-content">"title: Friend of a Friend (FOAF) vocabulary
                                Work on the GoodRelations ontology and related research and development has been partly supported by the Austrian BMVIT/FFG under the FIT-IT Semantic Systems project myOntology (grant no. 812515/9284), by a Young Researcher's Grant (Nachwuchsfoerderung 2005-2006) from the Leopold-Franzens-Universitaet Innsbruck, by the European Commission under the project SUPER (FP6-026850), and by the German Federal Ministry of Research (BMBF) by a grant under the KMU Innovativ program as part of the Intelligent Match project (FKZ 01IS10022B). The"</p>
                        </AccordionItem>
                    </Accordion>
                    <Accordion collapsible=true multiple=true>
                        <AccordionItem value="statistics">
                            <AccordionHeader slot>
                                "Statistics"
                            </AccordionHeader>
                            <div class="info-section-content">
                                <p>"Classes: "</p>
                                <p>"Object Properties: "</p>
                                <p>"Datatype Properties: "</p>
                                <p>"Individuals: "</p>
                                <p>"Nodes: "</p>
                                <p>"Edges: "</p>
                            </div>
                        </AccordionItem>
                    </Accordion>
                    <Accordion collapsible=true multiple=true>
                        <AccordionItem value="selection details">
                            <AccordionHeader slot>
                                "Selection Details"
                            </AccordionHeader>
                            <p id="selection_details" class="info-section-content">"Select an element in the visualization."</p>
                        </AccordionItem>
                    </Accordion>
                </div>
            </div>
        </main>
    }
}