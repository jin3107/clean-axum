# syntax=docker/dockerfile:1
FROM rust:1-slim-bookworm AS build
WORKDIR /src
COPY . .
# the cache mounts keep downloaded crates and compiled output between builds
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/target \
    cargo build --release --locked --bin {{project-name}} \
    && cp target/release/{{project-name}} /usr/local/bin/server

FROM debian:bookworm-slim
RUN useradd --system --uid 10001 app
COPY --from=build /usr/local/bin/server /usr/local/bin/server
USER app
ENV HOST=0.0.0.0 PORT=3000
EXPOSE 3000
CMD ["server"]
