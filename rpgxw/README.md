cargo build --target wasm32-unknown-unknown --release -p rpgxw 
cargo install --locked wasm-bindgen-cli
wasm-bindgen target/wasm32-unknown-unknown/release/rpgxw.wasm --out-dir pkg --target nodejs
