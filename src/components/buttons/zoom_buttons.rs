use leptos::prelude::*;
use thaw::*;
use crate::pages::home::*;

#[component]
pub fn ZoomInButton() -> impl IntoView {
    let SidebarOpen(sidebar_open) = use_context::<SidebarOpen>().expect("SidebarOpen should be provided");
    let IsFirstLoad(is_first_load) = use_context::<IsFirstLoad>().expect("IsFirstLoad should be provided");
    view! {
        <div class=move || {
            if is_first_load.get() {
                if sidebar_open.get() {
                    "zoom-in-button zoom-in-button-expand"
                } else {
                    "zoom-in-button zoom-in-button-collapse zoom-in-button-collapsed"
                }
            } else {
                if sidebar_open.get() {
                    "zoom-in-button"
                } else {
                    "zoom-in-button zoom-in-button-collapsed"
                }
            }
        }>    
            <Button 
                shape=ButtonShape::Square
                class="zoom-in-button-Bob"
                icon=icondata::AiZoomInOutlined>
            </Button>
        </div>
    }
}

#[component]
pub fn ZoomOutButton() -> impl IntoView {
    let SidebarOpen(sidebar_open) = use_context::<SidebarOpen>().expect("SidebarOpen should be provided");
    let IsFirstLoad(is_first_load) = use_context::<IsFirstLoad>().expect("IsFirstLoad should be provided");
    view! {
        <div class=move || {
            if is_first_load.get() {
                if sidebar_open.get() {
                    "zoom-out-button zoom-out-button-expand"
                } else {
                    "zoom-out-button zoom-out-button-collapse zoom-out-button-collapsed"
                }
            } else {
                if sidebar_open.get() {
                    "zoom-out-button"
                } else {
                    "zoom-out-button zoom-out-button-collapsed"
                }
            }
        }>    
            <Button 
                shape=ButtonShape::Square
                icon=icondata::AiZoomOutOutlined>
            </Button>
        </div>
    }
}

