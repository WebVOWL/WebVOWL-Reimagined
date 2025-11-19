// mod about_menu;
// mod export_menu;
// mod filter_menu;
// mod modes_menu;
// mod ontology_menu;
mod options_menu;
// mod search_menu;

use crate::components::lists::{ListDetails, ListElement};
use crate::components::menu::{mega_menu::MegaMenu, vertical_menu::VerticalMenu};
use leptos::prelude::*;
use options_menu::OptionsMenu;
use thaw::*;

#[component]
pub fn WorkBenchButton(
    #[prop(default=ButtonShape::Rounded)] shape: ButtonShape,
    icon: icondata::Icon,
    #[prop(into)] text: String,
) -> impl IntoView {
    view! {
        <Button shape=shape icon=icon>
            {text}
        </Button>
    }
}

#[component]
fn WorkbenchMenuItems(#[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <div class="workbench-menu-header">
            <h3>{title}</h3>
        </div>
        <div>
            {children()}
        </Flex>
    }
}

// #[component]
// pub fn Workbench() -> impl IntoView {
//     view! {
//         <Flex
//             class="workbench"
//             align=FlexAlign::Center
//             justify=FlexJustify::SpaceBetween
//             vertical=true
//         >
//             <Flex vertical=true gap=FlexGap::Large>
//                 <OntologyMenu />
//                 <OntologyMenu />
//             </Flex>
//             <Flex
//                 style="padding-bottom: 2rem;"
//                 vertical=true
//                 gap=FlexGap::Large
//             >
//                 <ThemeSelection />
//                 <OptionsMenu />
//                 <AboutMenu />
//             </Flex>
//         </Flex>
//     }
// }

#[component]
pub fn NewWorkbench() -> impl IntoView {
    view! {
        <VerticalMenu>
            <ListElement
                title="Load Ontology"
                icon=icondata::BiMenuRegular
            >
                <button/>
            </ListElement>

            <ListElement
                title="Search"
                icon=icondata::BiMenuRegular
            >
                <button/>
            </ListElement>

            <ListElement
                title="Filter"
                icon=icondata::BiMenuRegular
            >
                <button/>
            </ListElement>

            <ListElement
                title="Export"
                icon=icondata::BiMenuRegular
            >
                <button/>
            </ListElement>

            <ListDetails
                title="Settings"
                icon=icondata::IoSettingsOutline
            >
                <ListElement title="Simulator">
                    <button/>
                </ListElement>
            </ListDetails>

            <ListElement
                title="About"
                icon=icondata::BiMenuRegular
            >
                <button/>
            </ListElement>
        </VerticalMenu>
    }
}
