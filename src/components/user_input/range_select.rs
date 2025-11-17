use leptos::prelude::*;

#[component]
pub fn Slider(#[prop(into)] min: String) -> impl IntoView {
    view! {
        <label for="test">
            <span
                class="block text-sm font-medium text-gray-900"
                >"Max Volume"</span>
            <input
                type="range"
                name="test"
                id="test"
                min="0"
                max="100"
                value="20"
                class="mt-3 h-3.5 w-full appearance-none rounded-full bg-gray-300 [&amp;::-webkit-slider-thumb]:size-7 [&amp;::-webkit-slider-thumb]:cursor-pointer [&amp;::-webkit-slider-thumb]:appearance-none [&amp;::-webkit-slider-thumb]:rounded-full [&amp;::-webkit-slider-thumb]:border-[6px] [&amp;::-webkit-slider-thumb]:border-gray-500 [&amp;::-webkit-slider-thumb]:bg-gray-200"
            >
        </label>
    }
}
