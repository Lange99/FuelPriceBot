FROM rust:1.49 as build

RUN USER=root cargo new --bin langeFuel
WORKDIR /langeFuel

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/langeFuel*
RUN cargo build --release

FROM rust:1.49-slim-buster

ENV TELOXIDE_FUEL_TOKEN=$TELOXIDE_FUEL_TOKEN

COPY --from=build /langeFuel/target/release/langeFuel .

CMD ["./langeFuel"]