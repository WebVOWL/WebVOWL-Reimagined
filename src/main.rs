use actix_files::Files;
use actix_session::{SessionMiddleware, storage::RedisSessionStore};
use actix_web::*;
use actix_web::cookie::Key;
use env_logger::Env;
use leptos::prelude::*;
use leptos_actix::{LeptosRoutes, generate_route_list};
use leptos_meta::MetaTags;
use log::info;
use webvowl_reimagined::app::App;
use webvowl_reimagined::hydration_scripts::HydrationScripts as Hydro;
use webvowl_reimagined::session_handler::index;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();

    let pkg_name = env!("CARGO_PKG_NAME");
    let pkg_version = env!("CARGO_PKG_VERSION");
    info!("Starting {pkg_name} server [v{pkg_version}]");

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    // When using `Key::generate()` it is important to initialize outside of the
    // `HttpServer::new` closure. When deployed the secret key should be read from a
    // configuration file or environment variables.
    let secret_key = Key::generate();

    let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();



    HttpServer::new(move || {
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;
        

        App::new()
            // Add session management to your application using Redis for session state storage
            .wrap(
                SessionMiddleware::new(
                    redis_store.clone(),
                    secret_key.clone(),
                )
            )
            .route("/counter", web::get().to(index))
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
                                <AutoReload options=leptos_options.clone() />
                                <Hydro options=leptos_options.clone() />
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
                    .add(("Cross-Origin-Opener-Policy", "same-origin"))
                    .add(("Cross-Origin-Embedder-Policy", "require-corp")),
            )
    })
    .bind(&addr)?
    .run()
    .await
}
