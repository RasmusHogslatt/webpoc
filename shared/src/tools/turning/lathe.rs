use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum LatheType {
    #[default]
    TurningInsert,
    ThreadingTool,
    PartingToolHolder,
}
