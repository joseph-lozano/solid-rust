FROM node:18 AS node_builder

WORKDIR /app
COPY frontend/package.json frontend/package-lock.json frontend/
WORKDIR /app/frontend/
RUN npm install
COPY frontend/ /app/frontend/
RUN npm run build

FROM rust:1.71 AS rust_builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY --from=node_builder /app/frontend/dist ./frontend/dist
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=rust_builder /app/target/release/solid-rust .
COPY --from=node_builder /app/frontend/dist ./frontend/dist

ENTRYPOINT [ "./solid-rust" ]

