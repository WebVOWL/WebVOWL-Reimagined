use crate::signals::menu_signals::SidebarOpen;
use leptos::prelude::*;
use log::info;
use thaw::{Button, ButtonAppearance, ButtonShape, ButtonType, ConfigProvider, Tooltip};

#[component]
pub fn PauseButton() -> impl IntoView {
    let SidebarOpen(sidebar_open) = use_context::<SidebarOpen>().unwrap();

    view! {
        <div
            class="interact-0-1"
            class=("column0-collapsed", move || *sidebar_open.read() == false)
        >
            <ConfigProvider>
                <Tooltip content="Pause simulation">
                    <Button
                        class="button"
                        appearance=ButtonAppearance::Secondary
                        shape=ButtonShape::Rounded
                        icon=icondata::AiPauseCircleOutlined
                    ></Button>
                </Tooltip>
            </ConfigProvider>
        </div>
    }
}

#[component]
pub fn ResetButton() -> impl IntoView {
    let SidebarOpen(sidebar_open) = use_context::<SidebarOpen>().unwrap();

    view! {
        <div
            class="interact-0-0"
            class=("column0-collapsed", move || *sidebar_open.read() == false)
        >
            <ConfigProvider>
                <Tooltip content="Reset graph">
                    <Button
                        class="button"
                        appearance=ButtonAppearance::Secondary
                        shape=ButtonShape::Rounded
                        icon=icondata::BiResetRegular
                    ></Button>
                </Tooltip>
            </ConfigProvider>
        </div>
    }
}

#[component]
pub fn ZoomInButton() -> impl IntoView {
    let SidebarOpen(sidebar_open) = use_context::<SidebarOpen>().unwrap();

    view! {
        <div
            class="interact-1-1"
            class=("column1-collapsed", move || *sidebar_open.read() == false)
        >
            <ConfigProvider>
                <Tooltip content="Zoom in">
                    <Button
                        class="button"
                        appearance=ButtonAppearance::Secondary
                        shape=ButtonShape::Rounded
                        icon=icondata::AiZoomInOutlined
                    ></Button>
                </Tooltip>
            </ConfigProvider>
        </div>
    }
}

#[component]
pub fn ZoomOutButton() -> impl IntoView {
    let SidebarOpen(sidebar_open) = use_context::<SidebarOpen>().unwrap();

    view! {
        <div
            class="interact-1-0"
            class=("column1-collapsed", move || *sidebar_open.read() == false)
        >
            <ConfigProvider>
                <Tooltip content="Zoom out">
                    <Button
                        class="button"
                        appearance=ButtonAppearance::Secondary
                        shape=ButtonShape::Rounded
                        icon=icondata::AiZoomOutOutlined
                    ></Button>
                </Tooltip>
            </ConfigProvider>
        </div>
    }
}

#[component]
pub fn CenterGraphButton() -> impl IntoView {
    let SidebarOpen(sidebar_open) = use_context::<SidebarOpen>().unwrap();

    view! {
        <div
            class="interact-0-2"
            class=("column0-collapsed", move || *sidebar_open.read() == false)
        >
            <ConfigProvider>
                <Tooltip content="Fit graph to screen">
                    <Button
                        class="button"
                        appearance=ButtonAppearance::Secondary
                        shape=ButtonShape::Rounded
                        icon=icondata::MdiImageFilterCenterFocus
                    ></Button>
                </Tooltip>
            </ConfigProvider>
        </div>
    }
}

#[component]
pub fn GraphInteractionButtons() -> impl IntoView {
    view! {
        <CenterGraphButton />
        <ZoomInButton />
        <ZoomOutButton />
        <ResetButton />
        <PauseButton />
    }
}
