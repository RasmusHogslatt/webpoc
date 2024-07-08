#![warn(clippy::all, rust_2018_idioms)]

pub mod app;
pub mod db;
pub use app::TemplateApp;
pub use db::*;
