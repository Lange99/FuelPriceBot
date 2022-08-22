FROM rust:1.63.0 as build
COPY . .
RUN cargo build --release

FROM ubuntu:20.04
COPY --from=build /target/release/fuelPriceBot .
RUN apt-get -y update
RUN apt-get -y install ca-certificates

ENTRYPOINT ["./fuelPriceBot"]
