use serde::{Deserialize, Serialize};

use crate::types::season::Season;

#[derive(Debug, Deserialize, Serialize)]
pub struct Anime {
    title: String,
    year: i32,
    season: Season,
    wday: String,
    time: String,
    station: String,
}

impl Anime {
    pub fn new(title: String, year: i32, season: Season, detail: Detail) -> Self {
        return Self {
            title: title,
            year: year,
            season: season,
            wday: detail.wday,
            time: detail.time,
            station: detail.station,
        };
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Detail {
    wday: String,
    time: String,
    station: String,
}

impl Detail {
    pub fn new(wday: String, time: String, station: String) -> Detail {
        return Detail {
            wday: wday,
            time: time,
            station: station,
        };
    }
}

pub type Animes = Vec<Anime>;
