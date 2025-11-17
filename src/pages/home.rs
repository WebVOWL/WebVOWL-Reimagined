// use crate::components::menu::right_side_bar::RightSidebar;
use crate::blocks::workbench::NewWorkbench;

use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="WebVOWL Reimagined" />
        <main class="-z-99">
            <canvas class="-z-98 size-full fixed" id="canvas" />
            <NewWorkbench />
        </main>
    }
}
