use leptos::prelude::*;
use thaw::*;
use crate::pages::home::*;

#[component]
pub fn SearchMenu() -> impl IntoView {
    let ShowSearchMenu(show_search_menu) = use_context::<ShowSearchMenu>().expect("ShowSearchMenu should be provided");
    let search_query = RwSignal::new(String::new());
    view! {
        <div class=move || {
        if show_search_menu.get() {
            "workbench-menu"
        } else {
            "workbench-menu menu-hidden"
        }
        }>
            <div class="workbench-menu-header">
                <h3>"Search"</h3>
            </div>
            <div class="workbench-menu-content">
                <p class="workbench-input-label">"Enter search query:"</p>
                <Input
                    class="workbench-url-input"
                    placeholder="Enter search query"
                    value=search_query
                />
                <p class="workbench-input-label">"SPARQL Query:"</p>
                <Textarea class="workbench-sparql-input" placeholder="Enter SPARQL query"/>
            </div>
        </div>
    }
}
