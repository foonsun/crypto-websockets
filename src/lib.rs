#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_errors_doc)]
mod error;
pub mod binance_model;
pub mod binance_uswap_model;
pub mod models;
pub mod websocket;
pub mod subscription;

pub use crate::models::*;
pub use crate::error::*;
pub use crate::{websocket::Websocket};