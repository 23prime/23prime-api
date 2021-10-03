use log::{debug, info};
use std::str;

use actix_web::client::Client;
use actix_web::web::Buf;
use scraper::{Html, Selector};

use crate::types::season::Season;

const BASE_URL: &str = "https://akiba-souken.com/anime/";
const SELECTOR: &str = "div.animeList dl dd a";

pub async fn fetch_all(season: &Season) -> Vec<String> {
    let url = mk_url(&season);
    debug!("url = {:?}", url);

    let response = Client::default().get(url).send().await;
    debug!("response = {:?}", response);

    let body;

    if let Ok(mut res) = response {
        body = res.body().limit(20_000_000).await.unwrap();
    } else {
        return vec![];
    }

    let document = Html::parse_document(&str::from_utf8(body.bytes()).unwrap());
    let selector = Selector::parse(SELECTOR).unwrap();
    let elements = document.select(&selector);
    let animes = elements.map(|e| e.inner_html()).collect::<Vec<String>>();
    info!("animes = {:?}", animes);

    return animes;
}

fn mk_url(season: &Season) -> String {
    match season {
        Season::Spring => return format!("{}{}/", BASE_URL, "spring"),
        Season::Summer => return format!("{}{}/", BASE_URL, "summer"),
        Season::Fall => return format!("{}{}/", BASE_URL, "autumn"),
        Season::Winter => return format!("{}{}/", BASE_URL, "winter"),
    };
}
