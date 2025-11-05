use super::{WorkBenchButton, WorkbenchMenuItems};
use leptos::prelude::*;
use thaw::*;

#[component]
pub fn SimulatorSettings() -> impl IntoView {
    let search_query = RwSignal::new(String::new());
    let class_distance_value = RwSignal::new(0.0);
    let datatype_distance_value = RwSignal::new(0.0);
    let dynamic_label_check = RwSignal::new(false);
    let label_width_value = RwSignal::new(0.0);
    let node_scaling_check = RwSignal::new(false);
    let compact_notation_check = RwSignal::new(false);
    let color_externals_check = RwSignal::new(false);

    view! {
        <fieldset>
            <legend>"Simulation (WIP)"</legend>
            <Flex gap=FlexGap::Size(20) vertical=true>
                <Field label="Class Distance">
                    <Slider
                        value=class_distance_value
                        max=600.0
                        step=10.0
                        show_stops=false
                    />
                </Field>
                <Field label="Datatype Distance">
                    <Slider
                        value=datatype_distance_value
                        max=600.0
                        step=10.0
                        show_stops=false
                    />
                </Field>

                <Checkbox checked=dynamic_label_check label="Dynamic labels" />
                <Checkbox checked=node_scaling_check label="Node scaling" />
                <Checkbox
                    checked=compact_notation_check
                    label="Compact notation"
                />
                <Checkbox
                    checked=color_externals_check
                    label="Color externals"
                />
                <Field label="Max label width">
                    <Slider
                        value=label_width_value
                        max=600.0
                        step=10.0
                        show_stops=false
                    />
                </Field>
            </Flex>
        </fieldset>
    }
}

#[component]
pub fn OptionsMenu() -> impl IntoView {
    view! {
        <Popover
            trigger_type=PopoverTriggerType::Click
            position=PopoverPosition::RightEnd
        >
            <PopoverTrigger slot>
                <WorkBenchButton
                    text="Settings"
                    icon=icondata::BiMenuRegular
                ></WorkBenchButton>
            </PopoverTrigger>
            <WorkbenchMenuItems title="Settings">
                <SimulatorSettings />
            </WorkbenchMenuItems>
        </Popover>
    }
}
