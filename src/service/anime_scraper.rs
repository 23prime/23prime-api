use log::{debug, info};
use std::str;

use actix_web::client::Client;
use actix_web::web::Buf;
use chrono::{Datelike, Local};
use scraper::element_ref::ElementRef;
use scraper::{Html, Selector};

use crate::types::animes::{Anime, Animes, Detail};
use crate::types::season::Season;
use crate::types::wday::WDay;

const BASE_URL: &str = "https://akiba-souken.com/anime/";

pub async fn fetch(season: Season) -> Animes {
    let url = mk_url(&season);
    let response = Client::default().get(url).send().await;
    let body;

    if let Ok(mut res) = response {
        body = res.body().limit(20_000_000).await.unwrap();
    } else {
        return vec![];
    }

    let document = Html::parse_document(&str::from_utf8(body.bytes()).unwrap());
    let selector = Selector::parse("div.itemBox").unwrap();
    let anime_items = document.select(&selector);

    let animes = anime_items
        .into_iter()
        .map(|i| {
            Anime::new(
                parse_title(&i),
                Local::now().year(),
                season.clone(),
                parse_detail(&i),
            )
        })
        .collect::<Animes>();
    info!("fetch animes = {:?}", animes);

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

fn parse_title(elem: &ElementRef) -> String {
    let selector = Selector::parse("h2 a").unwrap();
    return elem.select(&selector).nth(0).unwrap().inner_html();
}

fn parse_detail(elem: &ElementRef) -> Detail {
    let selector = Selector::parse("div.firstDate").unwrap();
    let inner = elem.select(&selector).nth(0).unwrap().inner_html();

    let splited_by_nbsp = inner.split("&nbsp;").collect::<Vec<&str>>();
    debug!("splited_by_nbsp = {:?}", splited_by_nbsp);

    let date_station = splited_by_nbsp[2].replace(")", "(");

    let date_station_vec = date_station.split('(').collect::<Vec<&str>>();
    debug!("date_station_vec = {:?}", date_station_vec);

    let wday = parse_wday_jp(date_station_vec[1]);
    let time = parse_time(date_station_vec[2]);
    let station = date_station_vec[3].to_string();

    let result = Detail::new(wday, time, station);
    info!("result = {:?}", result);
    return result;
}

fn parse_wday_jp(wday_jp: &str) -> String {
    let wday = WDay::from_jp(wday_jp);
    let mut result = "---".to_string();

    if let Some(s) = wday {
        result = s.to_string();
    }

    return result;
}

fn parse_time(time: &str) -> String {
    let replaced = time.replace("～", "");
    if replaced.is_empty() {
        return "--:--".to_string();
    }
    return replaced;
}
