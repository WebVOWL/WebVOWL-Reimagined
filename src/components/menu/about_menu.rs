use leptos::*;
use thaw::*;



#[component]
pub fn AboutMenu() -> impl IntoView {
    let on_select = Box::new(|value: &str| leptos::logging::warn!("{}", value)); 
    view! {
        <ConfigProvider>
            <Menu on_select trigger_type=MenuTriggerType::Hover position=MenuPosition::Top>
                <MenuTrigger slot>
                    <Button shape=ButtonShape::Square icon=icondata::AiCopyrightOutlined>"About"</Button>
                </MenuTrigger>
                <Button>"MIT License © 2014-2019"</Button>
                <Caption1Strong>"WebVOWL Developers:"</Caption1Strong>
                <Caption1Strong>"Vincent Link, Steffen Lohmann,
                                Eduard Marbach, Stefan Negru, Vitalis Wiens"
                </Caption1Strong>
                <Button>"MIT License © 2025"</Button>
                <Caption1Strong>"WebVOWL-Reimagined Developers:"</Caption1Strong>
                <Caption1Strong>"Kneckerino, KristianEmilWN, nikarnik,TheRealMorgenfrue"</Caption1Strong>
                <Button>"Version 1.3.9 (release history)"</Button>
                <Button>"VOWL Specification »"</Button>
            </Menu>
        </ConfigProvider>
    }
}