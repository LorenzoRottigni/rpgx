cargo build --target wasm32-unknown-unknown --release -p rpgxw 
cargo install --locked wasm-bindgen-cli

Create wasm bundle for nodejs playground:
wasm-bindgen target/wasm32-unknown-unknown/release/rpgxw.wasm --out-dir playground/nodejs/wasm --target nodejs

Create wasm bundle for vuejs playground:

wasm-bindgen target/wasm32-unknown-unknown/release/rpgxw.wasm \
  --out-dir playground/vuejs/src/wasm \
  --target bundler
