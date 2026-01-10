//! Shared SPARQL query strings for VOWL-R.
//!
//! This crate is intentionally dependency-free and WASM-safe so it can be used by:
//! - the SSR/server side (via `vowlr-database`)
//! - the client/wasm side (via `vowlr-reimagined`)

pub mod default;
pub mod default_query;
pub mod filter_menu_patterns;
pub mod general;
