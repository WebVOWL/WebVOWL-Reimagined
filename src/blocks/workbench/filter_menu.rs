use leptos::prelude::*;
use thaw::*;
use crate::pages::home::*;

#[component]
pub fn FilterMenu() -> impl IntoView {
    let ShowFilterMenu(show_filter_menu) = use_context::<ShowFilterMenu>().expect("ShowFilterMenu should be provided");
    view! {
        <div class=move || {
        if show_filter_menu.get() {
            "workbench-menu"
        } else {
            "workbench-menu menu-hidden"
        }
        }>
            <div class="workbench-menu-header">
                <h3>"Filter"</h3>
            </div>
            <div class="workbench-menu-content">
            <ConfigProvider>
                <Accordion class="filter-menu-accordion" collapsible=true multiple=true>
                    <AccordionItem value="classes">
                        <AccordionHeader slot>
                            <p class="filter-menu-accordion-header">"Classes"</p>
                        </AccordionHeader>
                        <div class="filter-menu-accordion-section-content">
                            <div class="filter-menu-accordion-item">
                                <img src="/icons/Class.png" alt="Class"/>
                                <span>"Class: 20"</span>
                            </div>
                            <div class="filter-menu-accordion-item">
                                <img src="/icons/Deprecated_class.png" alt="Deprecated Class"/>
                                <span>"Deprecated Class: 5"</span>
                            </div>
                            <div class="filter-menu-accordion-item">
                                <img src="/icons/Anonymous_class.png" alt="Anonymous Class"/>
                                <span>"Anonymous Class: 5"</span>
                            </div>
                            <div class="filter-menu-accordion-item">
                                <img src="/icons/External_class.png" alt="External Class"/>
                                <span>"External Class: 5"</span>
                            </div>
                            <div class="filter-menu-accordion-item">
                                <img src="/icons/Equivalent_class.png" alt="Equivalent Class"/>
                                <span>"Equivalent Class: 15"</span>
                            </div>
                        </div>
                    </AccordionItem>
                </Accordion>
                <Accordion class="filter-menu-accordion" collapsible=true multiple=true>
                    <AccordionItem value="literals">
                        <AccordionHeader slot>
                            <p class="filter-menu-accordion-header">"Literals"</p>
                        </AccordionHeader>
                        <div class="filter-menu-accordion-section-content">
                            <div class="filter-menu-accordion-item">
                                <img src="/icons/Literal.png" alt="Literal"/>
                                <span>"Literal: 20"</span>
                            </div>
                        </div>
                    </AccordionItem>
                </Accordion>
                <Accordion class="filter-menu-accordion" collapsible=true multiple=true>
                    <AccordionItem value="RDF">
                        <AccordionHeader slot>
                            <p class="filter-menu-accordion-header">"RDF"</p>
                        </AccordionHeader>
                        <div class="filter-menu-accordion-section-content">
                            <div class="filter-menu-accordion-item">
                                <img src="/icons/RDF_class.png" alt="RDF Class"/>
                                <span>"RDF Class: 20"</span>
                            </div>
                            <div class="filter-menu-accordion-item">
                                <img src="/icons/RDF_resource.png" alt="RDF Resource"/>
                                <span>"RDF Resource: 15"</span>
                            </div>
                        </div>
                    </AccordionItem>
                </Accordion>
                <Accordion class="filter-menu-accordion" collapsible=true multiple=true>
                    <AccordionItem value="Operations">
                        <AccordionHeader slot>
                            <p class="filter-menu-accordion-header">"Set Operations"</p>
                        </AccordionHeader>
                        <div class="filter-menu-accordion-section-content">
                            <div class="filter-menu-accordion-item">
                                <img src="/icons/Union.png" alt="Union"/>
                                <span>"Union: 20"</span>
                            </div>
                            <div class="filter-menu-accordion-item">
                                <img src="/icons/Disjoint.png" alt="Intersection"/>
                                <span>"Intersection: 15"</span>
                            </div>
                            <div class="filter-menu-accordion-item">
                                <img src="/icons/Disjoint_union.png" alt="Difference"/>
                                <span>"Difference: 10"</span>
                            </div>
                            <div class="filter-menu-accordion-item">
                                <img src="/icons/Complement.png" alt="Complement"/>
                                <span>"Complement: 5"
                                <Checkbox checked=false />
                                </span>
                            </div>
                        </div>
                    </AccordionItem>
                </Accordion>
            </ConfigProvider>
            </div>
        </div>
    }
}