release:
  cargo clean --release
  cargo build --release

test:
  cargo test -- --nocapture

run example:
  cargo run --example {{example}}

fmt:
  rustfmt src/**/*.rs tests/*.rs