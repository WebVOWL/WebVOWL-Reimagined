use crate::signals::menu_signals::SidebarOpen;
use floating_ui_leptos::{UseFloatingOptions, UseFloatingReturn, use_floating};
use leptos::prelude::*;
use thaw::{Button, ButtonShape};

#[component]
pub fn PauseButton() -> impl IntoView {
    let SidebarOpen(sidebar_open) = use_context::<SidebarOpen>().unwrap();

    view! {
        <div
            class:pause-button
            class=(
                "pause-button-collapsed",
                move || *sidebar_open.read() == false,
            )
        >
            <Button
                shape=ButtonShape::Square
                icon=icondata::AiPauseCircleOutlined
            ></Button>
        </div>
    }
}

#[component]
pub fn ResetButton() -> impl IntoView {
    let SidebarOpen(sidebar_open) = use_context::<SidebarOpen>().unwrap();

    view! {
        <div
            class:reset-button
            class=(
                "reset-button-collapsed",
                move || *sidebar_open.read() == false,
            )
        >
            <Button
                shape=ButtonShape::Square
                icon=icondata::BiResetRegular
            ></Button>
        </div>
    }
}

#[component]
pub fn ZoomInButton() -> impl IntoView {
    let SidebarOpen(sidebar_open) = use_context::<SidebarOpen>().unwrap();

    view! {
        <div
            class:zoom-in-button
            class=(
                "zoom-in-button-collapsed",
                move || *sidebar_open.read() == false,
            )
        >
            <Button
                shape=ButtonShape::Square
                class="zoom-in-button-Bob"
                icon=icondata::AiZoomInOutlined
            ></Button>
        </div>
    }
}

#[component]
pub fn ZoomOutButton() -> impl IntoView {
    let SidebarOpen(sidebar_open) = use_context::<SidebarOpen>().unwrap();

    let reference_ref = NodeRef::new();
    let floating_ref = NodeRef::new();

    let UseFloatingReturn { floating_styles } =
        use_floating(reference_ref, floating_ref, UseFloatingOptions::default());

    // TODO: https://floating-ui.rustforweb.org/frameworks/leptos.html#usage
    view! {
        <div
            class:zoom-out-button
            class=(
                "zoom-out-button-collapsed",
                move || *sidebar_open.read() == false,
            )
        >
            <Button
                shape=ButtonShape::Square
                icon=icondata::AiZoomOutOutlined
            ></Button>
        </div>
    }
}

#[component]
pub fn GraphInteractionButtons() -> impl IntoView {
    view! {
        <ZoomInButton />
        <ZoomOutButton />
        <ResetButton />
        <PauseButton />
    }
}
