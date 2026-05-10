set unstable #enabled for the use of which() function
install_wasm_bindgen_cli := if which("wasm-bindgen") != "" {
  "echo ✅ wasm-bindgen-cli is already installed"
} else {
  "cargo install wasm-bindgen-cli"
}

run path="examples/hello.bf": build
  @./target/release/bf-jit-wasm {{path}}

run-debug path:
  @cargo run -- {{path}}

build:
  @cargo build --release

build-web: build-wasm32
  @{{install_wasm_bindgen_cli}}
  @wasm-bindgen target/wasm32-unknown-unknown/debug/bf_jit_wasm.wasm \
    --out-dir target/web \
    --target web
  @cp src/web/* target/web/
  @mkdir target/web/examples
  @cp examples/* target/web/examples/

build-wasm32:
  @cargo build --lib --target wasm32-unknown-unknown