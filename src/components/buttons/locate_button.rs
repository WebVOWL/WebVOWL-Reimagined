use leptos::*;
use thaw::*;


#[component]
pub fn LocateButton() -> impl IntoView {
    view! {       
        <ConfigProvider>
            <Button shape=ButtonShape::Square class="locateButton">"⌖"</Button>
        </ConfigProvider>
    }
}