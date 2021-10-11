# anime-api

Web API for animes.

## Run

### With Docker

```bash
$ docker-compose -f docker-compose.local.yml up
```

### Without Docker

```bsh
$ cargo run
```

(Recommend) If using [cargo-watch](https://docs.rs/crate/cargo-watch/):

```bash
$ cargo watch -x run
```

## Build and Deploy

```bash
$ aws ecr-public get-login-password --region us-east-1 | docker login --username AWS --password-stdin public.ecr.aws/m0z8x5y6
$ docker-compose build
$ docker tag anime-api:latest public.ecr.aws/m0z8x5y6/anime-api:latest
$ docker push public.ecr.aws/m0z8x5y6/anime-api:latest
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

Access to My URL: <https://23prime.xyz>

Then, redirect to `/auth/callback` and get access token:

```json
{
    id: "{your id}",
    token: "{access token}"
}
```

You can send API request with the header `Authorization: Bearer {access_token}`.

## APIs

### `/api/index`

| Method | Requie Auth |
| :----: | :---------: |
|   GET  |      o      |

#### `GET`

- Response Body

```txt
Hello, Anime API!!
```

### `/api/echo`

| Method | Requie Auth |
| :----: | :---------: |
|   GET  |      o      |
|  POST  |      o      |

#### `GET`

- Response Body

```txt
{your request body}
```

#### `POST`

- Response Body

```txt
{your request body}
```

### `/api/animes/:year/:season`

| Method | Requie Auth |
| :----: | :---------: |
|   GET  |      o      |

#### `GET`

- Params

|   Param  | Kind |   Type  | Required | remarks                        |
| :------: | :--: | :-----: | :------: | :----------------------------- |
|  `year`  | path | integer |   false  |                                |
| `season` | path |  string |   false  | `spring\|summer\|fall\|winter` |

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

| Method | Requie Auth |
| :----: | :---------: |
|   GET  |      o      |

#### `GET`

- Params

|   Param  | Kind |  Type  | Required | remarks                        |
| :------: | :--: | :----: | :------: | :----------------------------- |
| `season` | path | string |   true   | `spring\|summer\|fall\|winter` |

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
