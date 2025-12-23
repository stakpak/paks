//! Paks Registry API Rust SDK
//!
//! HTTP client for interacting with the Stakpak Paks Registry API.
//! Types are re-exported from `paks-api-schema`.

pub mod client;
pub mod error;

pub use client::PaksClient;
pub use error::ApiError;
