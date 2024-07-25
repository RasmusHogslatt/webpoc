use serde::{Deserialize, Serialize};

use crate::{holders::holder::Holder, tools::tool::Tool};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Library {
    pub tools: Vec<Tool>,
    pub holder: Vec<Holder>,
}
