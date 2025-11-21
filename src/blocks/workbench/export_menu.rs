use crate::components::icon::Icon;
use super::WorkbenchMenuItems;
use leptos::prelude::*;


#[component]
pub fn ExportButton(
    #[prop(into)] label: String,
    #[prop(into)] icon: icondata::Icon,
    #[prop(optional)] on_click: Option<Callback<()>>

) -> impl IntoView {
    let onclick_handler = move |_| {
        if let Some(cb) = &on_click {
            cb.run(());
        }
    };

    view! {
        <button 
            class="relative flex items-center justify-center gap-1 h-10 w-40 m-2 bg-gray-200 text-[#000000] rounded-sm font-semibold"
            on:click=onclick_handler
        >
            <Icon icon=icon />
            {label}
        </button>
    }
}

#[component]
pub fn ExportMenu() -> impl IntoView {

    view! {
        <WorkbenchMenuItems title="Export Ontology">
            <div class="flex justify-center flex-wrap w-full">
                <ExportButton label="Json" icon=icondata::AiDownloadOutlined />
                <ExportButton label="SVG" icon=icondata::AiDownloadOutlined />
                <ExportButton label="TeX" icon=icondata::AiDownloadOutlined />
                <ExportButton label="TTL" icon=icondata::AiDownloadOutlined />
                <ExportButton label="URL" icon=icondata::AiDownloadOutlined />
                <ExportButton label="RDF" icon=icondata::AiDownloadOutlined />
                <ExportButton label="OWL" icon=icondata::AiDownloadOutlined />
            </div>
        </WorkbenchMenuItems>
    }
}