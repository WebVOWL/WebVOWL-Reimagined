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
    let repel_force: RwSignal<f64> = RwSignal::new(RepelForce::default().0.into());
    let spring_stiffness: RwSignal<f64> = RwSignal::new(SpringStiffness::default().0.into());
    let spring_neutral_length: RwSignal<f64> =
        RwSignal::new(SpringNeutralLength::default().0.into());
    let gravity_force = RwSignal::new(GravityForce::default().0.into());
    let delta_time: RwSignal<f64> = RwSignal::new(DeltaTime::default().0.into());
    let damping = RwSignal::new(Damping::default().0.into());
    let quadtree_theta: RwSignal<f64> = RwSignal::new(QuadTreeTheta::default().0.into());
    let freeze_thresh: RwSignal<f64> = RwSignal::new(FreezeThreshold::default().0.into());

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
                    // <Tooltip<_, f64> content=repel_force>
                        <Slider
                            value=repel_force
                            max=10e8
                            min=10e6
                            step=10e3
                            show_stops=false
                        />
                    // </Tooltip<_, f64>>

                </Field>
                <Field label="Edge Stiffness">
                    <Slider
                        value=spring_stiffness
                        max=600.0
                        min=100.0
                        step=100.0
                        show_stops=false
                    />
                </Field>
                <Field label="Edge Length">
                    <Slider
                        value=spring_neutral_length
                        max=120.0
                        min=20.0
                        step=10.0
                        show_stops=false
                    />
                </Field>
                <Field label="Center Gravity Strength">
                    <Slider
                        value=gravity_force
                        max=40.0
                        min=5.0
                        step=5.0
                        show_stops=false
                    />
                </Field>
                <Field label="Simulation Speed">
                    <Slider
                        value=delta_time
                        max=0.01
                        min=0.001
                        step=0.0005
                        show_stops=false
                    />
                </Field>
                <Field label="Damping">
                    <Slider
                        value=damping
                        max=0.9
                        min=0.1
                        step=0.1
                        show_stops=false
                    />
                </Field>
                <Field label="Simulation Accuracy">
                    <Slider
                        value=quadtree_theta
                        max=1.0
                        min=0.1
                        step=0.1
                        show_stops=false
                    />
                </Field>
                // FIXME: The slider of this field does not work.
                // However, it does work if you swap this field with the field above it
                //      (though the field above it stops working)
                // Apparently, thaw sliders stop working when there are more than 7 sliders in one place.
                // <Field label="Movement Threshold">
                //     <Slider
                //         value=freeze_thresh
                //         max=40.0
                //         min=5.0
                //         step=5.0
                //         show_stops=false
                //     />
                // </Field>



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
        // <Popover
        //     trigger_type=PopoverTriggerType::Click
        //     position=PopoverPosition::RightEnd
        // >
        //     <PopoverTrigger slot>
        //         <WorkBenchButton
        //             text="Settings"
        //             icon=icondata::BiMenuRegular
        //         ></WorkBenchButton>
        //     </PopoverTrigger>
            <WorkbenchMenuItems title="Settings">
                <SimulatorSettings />
            </WorkbenchMenuItems>
        // </Popover>
    }
}
