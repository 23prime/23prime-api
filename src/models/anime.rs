use diesel::prelude::*;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::dbconfig::POOL;
use crate::schema::gokabot::{animes, animes::dsl};

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
    pub fn all() -> Vec<Self> {
        let conn = POOL.get().expect("Failed to get DB connection from pool");
        return dsl::animes
            .load::<Anime>(&conn)
            .expect("Error loading animes");
    }

    pub fn find_by_year(year: i32) -> Vec<Self> {
        let conn = POOL.get().expect("Failed to get DB connection from pool");
        return dsl::animes
            .filter(dsl::year.eq(year))
            .load::<Anime>(&conn)
            .expect("Error loading animes");
    }

    pub fn find_by_season(year: i32, season: &str) -> Vec<Self> {
        let conn = POOL.get().expect("Failed to get DB connection from pool");
        return dsl::animes
            .filter(dsl::year.eq(year))
            .filter(dsl::season.eq(season))
            .load::<Anime>(&conn)
            .expect("Error loading animes");
    }

    pub fn create(new_animes: Vec<NewAnime>) -> QueryResult<Vec<Self>> {
        let conn = POOL.get().expect("Failed to get DB connection from pool");
        return diesel::insert_into(animes::table)
            .values(new_animes)
            .get_results(&conn);
    }

    pub fn update(anime: &Self) -> QueryResult<Self> {
        let conn = Lazy::force(&POOL)
            .get()
            .expect("Failed to get DB connection from pool");
        return diesel::update(anime).set(anime).get_result(&conn);
        // return anime.save_changes(&conn);
    }

    pub fn updates(animes: Vec<Self>) -> Vec<QueryResult<Self>> {
        let conn = POOL.get().expect("Failed to get DB connection from pool");
        // return animes.into_iter().map(|a| a.save_changes(&conn)).collect();
        return animes
            .into_iter()
            .map(|a| diesel::update(&a).set(&a).get_result(&conn))
            .collect();
    }

    pub fn delete(anime: &Self) -> QueryResult<Self> {
        let conn = POOL.get().expect("Failed to get DB connection from pool");
        return diesel::delete(anime).get_result(&conn);
    }
}
