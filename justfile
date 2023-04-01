release:
  cargo clean --release
  cargo build --release

test:
  cargo test -- --nocapture

docs:
  cargo doc --no-deps

run example:
  cargo run --example {{example}}

fmt:
  rustfmt src/**/*.rs tests/*.rs

clippy:
  cargo clippy

all:
  just fmt
  just clippy
  just test
  just docs