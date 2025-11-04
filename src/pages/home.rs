use crate::components::menu::right_side_bar::RightSidebar;
use crate::components::menu::workbench::Workbench;
use crate::components::theme::ThemeSelection;
use leptos::prelude::*;
use leptos_meta::*;
use thaw::{ConfigProvider, Layout};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="WebVOWL Reimagined" />
        <main>
            <canvas id="canvas" />

            <ThemeSelection />
            <RightSidebar />
            <Workbench />
        </main>
    }
}
