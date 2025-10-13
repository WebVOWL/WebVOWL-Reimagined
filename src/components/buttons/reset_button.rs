use leptos::*;
use thaw::*;


#[component]
pub fn ResetButton() -> impl IntoView {
    view! {      
        <ConfigProvider>
            <Button shape=ButtonShape::Square icon=icondata::VsDebugRestart>"Reset"</Button>
        </ConfigProvider>
    }
}