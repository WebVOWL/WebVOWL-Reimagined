use leptos::*;
use leptos::prelude::RwSignal;
//use thaw::*;
use leptos_shadcn_slider::Slider;

#[component]
pub fn ZoomSlider() -> impl IntoView {
    let slider_value = RwSignal::new(0.0);
    view! {
            <Slider 
                min=0.0 
                max=100.0 
                step=1.0
                value=slider_value 
            />
    }
}
