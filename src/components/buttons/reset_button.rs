use leptos::prelude::*;
use thaw::*;
use crate::pages::home::*;

#[component]
pub fn ResetButton() -> impl IntoView {
    let SidebarOpen(sidebar_open) = use_context::<SidebarOpen>().expect("SidebarOpen should be provided");
    let IsFirstLoad(is_first_load) = use_context::<IsFirstLoad>().expect("IsFirstLoad should be provided");
    view! {
        <div class=move || {
            if is_first_load.get() {
                if sidebar_open.get() {
                    "reset-button reset-button-expand"
                } else {
                    "reset-button reset-button-collapse reset-button-collapsed"
                }
            } else {
                if sidebar_open.get() {
                    "reset-button"
                } else {
                    "reset-button reset-button-collapsed"
                }
            }
        }>    
            <Button 
                shape=ButtonShape::Square
                icon=icondata::BiResetRegular>
            </Button>
        </div>
    }
}