#![warn(clippy::all, rust_2018_idioms)]

pub mod app;
pub use app::TemplateApp;
pub mod app_states;
pub mod settings;
pub mod widgets;
