use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::models::{Anime, NewAnime};
use crate::types::season::Season;
use crate::types::wday::WDay;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct StrictAnime {
    pub id: Option<i32>,
    pub year: Option<i32>,
    pub season: Option<Season>,
    pub day: Option<WDay>,
    pub time: Option<String>,
    pub station: Option<String>,
    pub title: Option<String>,
    pub recommend: Option<bool>,
}

pub type StrictAnimes = Vec<StrictAnime>;

impl StrictAnime {
    pub fn new(title: String, year: i32, season: Season, detail: Detail) -> Self {
        return Self {
            id: None,
            year: Some(year),
            season: Some(season),
            day: WDay::fron_en(&detail.day),
            time: Some(detail.time),
            station: Some(detail.station),
            title: Some(title),
            recommend: None,
        };
    }

    pub fn new_by_anime(anime: Anime) -> Self {
        return Self {
            id: Some(anime.id),
            year: Some(anime.year),
            season: Some(Season::new(&anime.season)),
            day: WDay::fron_en(&anime.day),
            time: Some(anime.time),
            station: Some(anime.station),
            title: Some(anime.title),
            recommend: Some(anime.recommend),
        };
    }

    pub fn new_by_animes(animes: Vec<Anime>) -> StrictAnimes {
        return animes
            .into_iter()
            .map(|a| StrictAnime::new_by_anime(a))
            .collect();
    }

    pub fn to_anime(self: Self) -> Option<Anime> {
        if self.id.is_none() || self.year.is_none() || self.season.is_none() || self.title.is_none()
        {
            return None;
        }

        return Some(Anime {
            id: self.id.unwrap(),
            year: self.year.unwrap(),
            season: self.season.unwrap().to_string(),
            day: self.day.map(|d| d.to_string()).unwrap_or("---".to_string()),
            time: self.time.unwrap_or("--:--".to_string()),
            station: self.station.unwrap_or("---".to_string()),
            title: self.title.unwrap(),
            recommend: self.recommend.unwrap_or(false),
        });
    }

    pub fn to_new_anime(self: Self) -> Option<NewAnime> {
        if self.year.is_none() || self.season.is_none() || self.title.is_none() {
            return None;
        }

        return Some(NewAnime {
            year: self.year.unwrap(),
            season: self.season.unwrap().to_string(),
            day: self.day.map(|d| d.to_string()).unwrap_or("---".to_string()),
            time: self.time.unwrap_or("--:--".to_string()),
            station: self.station.unwrap_or("---".to_string()),
            title: self.title.unwrap(),
            recommend: self.recommend.unwrap_or(false),
        });
    }

    pub fn to_new_animes(selfs: &Vec<Self>) -> Vec<Option<NewAnime>> {
        return selfs
            .into_iter()
            .map(|a| a.clone().to_new_anime())
            .collect();
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
        return Detail { day, time, station };
    }
}
