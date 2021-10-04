use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Season {
    Spring,
    Summer,
    Fall,
    Winter,
}
