FROM guilhermemarcello/gstreamer:rust-1.61 AS builder

RUN USER=root cargo new app

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release

COPY src src

RUN touch src/main.rs

RUN cargo build --release

FROM guilhermemarcello/gstreamer:debian-buster-slim AS base

COPY --from=builder /app/target/release/server-rust-gstreamer-rtp ./server-rust-gstreamer-rtp
CMD ["/server-rust-gstreamer-rtp"]
