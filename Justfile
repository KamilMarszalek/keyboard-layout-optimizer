default:
    just --list

# ---------- Rust ----------

rust-fmt:
    cd optimizer && cargo fmt --check

rust-fmt-fix:
    cd optimizer && cargo fmt

rust-clippy:
    cd optimizer && cargo clippy --all-targets --all-features -- -D warnings

rust-test:
    cd optimizer && cargo test --all-features --locked

rust-build:
    cd optimizer && cargo build --locked

rust-doc:
    cd optimizer && cargo doc --no-deps

rust-demo:
    cd optimizer && cargo run --example demo

wasm-target:
    rustup target add wasm32-unknown-unknown

wasm-build:
    cd optimizer && cargo build --target wasm32-unknown-unknown --locked

wasm-pack:
    cd optimizer && wasm-pack build --target web --out-dir ../frontend/src/wasm

rust-check: rust-fmt rust-clippy rust-test wasm-build

# ---------- Frontend ----------

frontend-install:
    cd frontend && npm ci

frontend-lint:
    cd frontend && npm run lint

frontend-lint-fix:
    cd frontend && npm run lint:fix

frontend-type-check:
    cd frontend && npm run type-check

frontend-test:
    cd frontend && npm run test -- --run

frontend-build:
    cd frontend && npm run build

frontend-dev:
    cd frontend && npm run dev

frontend-check: frontend-lint frontend-type-check frontend-test frontend-build

# ---------- All ----------

check: rust-check frontend-check

test: rust-test frontend-test

docs: rust-doc

setup: wasm-target frontend-install

clean:
    cd optimizer && cargo clean
    rm -rf frontend/dist frontend/src/wasm