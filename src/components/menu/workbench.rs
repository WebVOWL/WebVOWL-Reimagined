mod about_menu;
// mod export_menu;
// mod filter_menu;
// mod modes_menu;
mod ontology_menu;
mod options_menu;
// mod search_menu;

use about_menu::AboutMenu;
use leptos::prelude::*;
use thaw::*;
// use export_menu::ExportMenu;
// use filter_menu::FilterMenu;
// use leptos::prelude::*;
// use modes_menu::ModesMenu;
use crate::components::theme::ThemeSelection;
use ontology_menu::OntologyMenu;
use options_menu::OptionsMenu;
// use search_menu::SearchMenu;

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
        <Flex vertical=true gap=FlexGap::Size(30)>
            {children()}
        </Flex>
    }
}

#[component]
pub fn Workbench() -> impl IntoView {
    view! {
        <Flex
            class="workbench"
            align=FlexAlign::Center
            justify=FlexJustify::SpaceBetween
            vertical=true
        >
            <Flex vertical=true gap=FlexGap::Large>
                <OntologyMenu />
                <OntologyMenu />
            </Flex>
            <Flex
                style="padding-bottom: 2rem;"
                vertical=true
                gap=FlexGap::Large
            >
                <ThemeSelection />
                <OptionsMenu />
                <AboutMenu />
            </Flex>
        </Flex>
    }
}
