use leptos::*;
use thaw::*;



#[component]
pub fn ExportMenu() -> impl IntoView {
    let on_select = Box::new(|value: &str| leptos::logging::warn!("{}", value)); 
    view! {
        <ConfigProvider>
            <Menu on_select trigger_type=MenuTriggerType::Hover position=MenuPosition::Top>
                <MenuTrigger slot>
                    <Button shape=ButtonShape::Square icon=icondata::BiExportRegular>"Export"</Button>
                </MenuTrigger>
                <Button>"Export as JSON"</Button>
                <Button>"Export as SVG"</Button>
                <Button>"Export as TeX"</Button>
                <Button>"Export as TTL"</Button>
                <Button>"Export as URL"</Button>
            </Menu>
        </ConfigProvider>
    }
}