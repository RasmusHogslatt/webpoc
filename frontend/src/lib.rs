#![warn(clippy::all, rust_2018_idioms)]

pub mod app;
pub use app::Application;
pub mod app_states;
pub mod database_interactions;
pub mod singletons;
pub mod widgets;
