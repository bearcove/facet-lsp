#![forbid(unsafe_code)]
//! A small LSP 3.17 surface with Facet derives.
//!
//! This crate intentionally does not mirror the whole Language Server Protocol.
//! It contains only a compact reusable type surface, stdio JSON-RPC framing,
//! UTF-16 position conversion, and semantic-token encoding.

pub mod framing;
pub mod position;
pub mod semantic;
pub mod types;

pub use facet_json::RawJson;
