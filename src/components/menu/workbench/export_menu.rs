use leptos::prelude::*;
use thaw::*;
use crate::pages::home::*;

#[component]
pub fn ExportMenu() -> impl IntoView {
    let ShowExportMenu(show_export_menu) = use_context::<ShowExportMenu>().expect("ShowExportMenu should be provided");
    let on_select = Box::new(|value: &str| leptos::logging::warn!("{}", value));
    view! {
        <div class=move || {
        if show_export_menu.get() {
            "workbench-menu"
        } else {
            "workbench-menu menu-hidden"
        }
        }>
            <div class="workbench-menu-header">
                <h3>"Export ontology"</h3>
            </div>
            <div class="workbench-menu-content">
                <ConfigProvider>
                    <Button class="export-button">
                        "JSON"
                    </Button>
                    <Button class="export-button">
                        "SVG"
                    </Button>
                    <Button class="export-button">
                        "TeX"
                    </Button>
                    <Button class="export-button">
                        "TTL"
                    </Button>
                    <Button class="export-button">
                        "URL"
                    </Button>
                </ConfigProvider>
            </div>
        </div>
    }
}