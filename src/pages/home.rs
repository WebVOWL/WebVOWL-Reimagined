use crate::components::menu::right_side_bar::RightSidebar;
use crate::components::menu::workbench::Workbench;
use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Leptos + Tailwindcss" />
        <main>
            <canvas id="canvas" />
            <div class="min-h-screen bg-[rgba(201, 196, 196, 1)]">
                <RightSidebar />
                <Workbench />
            </div>
        </main>
    }
}
