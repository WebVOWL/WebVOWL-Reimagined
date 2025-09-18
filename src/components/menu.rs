use leptos::prelude::*;
use leptos_meta::*;
use thaw::*;


#[component]
pub fn OptionsMenu(
    zoomcheck: RwSignal<bool>,
    dynamiclabelcheck: RwSignal<bool>,
    nodescalingcheck: RwSignal<bool>,
    compactnotationcheck: RwSignal<bool>,
    colorexternalscheck: RwSignal<bool>,
    classdistancevalue: RwSignal<f64>,
    datatypedistancevalue: RwSignal<f64>,
    maxlabelwidthvalue: RwSignal<f64>,
) -> impl IntoView {
    view! {
        <Menu on_select position=MenuPosition::Top>
            <MenuTrigger slot>
                <Button>"Options"</Button>
            </MenuTrigger>
            <Checkbox checked=zoomcheck label="Zoom controls" />
            <Slider value=classdistancevalue max=600.0 />
            <Label>"Class Distance"</Label>
            <Slider value=datatypedistancevalue max=600.0 />
            <Label>"Datatype Distance"</Label>
            <Checkbox checked=dynamiclabelcheck label="Dynamic labels" />
            <Slider value=maxlabelwidthvalue max=600.0 />
            <Label>"Max label width"</Label>
            <Checkbox checked=nodescalingcheck label="Node scaling" />
            <Checkbox checked=compactnotationcheck label="Compact notation" />
            <Checkbox checked=colorexternalscheck label="Color externals" />
        </Menu>
    }
}