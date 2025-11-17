mod about_menu;
// mod export_menu;
// mod filter_menu;
// mod modes_menu;
mod ontology_menu;
mod options_menu;
// mod search_menu;

use about_menu::AboutMenu;
use leptos::prelude::*;
use leptos::server_fn::codec::{MultipartData, MultipartFormData};
use log::info;
use thaw::*;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};
// use export_menu::ExportMenu;
// use filter_menu::FilterMenu;
// use leptos::prelude::*;
// use modes_menu::ModesMenu;
use crate::components::theme::ThemeSelection;
use ontology_menu::OntologyMenu;
use options_menu::OptionsMenu;
// use search_menu::SearchMenu;

use crate::components::lists::{ListChild, ListDetails, ListElement};

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

/// Local reads file and calls for the datatype label and returns (label, data content)
#[server(input = MultipartFormData)]
pub async fn handle_local(data: MultipartData) -> Result<usize, ServerFnError> {
    // match fs::read_to_string(&path) {
    //     Ok(content) => Ok((
    //         Self::find_data_type(&path).unwrap_or(DataType::RDF),
    //         content,
    //     )),
    //     Err(e) => Err(format!("Error reading local file: {}", e)),
    // }

    // this will just measure the total number of bytes uploaded
    let mut data = data.into_inner().unwrap();
    let mut count = 0;
    while let Ok(Some(mut field)) = data.next_field().await {
        info!("\n[NEXT FIELD]\n");
        let name = field.name().unwrap_or_default().to_string();
        info!("  [NAME] {name}");
        while let Ok(Some(chunk)) = field.chunk().await {
            let len = chunk.len();
            count += len;
            info!("      [CHUNK] {len}");
            // in a real server function, you'd do something like saving the file here
        }
    }
    Ok(count)
}

#[component]
pub fn Upload() -> impl IntoView {
    let upload_action = Action::new_local(|data: &FormData| handle_local(data.clone().into()));
    // let status_msg = RwSignal::new(String::new());
    // let total = RwSignal::new(0 as u64);
    // let progress = RwSignal::new(0);
    // let progress = Memo::new(move |_| match upload_action.value().get() {
    //     Some(Ok(value)) => {
    //         info!("Test {}", value);
    //         value as u64
    //     }
    //     _ => 0,
    // });

    view! {
        <p>
            {move || {
                if upload_action.input().read().is_none()
                    && upload_action.value().read().is_none()
                {
                    "Upload a file.".to_string()
                } else if upload_action.pending().get() {
                    "Uploading...".to_string()
                } else if let Some(Ok(value)) = upload_action.value().get() {
                    value.to_string()
                } else {
                    format!("{:?}", upload_action.value().get())
                }
            }}
        </p>

        <form on:submit=move |e: SubmitEvent| {
            e.prevent_default();
            let target = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlFormElement>();
            let form_data = FormData::new_with_form(&target).unwrap();
            upload_action.dispatch_local(form_data);
        }>
            <input type="file" name="file_to_upload" />
            // on:input:target=move |e| {
            // if let Some(file_list) = e.target().files() {
            // let mut size = 0;
            // for i in 0..file_list.length() {
            // let item = file_list.item(i).unwrap();
            // size += item.size() as u64;
            // }
            // info!("File size: {}", size);
            // total.set(size);
            // }
            // }
            <input type="submit" />

        </form>
    }
}

#[component]
pub fn NewWorkbench() -> impl IntoView {
    view! {
        <div class="flex flex-col justify-between h-screen bg-white border-gray-100 w-fit border-e">
            <div class="py-6 px-4">
                <ul class="mt-6 space-y-1">
                    <ListElement
                        title="Load Ontology"
                        icon=icondata::BiMenuRegular
                    ></ListElement>

                    <ListElement
                        title="Search"
                        icon=icondata::BiMenuRegular
                    ></ListElement>

                    <ListElement
                        title="Filter"
                        icon=icondata::BiMenuRegular
                    ></ListElement>

                    <ListElement
                        title="Export"
                        icon=icondata::BiMenuRegular
                    ></ListElement>

                    <ListDetails
                        title="Settings"
                        icon=icondata::IoSettingsOutline
                    >
                        <ListChild title="Simulator"></ListChild>

                    </ListDetails>

                    <ListElement
                        title="About"
                        icon=icondata::BiMenuRegular
                    ></ListElement>
                </ul>
            </div>
        </div>
    }
}
