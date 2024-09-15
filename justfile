default:
  just --list

install:
  cargo install --path crates/hop-scip-cli --bin hop

build:
  cargo run --bin xtask-gen
  cargo fmt --all
  cargo build --all

test:
  cargo test --all
  cargo clippy --all-features --all-targets
  cargo fmt --all --check
