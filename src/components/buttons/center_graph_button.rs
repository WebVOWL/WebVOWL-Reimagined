use crate::pages::home::*;
use crate::signals::menu_signals::SidebarOpen;
use leptos::prelude::*;

#[component]
pub fn CenterGraphButton() -> impl IntoView {
    let SidebarOpen(sidebar_open) = use_context::<SidebarOpen>().unwrap();

    view! {
        <button
            class:center-graph-button
            class=(
                "center-graph-button-collapsed",
                move || *sidebar_open.read() == false,
            )
        >
            "⌖"
        </button>
    }
}
