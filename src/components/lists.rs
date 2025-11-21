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
            <a href="#" class="flex items-center gap-2 rounded-lg px-4 py-2 text-gray-500 hover:bg-gray-100 hover:text-gray-700">
            <MaybeShowIcon icon=icon></MaybeShowIcon>
            <span class="text-sm font-medium"> {title} </span>
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
          <summary class="flex cursor-pointer items-center justify-between rounded-lg px-4 py-2 text-gray-500 hover:bg-gray-100 hover:text-gray-700">
            <MaybeShowIcon icon=icon></MaybeShowIcon>
            <span class="text-sm font-medium"> {title} </span>

            <span class="shrink-0 transition duration-300 group-open:-rotate-180">
                <MaybeShowIcon icon=icondata::BiChevronDownRegular></MaybeShowIcon>
            </span>
          </summary>

          <ul class="mt-2 space-y-1 px-4">
            {children()}
          </ul>
        </details>
      </li>
    }
}

/// A child list without an icon. Usually used together with a [`ListDetails`].
#[component]
pub fn ListChild(#[prop(into)] title: String) -> impl IntoView {
    view! {
        <li>
            <a href="#" class="block rounded-lg px-4 py-2 text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700">
            {title}
            </a>
        </li>
    }
}
