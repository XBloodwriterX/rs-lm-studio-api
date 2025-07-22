#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

pub mod chat;
pub mod error;
pub(crate) mod macron {
    pub use macron_impl_display::*;
    pub use macron_impl_error::*;
    pub use macron_impl_from::*;
    pub use macron_str::*;
}

pub use chat::*;
pub use error::{Error, Result};
pub use macron::*;
