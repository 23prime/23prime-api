use std::str;

use awc::Client;
use chrono::{Datelike, Local};
use log::{debug, info};
use scraper::element_ref::ElementRef;
use scraper::{Html, Selector};

use crate::types::animes::{Detail, StrictAnime, StrictAnimes};
use crate::types::season::Season;
use crate::types::wday::WDay;

const BASE_URL: &str = "https://akiba-souken.com/anime/";

pub async fn fetch(season: Season) -> StrictAnimes {
    let url = mk_url(&season);
    if url.is_none() {
        return vec![];
    }

    let response = Client::default().get(url.unwrap()).send().await;

    let body = if let Ok(mut res) = response {
        res.body().limit(20_000_000).await.unwrap()
    } else {
        return vec![];
    };

    let document = Html::parse_document(str::from_utf8(&body).unwrap());
    let year = parse_year(&document);

    let selector = Selector::parse("div.itemBox").unwrap();
    let anime_items = document.select(&selector);

    let animes = anime_items
        .into_iter()
        .map(|i| StrictAnime::new(parse_title(&i), year, season.clone(), parse_detail(&i)))
        .collect::<StrictAnimes>();
    info!("fetch animes = {:?}", animes);

    return animes;
}

fn mk_url(season: &Season) -> Option<String> {
    match season {
        Season::Spring => return Some(format!("{}{}/", BASE_URL, "spring")),
        Season::Summer => return Some(format!("{}{}/", BASE_URL, "summer")),
        Season::Fall => return Some(format!("{}{}/", BASE_URL, "autumn")),
        Season::Winter => return Some(format!("{}{}/", BASE_URL, "winter")),
        Season::Other => return None,
    };
}

fn parse_year(document: &Html) -> i32 {
    let selector = Selector::parse("div#contents div h1").unwrap();
    let inner_html = document.select(&selector).next().unwrap().inner_html();
    let year_str = &inner_html[0..4];

    if let Ok(year) = year_str.parse::<i32>() {
        return year;
    } else {
        return Local::now().year();
    }
}

// == Note ==
// Two patterns for the tag of the title element.
//   1. default: <h2><a href="/anime/xxxxx/">example title</a></h2>
//   2. without `a` (commented out): <h2><!--<a href="/anime/xxxxx/">-->example title<!--</a>--></h2>
fn parse_title(elem: &ElementRef) -> String {
    let selector = Selector::parse("h2 a").unwrap();

    if let Some(title) = elem.select(&selector).next() {
        return title.inner_html();
    }

    let other_selector = Selector::parse("h2").unwrap();

    if let Some(title) = elem.select(&other_selector).next() {
        return title.inner_html();
    }

    return "".to_string();
}

fn parse_detail(elem: &ElementRef) -> Detail {
    let selector = Selector::parse("div.firstDate").unwrap();
    let inner = elem.select(&selector).next().unwrap().inner_html();

    let splitted_by_nbsp = inner.split("&nbsp;").collect::<Vec<&str>>();
    debug!("splitted_by_nbsp = {:?}", splitted_by_nbsp);

    let date_station = splitted_by_nbsp[2].replace(')', "(");

    let date_station_slice = date_station.split('(').collect::<Vec<&str>>();
    debug!("date_station_slice = {:?}", date_station_slice);

    let wday = parse_wday_jp(&date_station_slice);
    let time = parse_time(&date_station_slice);
    let station = parse_station(&date_station_slice);

    let result = Detail::new(wday, time, station);
    info!("result = {:?}", result);
    return result;
}

fn parse_wday_jp(date_station_slice: &[&str]) -> String {
    if date_station_slice.len() < 2 {
        return "---".to_string();
    }

    let wday_jp = date_station_slice[1];
    let wday = WDay::from_jp(wday_jp);

    if let Some(s) = wday {
        return s.to_string();
    }

    return "---".to_string();
}

fn parse_time(date_station_slice: &[&str]) -> String {
    if date_station_slice.len() < 3 {
        return "--:--".to_string();
    }

    let time = date_station_slice[2];
    let replaced = time.replace('～', "");

    if replaced.is_empty() {
        return "--:--".to_string();
    }

    return replaced;
}

fn parse_station(date_station_slice: &[&str]) -> String {
    if date_station_slice.len() < 4 {
        return "---".to_string();
    }

    return date_station_slice[3].to_string();
}
