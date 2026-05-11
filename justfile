run-dev:
  cargo run --color always --message-format human

build-dev:
  cargo build --color always --message-format human

run:
  cargo run --release --color always --message-format human

build:
  cargo build --release --color always --message-format human

test:
  cargo test --color always --message-format human

clean:
  cargo clean

wasm-serve-dev:
  trunk serve

wasm-serve:
  trunk serve --release

wasm-build:
  trunk build --release
