use crate::components::icon::MaybeShowIcon;
use leptos::prelude::*;

/// A generic list element.
#[component]
pub fn ListElement(
    #[prop(into)] title: String,
    #[prop(optional, into)] icon: MaybeProp<icondata::Icon>,
) -> impl IntoView {
    view! {
        <li>
            <a
                href="#"
                class="flex gap-2 items-center py-2 px-4 text-gray-500 rounded-lg hover:text-gray-700 hover:bg-gray-100"
            >
                <MaybeShowIcon icon=icon></MaybeShowIcon>
                <span class="text-sm font-medium">{title}</span>
            </a>
        </li>
    }
}

/// A list with a dropdown button containing children (usually [`ListChild`]).
#[component]
pub fn ListDetails(
    #[prop(into)] title: String,
    #[prop(optional, into)] icon: MaybeProp<icondata::Icon>,
    children: Children,
) -> impl IntoView {
    view! {
        <li>
            <details class="group [&amp;_summary::-webkit-details-marker]:hidden">
                <summary class="flex justify-between items-center py-2 px-4 text-gray-500 rounded-lg cursor-pointer hover:text-gray-700 hover:bg-gray-100">
                    <div class="flex gap-2 items-center">
                        <MaybeShowIcon icon=icon></MaybeShowIcon>
                        <span class="text-sm font-medium">{title}</span>
                    </div>

                    <span class="transition duration-300 shrink-0 group-open:-rotate-180">
                        <MaybeShowIcon icon=icondata::BiChevronDownRegular></MaybeShowIcon>
                    </span>
                </summary>

                <ul class="px-4 mt-2 space-y-1">{children()}</ul>
            </details>
        </li>
    }
}

/// A child list without an icon. Usually used together with a [`ListDetails`].
#[component]
pub fn ListChild(#[prop(into)] title: String) -> impl IntoView {
    view! {
        <li>
            <a
                href="#"
                class="block py-2 px-4 text-sm font-medium text-gray-500 rounded-lg hover:text-gray-700 hover:bg-gray-100"
            >
                {title}
            </a>
        </li>
    }
}
