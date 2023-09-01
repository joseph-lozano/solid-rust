dev: install_deps
    @concurrently -n "vite,rust" "pnpm run --prefix frontend dev" "cargo watch -x run"
install_deps:
    pnpm install --prefix frontend
setup: install_deps
    pnpm install -g concurrently
build: install_deps
    pnpm run --prefix frontend build
    cargo build --release
preview: build
    ./target/release/solid-rust
check: install_deps
    npx prettier --check .
    cargo clippy