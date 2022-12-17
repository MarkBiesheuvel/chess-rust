run:
  cargo run

release:
  cargo clean --release
  cargo build --release

test:
  cargo test -- --nocapture

fmt:
  rustfmt src/**/*.rs tests/*.rs