use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::establish_connection;
use crate::schema::gokabot::{animes, animes::dsl};

#[derive(Debug, Deserialize, Queryable, Serialize)]
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

#[derive(Debug, Deserialize, Insertable, Serialize)]
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
        let conn = establish_connection();
        return dsl::animes
            .load::<Anime>(&conn)
            .expect("Error loading animes");
    }

    pub fn find_by_year(year: i32) -> Vec<Self> {
        let conn = establish_connection();
        return dsl::animes
            .filter(dsl::year.eq(year))
            .load::<Anime>(&conn)
            .expect("Error loading animes");
    }

    pub fn find_by_season(year: i32, season: &str) -> Vec<Self> {
        let conn = establish_connection();
        return dsl::animes
            .filter(dsl::year.eq(year))
            .filter(dsl::season.eq(season))
            .load::<Anime>(&conn)
            .expect("Error loading animes");
    }

    pub fn create_animes(new_animes: &Vec<NewAnime>) -> QueryResult<Vec<Self>> {
        let conn = establish_connection();
        return diesel::insert_into(animes::table)
            .values(new_animes)
            .get_results(&conn);
    }
}
