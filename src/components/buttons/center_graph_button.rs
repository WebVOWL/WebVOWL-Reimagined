use leptos::prelude::*;
use thaw::*;
use crate::pages::home::*;

#[component]
pub fn CenterGraphButton() -> impl IntoView {
    let SidebarOpen(sidebar_open) = use_context::<SidebarOpen>().expect("SidebarOpen should be provided");
    let IsFirstLoad(is_first_load) = use_context::<IsFirstLoad>().expect("IsFirstLoad should be provided");
    view! {
        <div class=move || {
            if is_first_load.get() {
                if sidebar_open.get() {
                    "center-graph-button center-graph-button-expand"
                } else {
                    "center-graph-button center-graph-button-collapse center-graph-button-collapsed"
                }
            } else {
                if sidebar_open.get() {
                    "center-graph-button"
                } else {
                    "center-graph-button center-graph-button-collapsed"
                }
            }
        }>    
            <Button>"⌖"</Button>
        </div>
    }
}