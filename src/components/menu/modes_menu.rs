use leptos::prelude::*;
use thaw::*;



#[component]
pub fn ModesMenu() -> impl IntoView {
    let on_select = Box::new(|value: &str| leptos::logging::warn!("{}", value)); 
    view! {
        <ConfigProvider>
            <Menu on_select trigger_type=MenuTriggerType::Hover position=MenuPosition::Top>
                <MenuTrigger slot>
                    <Button shape=ButtonShape::Square icon=icondata::AiStarOutlined>"Modes"</Button>
                </MenuTrigger>
                <Button>"Editing (experimental)"</Button>
                <Button>"Pick & pin"</Button>
            </Menu>
        </ConfigProvider>
    }
}