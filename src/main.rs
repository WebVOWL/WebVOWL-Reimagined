use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button
            on:click=move |_| {
                *set_count.write() += 1;
            }
        >
            {move || if count.get() == 0 {
                "Click me".to_string()
            } else {
                count.get().to_string()
            }}
        </button>
    }
}

fn main() {
    leptos::mount::mount_to_body(App)
}
