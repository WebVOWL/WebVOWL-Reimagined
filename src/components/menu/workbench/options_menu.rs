use super::{WorkBenchButton, WorkbenchMenuItems};
use grapher::web::prelude::*;
use leptos::prelude::*;
use thaw::*;

#[component]
pub fn SimulatorSettings() -> impl IntoView {
    // let class_distance_value = RwSignal::new(0.0);
    // let datatype_distance_value = RwSignal::new(0.0);
    // let dynamic_label_check = RwSignal::new(false);
    // let label_width_value = RwSignal::new(0.0);
    // let node_scaling_check = RwSignal::new(false);
    // let compact_notation_check = RwSignal::new(false);
    // let color_externals_check = RwSignal::new(false);
    let repel_force: RwSignal<f64> = RwSignal::new(10e7);
    let spring_stiffness = RwSignal::new(300.0);
    let spring_neutral_length = RwSignal::new(70.0);
    let gravity_force = RwSignal::new(30.0);
    let delta_time = RwSignal::new(0.005);
    let damping = RwSignal::new(0.8);
    let quadtree_theta = RwSignal::new(0.8);
    let freeze_thresh = RwSignal::new(10.0);

    Effect::new(move |_| {
        let mut sim_chan = EVENT_DISPATCHER.sim_chan.write().unwrap();
        sim_chan.iter_write([
            SimulatorEvent::RepelForceUpdated(repel_force.get() as f32),
            SimulatorEvent::SpringStiffnessUpdated(spring_stiffness.get() as f32),
            SimulatorEvent::SpringNeutralLengthUpdated(spring_neutral_length.get() as f32),
            SimulatorEvent::GravityForceUpdated(gravity_force.get() as f32),
            SimulatorEvent::DeltaTimeUpdated(delta_time.get() as f32),
            SimulatorEvent::DampingUpdated(damping.get() as f32),
            SimulatorEvent::QuadTreeThetaUpdated(quadtree_theta.get() as f32),
            SimulatorEvent::FreezeThresholdUpdated(freeze_thresh.get() as f32),
        ]);
    });

    view! {}

    view! {
        <fieldset>
            <legend>"Graph Simulation"</legend>
            <Flex gap=FlexGap::Size(20) vertical=true>
                <Field label="Node Distance">
                    <Tooltip<_, f64> content=repel_force>
                        <Slider
                            value=repel_force
                            max=10e9
                            min=10e5
                            step=10e3
                            show_stops=false
                        />
                    </Tooltip<_, f64>>

                </Field>
                <Field label="Edge Stiffness">
                    <Slider
                        value=spring_stiffness
                        max=600.0
                        min=50.0
                        step=50.0
                        show_stops=false
                    />
                </Field>
                <Field label="Edge Length">
                    <Slider
                        value=spring_neutral_length
                        max=200.0
                        min=20.0
                        step=20.0
                        show_stops=false
                    />
                </Field>
                <Field label="Center Gravity Strength">
                    <Slider
                        value=gravity_force
                        max=100.0
                        min=0.0
                        step=10.0
                        show_stops=false
                    />
                </Field>
                <Field label="Simulation Speed">
                    <Slider
                        value=delta_time
                        max=0.1
                        min=0.0005
                        step=0.0005
                        show_stops=false
                    />
                </Field>
                <Field label="Damping">
                    <Slider
                        value=damping
                        max=1.0
                        min=0.0
                        step=0.1
                        show_stops=false
                    />
                </Field>
                <Field label="Simulation Accuracy">
                    <Slider
                        value=quadtree_theta
                        max=0.8
                        min=0.0
                        step=0.1
                        show_stops=false
                    />
                </Field>
                <Field label="Movement Threshold">
                    <Slider
                        value=freeze_thresh
                        max=50.0
                        min=10.0
                        step=5.0
                        show_stops=false
                    />
                </Field>
            // <Field label="Class Distance">
            // <Slider
            // value=class_distance_value
            // max=600.0
            // step=10.0
            // show_stops=false
            // />
            // </Field>
            // <Field label="Datatype Distance">
            // <Slider
            // value=datatype_distance_value
            // max=600.0
            // step=10.0
            // show_stops=false
            // />
            // </Field>

            // <Checkbox checked=dynamic_label_check label="Dynamic labels" />
            // <Checkbox checked=node_scaling_check label="Node scaling" />
            // <Checkbox
            // checked=compact_notation_check
            // label="Compact notation"
            // />
            // <Checkbox
            // checked=color_externals_check
            // label="Color externals"
            // />
            // <Field label="Max label width">
            // <Slider
            // value=label_width_value
            // max=600.0
            // step=10.0
            // show_stops=false
            // />
            // </Field>
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
