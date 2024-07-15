use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum TapType {
    #[default]
    Tap,
    Form,
    Bottom,
    Through,
}
