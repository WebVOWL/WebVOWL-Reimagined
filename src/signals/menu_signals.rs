use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct ShowOntologyMenu(pub RwSignal<bool>);

#[derive(Clone, Copy)]
pub struct ShowSearchMenu(pub RwSignal<bool>);

#[derive(Clone, Copy)]
pub struct ShowFilterMenu(pub RwSignal<bool>);

#[derive(Clone, Copy)]
pub struct ShowExportMenu(pub RwSignal<bool>);

#[derive(Clone, Copy)]
pub struct ShowOptionsMenu(pub RwSignal<bool>);

#[derive(Clone, Copy)]
pub struct ShowAboutMenu(pub RwSignal<bool>);

#[derive(Clone, Copy)]
pub struct SidebarOpen(pub RwSignal<bool>);
