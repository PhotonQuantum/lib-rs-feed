FROM rust:slim-bullseye AS builder

WORKDIR /work

RUN apt-get -y update

RUN apt-get -y install pkg-config libssl-dev ca-certificates

COPY src ./src

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /work

RUN apt-get -y update

RUN apt-get -y install ca-certificates

COPY --from=builder ./work/target/release/lib-rs-feed ./

CMD ["./lib-rs-feed"]