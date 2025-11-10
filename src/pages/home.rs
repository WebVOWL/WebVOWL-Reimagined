use crate::components::menu::right_side_bar::RightSidebar;
use crate::components::menu::workbench::Workbench;

use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="WebVOWL Reimagined" />
        <main>
            <canvas id="canvas" />
            <RightSidebar />
            <Workbench />
        </main>
    }
}
