use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum WDay {
    Sun,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
}

impl WDay {
    pub fn fron_en(en_str: &str) -> Option<Self> {
        let lower: &str = &en_str.to_lowercase();
        match lower {
            "sun" => Some(Self::Sun),
            "mon" => Some(Self::Mon),
            "tue" => Some(Self::Tue),
            "wed" => Some(Self::Wed),
            "thu" => Some(Self::Thu),
            "fri" => Some(Self::Fri),
            "sat" => Some(Self::Sat),
            _ => None,
        }
    }

    pub fn from_jp(jp_str: &str) -> Option<Self> {
        match jp_str {
            "日" => Some(Self::Sun),
            "月" => Some(Self::Mon),
            "火" => Some(Self::Tue),
            "水" => Some(Self::Wed),
            "木" => Some(Self::Thu),
            "金" => Some(Self::Fri),
            "土" => Some(Self::Sat),
            _ => None,
        }
    }
}

impl fmt::Display for WDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sun => return write!(f, "Sun"),
            Self::Mon => return write!(f, "Mon"),
            Self::Tue => return write!(f, "Tue"),
            Self::Wed => return write!(f, "Wed"),
            Self::Thu => return write!(f, "Thu"),
            Self::Fri => return write!(f, "Fri"),
            Self::Sat => return write!(f, "Sat"),
        }
    }
}
