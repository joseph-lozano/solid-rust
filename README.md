# Solid + Rust

## Usage
`cd frontend`, `npm install`, then run `npm run dev`

In another terminal, run `cargo run` in the main directory.

Navigate to http://localhost:3000

## Depoloyment

Just use the Dockerfile

## I want to use React
Delete `frontend/` and re-create it with `npm crate vite`, selecting `React`
You may need to edit the index.html template in `src/main.rs`
if the generated assets are named something other than `index.css` or `index.js`

## Contributing
PRs welcome.