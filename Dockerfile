FROM rust:1.63.0 as build
COPY . .
RUN cargo build --
ENTRYPOINT ["/bin/sh", "-c", "./fuelPriceBot"]
