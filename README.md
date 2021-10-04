# anime-api

Web API for animes.

## Run

```bash
$ cargo run
```

(Recommend) If using [cargo-watch](https://docs.rs/crate/cargo-watch/):

```bash
$ cargo watch -x run
```

## APIs

### `/animes`

#### `GET`

- Response Body

```json
{
    "animes": [
        {
            "id": {id},
            "year": {YYYY},
            "season": "{spring|summer|fall|winter}",
            "day": "{Sun|Mon|Tue|Wed|Thu|Fri|Sat}",
            "time": "hh:mm",
            "station": "{station}",
            "title": "{title}",
            "recommend": {true|false}
        },
        // and more...
    ]
}
```

### `/scrape/:season`

#### `GET`

- Response Body

```json
{
    "animes": [
       {
            "title": "{title}",
            "year": {current year (YYYY)},
            "season": "{spring|summer|fall|winter}",
            "wday": "{Sun|Mon|Tue|Wed|Thu|Fri|Sat}",
            "time": "hh:mm",
            "station": "{station}"
        },
        // and more...
    ]
}
```
