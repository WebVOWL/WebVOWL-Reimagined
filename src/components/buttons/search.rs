use leptos::prelude::RwSignal;
use leptos::*;
use thaw::*;


#[component]
pub fn SearchButton() -> impl IntoView {
    let searchvalue = RwSignal::new("".to_string());
    view! {       
        <ConfigProvider>
            <Input placeholder="Search" value=searchvalue />
        </ConfigProvider>
    }
}