FROM rust:1.83.0-alpine3.20 AS base
RUN apk add --no-cache musl-dev git
RUN cargo install cargo-chef --locked --version 0.1.71
WORKDIR /app
RUN git config --global --add safe.directory /app
ARG FEATURES

FROM base AS dev
RUN cargo install cargo-watch --locked --version 8.5.3
VOLUME /app
VOLUME /app/.git
EXPOSE 25565
ENTRYPOINT ["cargo", "watch", "-x"]
CMD ["run -- server"]

FROM base AS planner
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo chef prepare --recipe-path recipe.json

FROM base AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --no-default-features --features $FEATURES
COPY src src
COPY build.rs .
COPY Cargo.toml .
COPY Cargo.lock .
COPY .git .git
RUN cargo build --release --no-default-features --features $FEATURES
RUN strip target/release/composition

FROM alpine:3.20 AS prod
RUN apk add --no-cache tini
RUN addgroup --gid 25565 --system composition && adduser --uid 25565 --system --ingroup composition --home /app composition
VOLUME /app/data
WORKDIR /app/data
COPY --from=builder /app/target/release/composition /app
EXPOSE 25565
USER composition
ENTRYPOINT ["tini", "--", "/app/composition"]
