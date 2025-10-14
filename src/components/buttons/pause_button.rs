use leptos::*;
use thaw::*;


#[component]
pub fn PauseButton() -> impl IntoView {
    view! {     
        <ConfigProvider>
            <Button shape=ButtonShape::Square icon=icondata::AiPauseOutlined>"Pause"</Button>
        </ConfigProvider>
    }
}