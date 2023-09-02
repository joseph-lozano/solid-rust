# Solid + Rust

## Quick start

1. Install [Rust](https://rustup.rs) and [Node](https://nodejs.org/en).

2. Install [just](https://github.com/casey/just):

3. Install needed dependencies:

4. Run dev server. This will automatically download node dependencies.

```bash
cargo install just
just setup
just dev
```

## Depoloyment

Just use the Dockerfile. I have tested deployment on [Railway](https://railway.app) at https://solid-rust.up.railway.app/.

### I want to use React

Delete `frontend/` and re-create it with `pnpm create vite`, selecting `React`
You may need to edit the index.html template in `src/main.rs`
if the generated assets are named something other than `index.css` or `index.js`

### I want to rename the project

Replace `solid-rust` with your project name in the follwing files:

- `justfile`
- `Cargo.toml`
- `Cargo.lock`
- `Dockerfile`

## Contributing

PRs welcome.
