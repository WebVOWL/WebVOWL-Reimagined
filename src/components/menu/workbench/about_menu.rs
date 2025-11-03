use leptos::prelude::*;
use thaw::*;
use crate::pages::home::*;

#[component]
pub fn AboutMenu() -> impl IntoView {
    let ShowAboutMenu(show_about_menu) = use_context::<ShowAboutMenu>().expect("ShowAboutMenu should be provided");
    view! {
        <div class=move || {
        if show_about_menu.get() {
            "workbench-menu"
        } else {
            "workbench-menu menu-hidden"
        }
        }>
            <div class="workbench-menu-header">
                <h3>"About"</h3>
            </div>
            <div class="workbench-menu-content">
            <p class="workbench-menu-text">
                "WebVOWL-Reimagined is an open-source ontology visualization tool developed to provide an enhanced user experience and improved performance over the original WebVOWL. It leverages modern web technologies to offer a more intuitive interface for exploring ontologies."
            </p>
            </div>
        </div>
    }
}