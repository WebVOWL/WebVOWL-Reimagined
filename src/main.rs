mod app;

use crate::app::App;
use actix_files::Files;
use actix_web::*;
use leptos::prelude::*;
use leptos_actix::{LeptosRoutes, generate_route_list};
use leptos_meta::MetaTags;

// // Use mimalloc as memory allocator when running containerized
// // to avoid poor performance
// #[cfg(target_env = "musl")]
// use mimalloc::MiMalloc;

// #[cfg(target_env = "musl")]
// #[global_allocator]
// static GLOBAL: MiMalloc = MiMalloc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    HttpServer::new(move || {
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    use leptos::prelude::*;

                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8" />
                                <meta description="WebVOWL rebuilt from stratch with a strong focus on performance and scalability" />
                                <meta
                                    name="viewport"
                                    content="width=device-width, initial-scale=1"
                                />
                                <meta apple-mobile-web-app-capable="yes" />
                                // <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone() />
                                <MetaTags />
                            </head>
                            <body>
                                <App />
                            </body>
                        </html>
                    }
                }
            })
            .service(Files::new("/", site_root.as_ref()))
            .wrap(middleware::Compress::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("Cross-Origin-Opener-Policy", "same-origin")),
            )
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("Cross-Origin-Embedder-Policy", "require-corp")),
            )
    })
    .bind(&addr)?
    .run()
    .await
}
