test:
  cargo test  

test-dbg:
  TEST_LOG=debug cargo test | bunyan

fmt:
  cargo fmt --all

fmt-check:
  cargo fmt --all -- --check

watch:
  cargo watch -x run -q | bunyan
