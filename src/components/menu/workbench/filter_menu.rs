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
            </div>
        </div>
    }
}