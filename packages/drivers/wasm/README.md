# RPGX WASM Driver

This package provides WebAssembly (WASM) bindings for the RPGX game engine, enabling integration with web-based frontends such as Node.js and Vue.js.

## Prerequisites

- Rust toolchain with `wasm32-unknown-unknown` target
- [`wasm-bindgen-cli`](https://github.com/rustwasm/wasm-bindgen)

Install the target and CLI:

```sh
rustup target add wasm32-unknown-unknown
cargo install --locked wasm-bindgen-cli
```

## Building

Build the WASM driver:

```sh
make build-wasm-driver
```

## Bundling for Playgrounds

To generate the WASM driver and Node.js loader (used by both Node.js and Vue.js playgrounds):

```sh
make build-js-driver
```

## Usage

Import the generated WASM module in your JavaScript/TypeScript frontend as per the [wasm-bindgen documentation](https://rustwasm.github.io/docs/wasm-bindgen/).

## License

RPGX is licensed under the [MIT License](../../../LICENSE).