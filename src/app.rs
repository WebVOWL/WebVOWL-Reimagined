use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{FlatRoutes, Route, Router},
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="webvowl" href="/pkg/webvowl.css" />
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico" />
        <Router>
            <FlatRoutes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home />
            </FlatRoutes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (value, set_value) = signal(0);

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
        <Title text="Leptos + Tailwindcss" />
        <main>
            <div class="flex flex-col min-h-screen font-mono text-white bg-gradient-to-tl from-blue-800 to-blue-500">
                <div class="flex flex-row-reverse flex-wrap m-auto">
                    <button
                        on:click=move |_| set_value.update(|value| *value += 1)
                        class="py-2 px-3 m-1 text-white bg-blue-700 rounded border-l-2 border-b-4 border-blue-800 shadow-lg"
                    >
                        "+"
                    </button>
                    <button class="py-2 px-3 m-1 text-white bg-blue-800 rounded border-l-2 border-b-4 border-blue-900 shadow-lg">
                        {value}
                    </button>
                    <button
                        on:click=move |_| set_value.update(|value| *value -= 1)
                        class="py-2 px-3 m-1 text-white bg-blue-700 rounded border-l-2 border-b-4 border-blue-800 shadow-lg"
                        class:invisible=move || { value.get() < 1 }
                    >
                        "-"
                    </button>
                </div>
            </div>
        </main>
    }
}
