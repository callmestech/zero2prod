test:
  cargo test

fmt:
  cargo fmt --all

fmt-check:
  cargo fmt --all -- --check

watch:
  cargo watch -x run -q | bunyan
