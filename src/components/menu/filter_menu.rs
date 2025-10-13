use leptos::prelude::*;
use thaw::*;


#[component]pub fn FilterMenu() -> impl IntoView {
    let datatypepropertycheck = RwSignal::new(false);
    let objectpropertycheck = RwSignal::new(false);
    let solitarypropertycheck = RwSignal::new(false);
    let classdisjointnesscheck = RwSignal::new(false);
    let setoperatorscheck = RwSignal::new(false);
    let degreeofcollapsingcheck = RwSignal::new(0.0);
    let on_select = Box::new(|value: &str| leptos::logging::warn!("{}", value)); 
    view! {
        <ConfigProvider>
            <Menu on_select trigger_type=MenuTriggerType::Hover position=MenuPosition::Top>
                <MenuTrigger slot>
                    <Button shape=ButtonShape::Square icon=icondata::BiFilterAltRegular>"Filter"</Button>
                </MenuTrigger>
                <Checkbox checked=datatypepropertycheck label="Datatype properties" />
                <Checkbox checked=objectpropertycheck label="Object properties" />
                <Checkbox checked=solitarypropertycheck label="Solitary properties" />
                <Checkbox checked=classdisjointnesscheck label="Class disjointness" />
                <Checkbox checked=setoperatorscheck label="Set operators" />
                <Slider value=degreeofcollapsingcheck max=16.0 step=1.0 show_stops=false />
                <Label>"Degree of collapsing"</Label>
            </Menu>
        </ConfigProvider>
    }
}