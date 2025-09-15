use leptos::serde_json;
use leptos::{IntoView, WasmSplitManifest, component, config::LeptosOptions, prelude::*, view};
use std::{path::PathBuf, sync::OnceLock};

/// Inserts hydration scripts that add interactivity to your server-rendered HTML.
///
/// This should be included in the `<head>` of your application shell.
#[component]
pub fn HydrationScripts(
    /// Configuration options for this project.
    options: LeptosOptions,
    /// Should be `true` to hydrate in `islands` mode.
    #[prop(optional)]
    islands: bool,
    /// Should be `true` to add the “islands router,” which enables limited client-side routing
    /// when running in islands mode.
    #[prop(optional)]
    islands_router: bool,
    /// A base url, not including a trailing slash
    #[prop(optional, into)]
    root: Option<String>,
) -> impl IntoView {
    static SPLIT_MANIFEST: OnceLock<Option<WasmSplitManifest>> = OnceLock::new();

    if let Some(splits) = SPLIT_MANIFEST.get_or_init(|| {
        let root = root.clone().unwrap_or_default();

        let site_dir = &options.site_root;
        let pkg_dir = &options.site_pkg_dir;
        let path = PathBuf::from(site_dir.to_string());
        let path = path
            .join(pkg_dir.to_string())
            .join("__wasm_split_manifest.json");
        let file = std::fs::read_to_string(path).ok()?;
        let manifest = WasmSplitManifest(ArcStoredValue::new((
            format!("{root}/{pkg_dir}"),
            serde_json::from_str(&file).expect("could not read manifest file"),
        )));

        Some(manifest)
    }) {
        provide_context(splits.clone());
    }

    let mut js_file_name = options.output_name.to_string();
    let mut wasm_file_name = options.output_name.to_string();
    if options.hash_files {
        let hash_path = std::env::current_exe()
            .map(|path| path.parent().map(|p| p.to_path_buf()).unwrap_or_default())
            .unwrap_or_default()
            .join(options.hash_file.as_ref());
        if hash_path.exists() {
            let hashes = std::fs::read_to_string(&hash_path).expect("failed to read hash file");
            for line in hashes.lines() {
                let line = line.trim();
                if !line.is_empty() {
                    if let Some((file, hash)) = line.split_once(':') {
                        if file == "js" {
                            js_file_name.push_str(&format!(".{}", hash.trim()));
                        } else if file == "wasm" {
                            wasm_file_name.push_str(&format!(".{}", hash.trim()));
                        }
                    }
                }
            }
        } else {
            leptos::logging::error!("File hashing is active but no hash file was found");
        }
    } else if std::option_env!("LEPTOS_OUTPUT_NAME").is_none() {
        wasm_file_name.push_str("_bg");
    }

    let pkg_path = &options.site_pkg_dir;
    #[cfg(feature = "nonce")]
    let nonce = crate::nonce::use_nonce();
    #[cfg(not(feature = "nonce"))]
    let nonce = None::<String>;
    let script = if islands {
        if let Some(sc) = Owner::current_shared_context() {
            sc.set_is_hydrating(false);
        }
        include_str!("../public/js/island_script.js")
    } else {
        include_str!("../public/js/hydration_script.js")
    };

    let islands_router = islands_router
        .then_some(include_str!("../public/js/islands_routing.js"))
        .unwrap_or_default();

    let root = root.unwrap_or_default();
    view! {
        <link
            rel="modulepreload"
            href=format!("{root}/{pkg_path}/{js_file_name}.js")
            crossorigin=nonce.clone()
        />
        <link
            rel="preload"
            href=format!("{root}/{pkg_path}/{wasm_file_name}.wasm")
            r#as="fetch"
            r#type="application/wasm"
            crossorigin=nonce.clone().unwrap_or_default()
        />
        <script type="module" nonce=nonce>
            {format!(
                "{script}({root:?}, {pkg_path:?}, {js_file_name:?}, {wasm_file_name:?});{islands_router}",
            )}
        </script>
    }
}
