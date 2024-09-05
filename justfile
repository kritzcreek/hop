default:
  just --list

install:
  cargo run --bin xtask-gen
  cargo install --path crates/hop

fmt:
  cargo fmt --all

test:
  cargo test --all
  cargo clippy --all-features --all-targets
  cargo fmt --all --check
