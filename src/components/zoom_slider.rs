use leptos::prelude::*;
use thaw::*;
use crate::pages::home::*;

#[component]
pub fn ZoomSlider() -> impl IntoView {
    let zoom_level = RwSignal::new(1.0);
    let SidebarOpen(sidebar_open) = use_context::<SidebarOpen>().expect("SidebarOpen should be provided");
    let IsFirstLoad(is_first_load) = use_context::<IsFirstLoad>().expect("IsFirstLoad should be provided");
    
    view! {
        <div class=move || {
            if is_first_load.get() {
                if sidebar_open.get() {
                    "zoom-slider zoom-slider-expand"
                } else {
                    "zoom-slider zoom-slider-collapse zoom-slider-collapsed"
                }
            } else {
                if sidebar_open.get() {
                    "zoom-slider"
                } else {
                    "zoom-slider zoom-slider-collapsed"
                }
            }
        }>
            <ConfigProvider>
                <Slider
                    min=0.0
                    max=100.0
                    step=1.0
                    value=zoom_level
                    show_stops=false
                    vertical=true
                />
            </ConfigProvider>
        </div>
    }
}
