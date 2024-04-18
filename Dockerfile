FROM rust:1.75-slim-bullseye as build

RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
      protobuf-compiler

ARG APP_NAME=ploy-engine

WORKDIR /build

COPY Cargo.lock Cargo.toml ./

RUN mkdir src \
    && echo "fn main() {println!(\"Hello, world!\");}" > src/main.rs \
    && cargo build --release

COPY proto proto
COPY src src
COPY build.rs build.rs

RUN cargo build --locked --release
RUN cp ./target/release/$APP_NAME /bin/server

FROM debian:bullseye-slim AS final

WORKDIR /app

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home /nonexistent \
    --shell /sbin/nologin \
    --no-create-home \
    --uid 10001 \
    appuser

RUN echo "" > ploy-engine.log
RUN chown appuser ploy-engine.log

USER appuser

COPY --from=build /bin/server /bin/
COPY log4rs.yaml log4rs.yaml
COPY data data
COPY Lib Lib

CMD ["/bin/server"]
