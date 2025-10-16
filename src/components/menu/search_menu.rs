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
            "search-menu"
        } else {
            "search-menu menu-hidden"
        }
        }>
            <div class="search-menu-header">
                <h3>"Search"</h3>
            </div>
            <div class="search-menu-content">
                <p class="ontology-input-label">"Enter search query:"</p>
                <Input
                    class="search-url-input"
                    placeholder="Enter search query"
                    value=search_query
                />
                <p class="ontology-input-label">"SPARQL Query:"</p>
                <Textarea class="ontology-sparql-input" placeholder="Enter SPARQL query"/>
            </div>
        </div>
    }

}
