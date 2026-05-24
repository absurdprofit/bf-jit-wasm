set unstable #enabled for the use of which() function
install_wasm_bindgen_cli := if which("wasm-bindgen") != "" {
  "echo ✅ wasm-bindgen-cli is already installed"
} else {
  "cargo install wasm-bindgen-cli"
}

install_wasm_opt_cli := if which("wasm-opt") != "" {
  "echo ✅ wasm-opt is already installed"
} else {
  "cargo install wasm-opt"
}


run path="examples/hello.bf": build
  @./target/release/bf-jit-wasm {{path}}

run-debug path:
  @cargo run -- {{path}}

build:
  @cargo build --release

build-web: build-wasm32
  @{{install_wasm_bindgen_cli}}
  @wasm-bindgen target/wasm32-unknown-unknown/release/bf_jit_wasm.wasm \
    --out-dir target/web \
    --target web

  @{{install_wasm_opt_cli}}
  @wasm-opt -O4 \
    -o target/web/bf_jit_wasm_bg.wasm \
    target/web/bf_jit_wasm_bg.wasm

  @cp src/web/* target/web/
  @mkdir -p target/web/examples
  @cp examples/* target/web/examples/

build-wasm32:
  @rustup target add wasm32-unknown-unknown
  @cargo build --release --lib --target wasm32-unknown-unknown