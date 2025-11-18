use std::fmt::Display;

use leptos::prelude::*;
use num::Num;

/// Sliding range of values.
///
/// # Panics
/// - If the value of `max` exceeds the size of T.
/// - If the value of `min` is less than the size of T.
///
/// ## Examples
/// - `max` fits into u16, but T = u8.
/// - `min` fits into i32, but T = u32
#[component]
pub fn Slider<T>(
    #[prop(into)] label: String,
    /// The value of the slider.
    /// V is implemented by all primitive number types
    /// in the standard library, e.g. f64, i128, isize, usize etc.
    #[prop(into)]
    value: RwSignal<T>,
    #[prop(into)] min: String,
    #[prop(into)] max: String,
    #[prop(into, default = "1.0".to_string())] step: String,
) -> impl IntoView
where
    T: Num + Display + Send + Sync + Clone + 'static,
{
    let name = label.replace(" ", "-");

    view! {
        <label for=name>
            <span class="block text-sm font-medium text-gray-900">{label}</span>
            <input
                on:input= move |event| {value.set(event.target().unwrap().as_f64().unwrap().into())}
                type="range"
                id=name.clone()
                min=min
                max=max
                step=step
                value=value.get().to_string()
                class="mt-3 w-full h-3.5 bg-gray-300 rounded-full appearance-none [&amp;::-webkit-slider-thumb]:size-7 [&amp;::-webkit-slider-thumb]:cursor-pointer [&amp;::-webkit-slider-thumb]:appearance-none [&amp;::-webkit-slider-thumb]:rounded-full [&amp;::-webkit-slider-thumb]:border-[6px] [&amp;::-webkit-slider-thumb]:border-gray-500 [&amp;::-webkit-slider-thumb]:bg-gray-200"
            />
        </label>
    }
}
