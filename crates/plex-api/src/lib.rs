#![deny(
    unreachable_pub,
    clippy::dbg_macro,
    clippy::print_stdout,
    clippy::print_stderr
)]

//! TODO
//!
mod error;
mod http_client;
pub mod media_container;
mod myplex;
mod player;
mod server;
pub mod url;
pub mod webhook;

pub use error::Error;
pub use http_client::{HttpClient, HttpClientBuilder};
pub use myplex::{
    account::RestrictionProfile, device, discover, pin::PinManager, sharing, MyPlex, MyPlexBuilder,
};
pub use player::Player;
pub use server::{library, prefs::Preferences, transcode, Server};

pub type Result<T = (), E = error::Error> = std::result::Result<T, E>;
