use serde::{Deserialize, Serialize};

use crate::types::season::Season;

#[derive(Debug, Deserialize, Serialize)]
pub struct Anime {
    title: String,
    year: i32,
    season: Season,
    day: String,
    time: String,
    station: String,
}

impl Anime {
    pub fn new(title: String, year: i32, season: Season, detail: Detail) -> Self {
        return Self {
            title: title,
            year: year,
            season: season,
            day: detail.day,
            time: detail.time,
            station: detail.station,
        };
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Detail {
    day: String,
    time: String,
    station: String,
}

impl Detail {
    pub fn new(day: String, time: String, station: String) -> Detail {
        return Detail {
            day: day,
            time: time,
            station: station,
        };
    }
}

pub type Animes = Vec<Anime>;
