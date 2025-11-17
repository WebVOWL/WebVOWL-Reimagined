use super::{WorkBenchButton, WorkbenchMenuItems};
use leptos::prelude::*;
use thaw::*;

pub fn NewWebVOWL() -> impl IntoView {
    // TODO: Maybe write this in a file and includestr!
    view! {
        <p>
            "WebVOWL Reimagined is an open-source ontology
            visualization tool developed to provide an enhanced
            user experience and improved performance over the original WebVOWL.
            "
        // It leverages modern web technologies to offer a more intuitive interface
        // for exploring ontologies."
        </p>
    }
}

pub fn Version() -> impl IntoView {
    view! { <p>"Version pending"</p> }
}

#[component]
pub fn AboutMenu() -> impl IntoView {
    view! {
        <Popover
            trigger_type=PopoverTriggerType::Click
            position=PopoverPosition::RightEnd
        >
            <PopoverTrigger slot>
                <WorkBenchButton
                    text="About"
                    icon=icondata::AiCopyrightOutlined
                ></WorkBenchButton>
            </PopoverTrigger>
            <WorkbenchMenuItems title="About">
                <Version />
                <NewWebVOWL />
            </WorkbenchMenuItems>
        </Popover>
    }
}
