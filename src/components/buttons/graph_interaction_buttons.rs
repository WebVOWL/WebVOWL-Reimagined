use grapher::web::prelude::EVENT_DISPATCHER;
use grapher::web::prelude::RenderEvent;
use leptos::prelude::*;
use leptos_icons::*;

#[component]
pub fn PauseButton(#[prop(into)] is_sidebar_open: RwSignal<bool>) -> impl IntoView {
    let paused = RwSignal::new(false);

    let toggle_pause = move |_| {
        let currently_paused = paused.get();
        if currently_paused {
            EVENT_DISPATCHER
                .rend_chan
                .write()
                .unwrap()
                .single_write(RenderEvent::Resumed);
            paused.set(false);
        } else {
            EVENT_DISPATCHER
                .rend_chan
                .write()
                .unwrap()
                .single_write(RenderEvent::Paused);
            paused.set(true);
        }
    };

    view! {
        <div
            class="interact-0-1"
            class=("column0-collapsed", move || !is_sidebar_open.get())
        >
            <button
                class="button"
                title=move || {
                    if paused.get() {
                        "Resume the graph simulation"
                    } else {
                        "Pause the graph simulation"
                    }
                }
                on:click=toggle_pause
            >
                {move || {
                    if paused.get() {
                        view! { <Icon icon=icondata::AiPlayCircleOutlined /> }
                    } else {
                        view! { <Icon icon=icondata::AiPauseCircleOutlined /> }
                    }
                }}
            </button>
        </div>
    }
}

#[component]
pub fn ResetButton(#[prop(into)] is_sidebar_open: RwSignal<bool>) -> impl IntoView {
    view! {
        <div
            class="interact-0-0"
            class=("column0-collapsed", move || !is_sidebar_open.get())
        >
            <button class="button" title="Reset the graph position">
                <Icon icon=icondata::AiReloadOutlined />
            </button>
        </div>
    }
}

#[component]
pub fn ZoomInButton(#[prop(into)] is_sidebar_open: RwSignal<bool>) -> impl IntoView {
    view! {
        <div
            class="interact-1-1"
            class=("column1-collapsed", move || !is_sidebar_open.get())
        >
            <button
                class="button"
                title="Zoom in on the graph"
                on:click=move |_| {
                    EVENT_DISPATCHER
                        .rend_chan
                        .write()
                        .unwrap()
                        .single_write(RenderEvent::Zoomed(-20.0));
                }
            >
                <Icon icon=icondata::AiZoomInOutlined />
            </button>
        </div>
    }
}

#[component]
pub fn ZoomOutButton(#[prop(into)] is_sidebar_open: RwSignal<bool>) -> impl IntoView {
    view! {
        <div
            class="interact-1-0"
            class=("column1-collapsed", move || !is_sidebar_open.get())
        >
            <button
                class="button"
                title="Zoom out on the graph"
                on:click=move |_| {
                    EVENT_DISPATCHER
                        .rend_chan
                        .write()
                        .unwrap()
                        .single_write(RenderEvent::Zoomed(20.0));
                }
            >
                <Icon icon=icondata::AiZoomOutOutlined />
            </button>
        </div>
    }
}

#[component]
pub fn CenterGraphButton(#[prop(into)] is_sidebar_open: RwSignal<bool>) -> impl IntoView {
    view! {
        <div
            class="interact-0-2"
            class=("column0-collapsed", move || !is_sidebar_open.get())
        >
            <button class="button" title="Center the graph">
                <Icon icon=icondata::MdiImageFilterCenterFocus />
            </button>
        </div>
    }
}

#[component]
pub fn GraphInteractionButtons(#[prop(into)] is_sidebar_open: RwSignal<bool>) -> impl IntoView {
    view! {
        <CenterGraphButton is_sidebar_open />
        <ZoomInButton is_sidebar_open />
        <ZoomOutButton is_sidebar_open />
        <ResetButton is_sidebar_open />
        <PauseButton is_sidebar_open />
    }
}
