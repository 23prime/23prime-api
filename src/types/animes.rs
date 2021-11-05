use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::models::Anime;
use crate::types::season::Season;
use crate::types::wday::WDay;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct StrictAnime {
    pub id: Option<i32>,
    pub year: i32,
    pub season: Season,
    pub day: Option<WDay>,
    pub time: String,
    pub station: String,
    pub title: String,
    pub recommend: Option<bool>,
}

pub type StrictAnimes = Vec<StrictAnime>;

impl StrictAnime {
    pub fn new(title: String, year: i32, season: Season, detail: Detail) -> Self {
        return Self {
            id: None,
            year: year,
            season: season,
            day: WDay::fron_en(&detail.day),
            time: detail.time,
            station: detail.station,
            title: title,
            recommend: None,
        };
    }

    pub fn new_by_anime(anime: Anime) -> Self {
        return Self {
            id: Some(anime.id),
            year: anime.year,
            season: Season::new(&anime.season),
            day: WDay::fron_en(&anime.day),
            time: anime.time,
            station: anime.station,
            title: anime.title,
            recommend: Some(anime.recommend),
        };
    }

    pub fn new_by_animes(animes: Vec<Anime>) -> StrictAnimes {
        return animes
            .into_iter()
            .map(|a| StrictAnime::new_by_anime(a))
            .collect::<StrictAnimes>();
    }
}

impl PartialOrd for StrictAnime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for StrictAnime {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.year != other.year {
            return self.year.cmp(&other.year);
        }

        if self.season != other.season {
            return self.season.cmp(&other.season);
        }

        if self.day != other.day {
            return self.day.cmp(&other.day);
        }

        if self.time != other.time {
            return self.time.cmp(&other.time);
        }

        return self.station.cmp(&other.station);
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
