use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum HolderType {
    #[default]
    ER8,
    ER11,
    ER16,
    ER32,
}
