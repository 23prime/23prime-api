use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::establish_connection;
use crate::schema::gokabot::animes::dsl;

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

impl Anime {
    pub fn all() -> Vec<Self> {
        let connection = establish_connection();
        return dsl::animes
            .load::<Anime>(&connection)
            .expect("Error loading posts");
    }
}
