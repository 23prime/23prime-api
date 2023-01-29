# 23prime-api #

[![Rust CI](https://github.com/23prime/23prime-api/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/23prime/23prime-api/actions/workflows/rust-ci.yml)

Backend Web API for [23prime-page](https://github.com/23prime/23prime-page).

## Run ##

### With Docker ###

```console
$ docker-compose -f docker-compose.local.yml up
```

### Only DB ###

```console
$ docker-compose -f docker-compose.local.yml up postgres
```

### Without Docker ###

```bsh
$ cargo run
```

(Recommend) If using [cargo-watch](https://docs.rs/crate/cargo-watch/):

```console
$ cargo watch -x run
```

## Test, Lint and Format ##

### Setup test ###

Some tests require DB connection, so you need up and migrate before testing.

```console
$ docker-compose -f docker-compose.test.yml up -d --wait
$ docker-compose -f docker-compose.test.yml exec api-test migration/target/debug/migration up
```

### Test without Docker ###

```console
$ env DATABASE_URL=postgres://admin:password@localhost:5442/GKBDB cargo test --all -- --nocapture --test-threads=1
```

### (Recommend) Test with Docker ###

```console
$ docker-compose -f docker-compose.test.yml exec api-test cargo test --all -- --nocapture --test-threads=1
```

### Lint ###

```console
$ cargo fmt --all -- --check
```

### Format ###

```console
$ cargo clippy --all-targets --all-features -- -D warnings -A clippy::needless_return
```

### Check all ###

```console
$ chmod +x check.sh
$ ./check.sh
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

See:

- [Setting Up Migration | SeaORM 🐚 An async & dynamic ORM for Rust](https://www.sea-ql.org/SeaORM/docs/next/migration/setting-up-migration/)
- [Writing Migration | SeaORM 🐚 An async & dynamic ORM for Rust](https://www.sea-ql.org/SeaORM/docs/next/migration/writing-migration/)
- [Running Migration | SeaORM 🐚 An async & dynamic ORM for Rust](https://www.sea-ql.org/SeaORM/docs/next/migration/running-migration/)
- [Seeding Data | SeaORM 🐚 An async & dynamic ORM for Rust](https://www.sea-ql.org/SeaORM/docs/next/migration/seeding-data/)

### Create ###

```console
$ cd migration/
$ target/debug/migration generate <migration name>
```

### Run ###

Check status:

```console
$ migration/target/debug/migration status
```

And run:

```console
$ migration/target/debug/migration up
```

## Development ##

### Install tools ###

If use only Docker, you need not to install these tools, because there are already installed in `Dockerfile-local`.

```console
$ rustup component add rustfmt
$ rustup component add clippy
$ cargo install cargo-watch cargo-edit
```

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
