FROM node:18 AS node_builder
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

WORKDIR /app
COPY frontend/package.json frontend/pnpm-lock.yaml frontend/

WORKDIR /app/frontend/
RUN pnpm install

COPY frontend/ /app/frontend/
RUN pnpm run build

FROM rust:1.72 AS rust_builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=rust_builder /app/target/release/solid-rust .
COPY --from=node_builder /app/frontend/dist ./frontend/dist

ENTRYPOINT [ "./solid-rust" ]

