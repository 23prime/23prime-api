# For build
FROM rust:1.66.1 as build
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs

COPY entity ./entity
COPY migration ./migration

RUN cargo build --release

COPY . .
RUN cargo build --release

# For production
FROM rust:1.66.1
COPY --from=build /app/target/release/okkey-api .
