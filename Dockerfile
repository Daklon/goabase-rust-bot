FROM rust@sha256:1cdce1c7208150f065dac04b580ab8363a03cff7ddb745ddc2659d58dbc12ea8 as build

COPY ./ ./

RUN cargo build --release

RUN mkdir -p /build-out

RUN cp target/release/goabase-bot /build-out/

# Ubuntu 18.04
FROM ubuntu@sha256:5f4bdc3467537cbbe563e80db2c3ec95d548a9145d64453b06939c4592d67b6d

ENV DEBIAN_FRONTEND=noninteractive
ENV TELOXIDE_TOKEN=TOKEN
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=build /build-out/goabase-bot /

CMD /goabase-bot
