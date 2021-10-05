# For build
FROM rust:1.55.0 as build
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY . .
RUN cargo build --release

# For production
FROM build
COPY --from=build /app/target/release/anime-api .
