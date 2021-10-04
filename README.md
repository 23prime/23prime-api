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

## Environment variables

Make `./.env` and add this:

```txt
ACTIX_HOST={application host}
ACTIX_PORT={application port}
RUST_LOG={info|debug|...}
DATABASE_URL=postgres://....
AUTHORITY={Auth0 authority}
```

## Migrate

### Install Diesel CLI

```bash
$ cargo install diesel_cli --no-default-features --features "postgres"
```

### Migrate

```bash
$ diesel migration generate ${migration name}
$ diesel migration run
$ diesel print-schema -s gokabot >> src/schema.rs
```

## Authorization

Authorized with [Authorization Code Flow by Auth0](https://auth0.com/docs/login/authentication/add-login-auth-code-flow).

Access to login URL:

```txt
https://23prime.jp.auth0.com/authorize?response_type=code&client_id={client_id}&redirect_uri=http:/localhost:8080/auth/callback&scope=openid&state=hoge
```

Then, redirect to `/auth/callback` and get access token:

```json
{
    "access_token": "{access_token}"
}
```

You can send API request with the header `Authorization: Bearer {access_token}`.

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
