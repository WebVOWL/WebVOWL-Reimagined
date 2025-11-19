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
        mt-3 \
        w-full \
        h-3.5 \
        bg-gray-300 \
        rounded-full \
        appearance-none \
        [-webkit-slider-thumb]:bg-pink-500 \
        [::-webkit-slider-thumb]:appearance-none \
        [::-webkit-slider-thumb]:w-5 \
        [::-webkit-slider-thumb]:h-5 \
        [::-webkit-slider-thumb]:rounded-full \
        [::-webkit-moz-range-thumb]:bg-pink-500 \
        [::-webkit-moz-range-thumb]:appearance-none \
        [::-webkit-moz-range-thumb]:w-5 \
        [::-webkit-moz-range-thumb]:h-5 \
        [::-webkit-moz-range-thumb]:rounded-full
        "
    );

    view! {
        <div class="flex justify-center w-70 h-fit">
            <label class="block w-fit text-sm font-medium text-gray-900" for=name.clone()>
                {label}
            </label>
            <input
                on:input= move |event| {
                    let t =  event.target().unwrap().unchecked_into::<HtmlInputElement>();
                    value.set(t.value().parse::<f64>().unwrap());}
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
