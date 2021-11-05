use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Season {
    Winter,
    Spring,
    Summer,
    Fall,
    Other,
}

impl Season {
    pub fn new(str: &str) -> Self {
        let lower: &str = &str.to_lowercase();
        match lower {
            "spring" => return Self::Spring,
            "summer" => return Self::Summer,
            "fall" => return Self::Fall,
            "autumn" => return Self::Fall,
            "winter" => return Self::Winter,
            _ => return Self::Other,
        }
    }
}

impl fmt::Display for Season {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Winter => return write!(f, "winter"),
            Self::Spring => return write!(f, "spring"),
            Self::Summer => return write!(f, "summer"),
            Self::Fall => return write!(f, "fall"),
            Self::Other => return write!(f, "---"),
        }
    }
}
