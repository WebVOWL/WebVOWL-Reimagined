use crate::components::icon::Icon;
use grapher::web::prelude::EVENT_DISPATCHER;
use grapher::web::prelude::RenderEvent;
use leptos::prelude::*;

#[component]
pub fn PauseButton() -> impl IntoView {
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
        <button
            class="w-[35px] h-[35px] bg-white flex items-center justify-center text-black border border-black cursor-pointer hover:bg-[#dd9900] transition-colors"
            title=move || {
                if paused.get() { "Resume the graph simulation" } else { "Pause the graph simulation" }
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
    }
}

#[component]
pub fn ResetButton() -> impl IntoView {
    view! {
        <button
            class="w-[35px] h-[35px] bg-white flex items-center justify-center text-black border border-black cursor-pointer hover:bg-[#dd9900] transition-colors"
            title="Reset the graph position"
        >
            <Icon icon=icondata::AiReloadOutlined />
        </button>
    }
}

#[component]
pub fn ZoomInButton() -> impl IntoView {
    view! {
        <button
            class="w-[35px] h-[35px] bg-white flex items-center justify-center text-black border border-black cursor-pointer hover:bg-[#dd9900] transition-colors"
            title="Zoom in on the graph"
            on:click=move |_| {
                EVENT_DISPATCHER
                    .rend_chan
                    .write()
                    .unwrap()
                    .single_write(RenderEvent::Zoomed(20.0));
            }
        >
            <Icon icon=icondata::AiZoomInOutlined />
        </button>
    }
}

#[component]
pub fn ZoomOutButton() -> impl IntoView {
    view! {
        <button
            class="w-[35px] h-[35px] bg-white flex items-center justify-center text-black border border-black cursor-pointer hover:bg-[#dd9900] transition-colors"
            title="Zoom out on the graph"
            on:click=move |_| {
                EVENT_DISPATCHER
                    .rend_chan
                    .write()
                    .unwrap()
                    .single_write(RenderEvent::Zoomed(-20.0));
            }
        >
            <Icon icon=icondata::AiZoomOutOutlined />
        </button>
    }
}

#[component]
pub fn CenterGraphButton() -> impl IntoView {
    view! {
        <button
            class="w-[35px] h-[35px] bg-white flex items-center justify-center text-black border border-black cursor-pointer hover:bg-[#dd9900] transition-colors"
            title="Center the graph"
            on:click=move |_| {
                EVENT_DISPATCHER
                    .rend_chan
                    .write()
                    .unwrap()
                    .single_write(RenderEvent::CenterGraph);
            }
        >
            <Icon icon=icondata::MdiImageFilterCenterFocus />
        </button>
    }
}

#[component]
pub fn GraphInteractionButtons(#[prop(into)] is_sidebar_open: RwSignal<bool>) -> impl IntoView {
    view! {
        <div
            class="absolute bottom-[1%] flex gap-1 transition-[right] duration-500 ease-in-out"
            class=("right-[23%]", move || is_sidebar_open.get())
            class=("right-[1%]", move || !is_sidebar_open.get())
        >
            <div class="flex flex-col gap-1 justify-end">
                <ZoomInButton />
                <ZoomOutButton />
            </div>
            <div class="flex flex-col gap-1">
                <CenterGraphButton />
                <PauseButton />
                <ResetButton />
            </div>
        </div>
    }
}
