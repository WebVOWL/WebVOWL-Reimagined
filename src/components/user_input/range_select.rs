use leptos::prelude::*;
use web_sys::HtmlInputElement;
use web_sys::wasm_bindgen::JsCast;

/// Sliding range of values.
#[component]
pub fn Slider(
    #[prop(into)] label: String,
    #[prop(into)] value: RwSignal<f64>,
    #[prop(into)] min: String,
    #[prop(into)] max: String,
    #[prop(into, default = "1.0".to_string())] step: String,
) -> impl IntoView
where
{
    let name = label.replace(" ", "-");
    let slider_class = format!(
        "
        relative \
        overflow-hidden \
        w-full \
        h-3.5 \
        text-blue-500 \
        bg-black-300 \
        text-2xl \
        slider-thumb-h-2 \
        slider-track-h-0.5 \
        active:cursor-grabbing \
        disabled:grayscale \
        disabled:opacity-30% \
        disabled:cursor-not-allowed \
        "
    );
    // rounded-full \

    view! {
        <div class="flex justify-center w-70 h-fit">
            <label
                class="block text-sm font-medium text-gray-900 w-fit"
                for=name.clone()
            >
                {label}
            </label>
            <input
                on:input=move |event| {
                    let t = event
                        .target()
                        .unwrap()
                        .unchecked_into::<HtmlInputElement>();
                    value.set(t.value().parse::<f64>().unwrap());
                }
                type="range"
                id=name
                min=min
                max=max
                step=step
                value=value.get().to_string()
                class=slider_class
            />
        </div>
    }
}
