use leptos::prelude::*;
use thaw::*;
use crate::pages::home::*;

#[component]
pub fn PauseButton() -> impl IntoView {
    let SidebarOpen(sidebar_open) = use_context::<SidebarOpen>().expect("SidebarOpen should be provided");
    let IsFirstLoad(is_first_load) = use_context::<IsFirstLoad>().expect("IsFirstLoad should be provided");
    view! {
        <div class=move || {
            if is_first_load.get() {
                if sidebar_open.get() {
                    "pause-button pause-button-expand"
                } else {
                    "pause-button pause-button-collapse pause-button-collapsed"
                }
            } else {
                if sidebar_open.get() {
                    "pause-button"
                } else {
                    "pause-button pause-button-collapsed"
                }
            }
        }>    
            <Button 
                shape=ButtonShape::Square
                icon=icondata::AiPauseCircleOutlined>
            </Button>
        </div>
    }
}