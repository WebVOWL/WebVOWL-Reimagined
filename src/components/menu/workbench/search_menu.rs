use leptos::prelude::*;
use thaw::*;

#[component]
pub fn SearchMenu() -> impl IntoView {
    view! {
        <div> "Hello search!" </div>
        <Checkbox label="Enable Search"/>
}   
}
