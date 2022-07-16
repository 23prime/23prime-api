# 23prime-api #

[![Rust CI](https://github.com/23prime/23prime-api/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/23prime/23prime-api/actions/workflows/rust-ci.yml)

Backend Web API for [23prime-page](https://github.com/23prime/23prime-page).

## Run ##

### With Docker ###

```console
$ docker-compose -f docker-compose.local.yml up
```

### Without Docker ###

```bsh
$ cargo run
```

(Recommend) If using [cargo-watch](https://docs.rs/crate/cargo-watch/):

```console
$ cargo watch -x run
```

## Test ##

### With Docker ###

```console
$ docker-compose -f docker-compose.local.yml run --rm api-local cargo test
```

When already docker-compose up:

```console
$ docker-compose -f docker-compose.local.yml exec api-local cargo test
```

### Without Docker ###

```console
$ cargo test
```

## Build and Deploy ##

```console
$ aws ecr-public get-login-password --region us-east-1 | docker login --username AWS --password-stdin public.ecr.aws/m0z8x5y6
$ docker-compose build
$ docker push public.ecr.aws/m0z8x5y6/okkey-api:latest
```

## Environment variables ##

Make `.env` and add some variables. See `.env.template`.

## Migrate by SeaORM ##

// TODO

## Authorization ##

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

## APIs ##

### `/api/index` ###

| Method | Require Auth |
| :----: | :----------: |
|   GET  |       o      |

#### `GET` ####

- Response Body

```txt
Hello, Anime API!!
```

### `/api/echo` ###

| Method | Require Auth |
| :----: | :----------: |
|   GET  |       o      |
|  POST  |       o      |

#### `GET` ####

- Response Body

```txt
{your request body}
```

#### `POST` ####

- Response Body

```txt
{your request body}
```

### `/api/animes/:year/:season` ###

| Method | Require Auth |
| :----: | :----------: |
|   GET  |       o      |

#### `GET` ####

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

#### `POST` ####

- Params

|    Param    | Kind |   Type  | Required | remarks |
| :---------: | :--: | :-----: | :------: | :------ |
|     `id`    | body | integer |   true   |         |
|    `year`   | body | integer |   true   |         |
|   `season`  | body |  string |   true   |         |
|    `day`    | body |  string |   true   |         |
|    `time`   | body |  string |   true   |         |
|  `station`  | body |  string |   true   |         |
|   `title`   | body |  string |   true   |         |
| `recommend` | body |   bool  |   true   |         |

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

#### `PUT` ####

- Params

|    Param    | Kind |   Type  | Required | remarks |
| :---------: | :--: | :-----: | :------: | :------ |
|     `id`    | body | integer |   true   |         |
|    `year`   | body | integer |   true   |         |
|   `season`  | body |  string |   true   |         |
|    `day`    | body |  string |   true   |         |
|    `time`   | body |  string |   true   |         |
|  `station`  | body |  string |   true   |         |
|   `title`   | body |  string |   true   |         |
| `recommend` | body |   bool  |   true   |         |

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

#### `DELETE` ####

- Params

|    Param    | Kind |   Type  | Required | remarks |
| :---------: | :--: | :-----: | :------: | :------ |
|     `id`    | body | integer |   true   |         |
|    `year`   | body | integer |   true   |         |
|   `season`  | body |  string |   true   |         |
|    `day`    | body |  string |   true   |         |
|    `time`   | body |  string |   true   |         |
|  `station`  | body |  string |   true   |         |
|   `title`   | body |  string |   true   |         |
| `recommend` | body |   bool  |   true   |         |

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

### `/scrape/:season` ###

| Method | Require Auth |
| :----: | :----------: |
|   GET  |       o      |

#### `GET` ####

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
