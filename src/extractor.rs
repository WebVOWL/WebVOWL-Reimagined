use leptos::prelude::window_event_listener;
use leptos::prelude::*;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{Storage, window};


/// Documentation for [``]
#[component]
pub fn LocalCookies() -> impl IntoView {
    let user_id = RwSignal::new(String::new());

    window_event_listener(leptos::ev::load, {
        let user_id = user_id.clone();
        move |_| {
            if let Some(win) = window() {
                if let Ok(Some(storage)) = win.local_storage() {
                    if let Ok(Some(id)) = storage.get_item("user_id") {
                        user_id.set(id);
                    }
                }
            }
        }
    });

    Effect::new({
        let user_id = user_id.clone();
        move |_| {
            if let Some(win) = window() {
                if let Ok(Some(storage)) = win.local_storage() {
                    let _ = storage.set_item("user_id", &user_id.get());
                }
            }
        }
    });

    view! {
        <div style="display: flex; flex-direction: column; gap: 0.5rem;">
            <h3>"Your User ID"</h3>
            <input
                type="text"
                prop:value=user_id
                placeholder="Enter your unique ID"
                on:input=move |ev| user_id.set(event_target_value(&ev))
            ></input>
            <button on:click=move |_| {
                
            }>"Sync to Server"</button>
        </div>
    }
}
