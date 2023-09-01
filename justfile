dev: setup
    npx -y concurrently -n "vite,rust" "npm run --prefix frontend dev" "cargo watch -x run"
setup:
    npm install --prefix frontend
    cargo build
 
