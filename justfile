install_wasm_bindgen_cli := "cargo install wasm-bindgen-cli"

default path: build
  @./target/release/bf-jit-wasm {{path}}

debug path:
  @cargo run -- {{path}}

build:
  @cargo build --release

build-web: build-wasm32
  @{{install_wasm_bindgen_cli}}
  @wasm-bindgen target/wasm32-unknown-unknown/debug/bf_jit_wasm.wasm \
    --out-dir target/web \
    --target web

build-wasm32:
  @cargo build --lib --target wasm32-unknown-unknown