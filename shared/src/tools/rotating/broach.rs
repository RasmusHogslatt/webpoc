use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum BroachType {
    #[default]
    HSS,
    Carbide,
}
