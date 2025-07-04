FROM rust:1.84 as builder

RUN USER=root cargo new --bin rust-bff
WORKDIR ./rust-bff
COPY ./Cargo.toml ./Cargo.toml

ADD . ./
RUN cargo build --release

FROM debian:bookworm-slim
ARG APP=/usr/src/app

RUN apt-get update \
  && apt-get install -y ca-certificates tzdata openssl libssl-dev \
  && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

ENV TZ=Etc/UTC \
  APP_USER=appuser

RUN groupadd $APP_USER \
  && useradd -g $APP_USER $APP_USER \
  && mkdir -p ${APP}

COPY --from=builder /rust-bff/target/release/rust-bff ${APP}/rust-bff

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./rust-bff"]
