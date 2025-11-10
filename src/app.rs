use crate::pages::home::Home;
use leptos::prelude::*;
use leptos_meta::Link;
use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{FlatRoutes, Route, Router},
};
use thaw::{ConfigProvider, LoadingBarProvider, ToasterProvider};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="webvowl" href="/pkg/webvowl.css" />
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico" />
        <ConfigProvider>
            <ToasterProvider>
                <LoadingBarProvider>
                    <TheRouter />
                </LoadingBarProvider>
            </ToasterProvider>
        </ConfigProvider>
    }
}

#[component]
pub fn TheRouter() -> impl IntoView {
    view! {
        <Router>
            <FlatRoutes fallback=|| "404 - Page not found.">
                <Route path=StaticSegment("") view=Home />
            </FlatRoutes>
        </Router>
    }
}
