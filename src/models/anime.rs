use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::dbconfig::POOL;
use crate::schema::gokabot::animes;

#[derive(AsChangeset, Debug, Deserialize, Identifiable, Queryable, Serialize)]
pub struct Anime {
    pub id: i32,
    pub year: i32,
    pub season: String,
    pub day: String,
    pub time: String,
    pub station: String,
    pub title: String,
    pub recommend: bool,
}

#[derive(Debug, Deserialize, Insertable, Serialize, Clone)]
#[table_name = "animes"]
pub struct NewAnime {
    pub year: i32,
    pub season: String,
    pub day: String,
    pub time: String,
    pub station: String,
    pub title: String,
    pub recommend: bool,
}

impl Anime {
    pub fn delete(anime: &Self) -> QueryResult<Self> {
        let conn = POOL.get().expect("Failed to get DB connection from pool");
        return diesel::delete(anime).get_result(&conn);
    }
}
