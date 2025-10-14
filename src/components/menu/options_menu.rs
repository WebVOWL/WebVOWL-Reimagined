use leptos::prelude::*;
use thaw::*;



#[component]
pub fn OptionsMenu() -> impl IntoView {
    let zoomcheck = RwSignal::new(false);
    let classdistancevalue = RwSignal::new(0.0);
    let datatypedistancevalue = RwSignal::new(0.0);
    let dynamiclabelcheck = RwSignal::new(false);
    let maxlabelwidthvalue = RwSignal::new(0.0);
    let nodescalingcheck = RwSignal::new(false);
    let compactnotationcheck = RwSignal::new(false);
    let colorexternalscheck = RwSignal::new(false);
    let on_select = Box::new(|value: &str| leptos::logging::warn!("{}", value)); 
    view! {
        <ConfigProvider>
            <Menu on_select trigger_type=MenuTriggerType::Hover position=MenuPosition::Top>
                <MenuTrigger slot>
                    <Button shape=ButtonShape::Square icon=icondata::IoOptions>"Options"</Button>
                </MenuTrigger>
                <Checkbox checked=zoomcheck label="Zoom controls" />
                <Slider value=classdistancevalue max=600.0 step=10.0 show_stops=false />
                <Label>"Class Distance"</Label>
                <Slider value=datatypedistancevalue max=600.0 step=10.0 show_stops=false />
                <Label>"Datatype Distance"</Label>
                <Checkbox checked=dynamiclabelcheck label="Dynamic labels" />
                <Slider value=maxlabelwidthvalue max=600.0 step=10.0 show_stops=false />
                <Label>"Max label width"</Label>
                <Checkbox checked=nodescalingcheck label="Node scaling" />
                <Checkbox checked=compactnotationcheck label= "Compact notation" />
                <Checkbox checked=colorexternalscheck label="Color externals" />
            </Menu>
        </ConfigProvider>
    }
}