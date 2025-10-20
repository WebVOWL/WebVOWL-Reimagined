use leptos::prelude::*;
use thaw::*;
use crate::pages::home::*;



use leptos::prelude::*;
use thaw::*;
use crate::pages::home::*;

#[component]
pub fn OptionsMenu() -> impl IntoView {
    let ShowOptionsMenu(show_options_menu) = use_context::<ShowOptionsMenu>().expect("ShowOptionsMenu should be provided");
    let search_query = RwSignal::new(String::new());
    let class_distance_value = RwSignal::new(0.0);
    let datatype_distance_value = RwSignal::new(0.0);
    let dynamic_label_check = RwSignal::new(false);
    let label_width_value = RwSignal::new(0.0);
    let node_scaling_check = RwSignal::new(false);
    let compact_notation_check = RwSignal::new(false);
    let color_externals_check = RwSignal::new(false);
    view! {
        <div class=move || {
        if show_options_menu.get() {
            "workbench-menu"
        } else {
            "workbench-menu menu-hidden"
        }
        }>
            <div class="workbench-menu-header">
                <h3>"Options"</h3>
            </div>
            <div class="workbench-menu-content">
                <p class="workbench-menu-text">"Class Distance"</p>
                <Slider class="workbench-slider" value=class_distance_value max=600.0 step=10.0 show_stops=false />
                <p class="workbench-menu-text">"Datatype Distance"</p>
                <Slider class="workbench-slider" value=datatype_distance_value max=600.0 step=10.0 show_stops=false />
                <Checkbox class="workbench-checkbox" checked=dynamic_label_check label="Dynamic labels" />
                <p class="workbench-menu-text">"Max label width"</p>
                <Slider class="workbench-slider" value=label_width_value max=600.0 step=10.0 show_stops=false />
                <Checkbox class="workbench-checkbox" checked=node_scaling_check label="Node scaling" />
                <Checkbox class="workbench-checkbox" checked=compact_notation_check label="Compact notation" />
                <Checkbox class="workbench-checkbox" checked=color_externals_check label="Color externals" />

            </div>
        </div>
    }
}