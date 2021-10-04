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
