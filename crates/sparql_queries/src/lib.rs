//! Shared SPARQL query strings for WebVOWL-Reimagined.
//!
//! This crate is intentionally dependency-free and WASM-safe so it can be used by:
//! - the SSR/server side (via `webvowl-database`)
//! - the client/wasm side (via `webvowl-reimagined`)

pub mod default;
pub mod general;
pub mod filter_menu_patterns;
pub mod default_query;

pub const DEFAULT_QUERY: &str = include_str!("default.rq");
