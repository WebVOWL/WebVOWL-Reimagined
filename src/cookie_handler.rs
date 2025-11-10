use leptos::prelude::*;
use uuid::Uuid;
use web_sys::{Storage, window};

#[component]
pub fn LocalCookies() -> impl IntoView {
    let user_id = RwSignal::new(String::new());

    Effect::new({
        let user_id = user_id.clone();
        move |_| {
            if let Some(win) = window() {
                if let Ok(Some(storage)) = win.local_storage() {
                    match storage.get_item("user_id") {
                        Ok(Some(id)) if !id.is_empty() => {
                            user_id.set(id);
                        }
                        _ => {
                            let new_id = Uuid::new_v4().to_string();
                            let _ = storage.set_item("user_id", &new_id);
                            user_id.set(new_id);
                        }
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
            <h3>Your Cookie</h3>
            <text>{user_id}</text>
            <button on:click=move |_| {
                if let Some(win) = window() {
                    if let Ok(Some(storage)) = win.local_storage() {
                        let _ = storage.remove_item("user_id");
                        win.location().reload().unwrap();
                    }
                }
            }>Clear Cookie</button>
        </div>
    }
}
