# RPGX

<img src="https://s3.rottigni.tech/rpgx/rpgx_logo_transparent.webp" alt="RPGX Logo" width="400" />

RPGX is a lightweight, modular, and extensible RPG game engine written in Rust, designed for flexibility and ease of use. It provides a rich grid-based architecture with layered maps, tile effects, pathfinding, and entity movement logic.

Built with modern Rust paradigms, RPGX is distributed both as a native Rust crate and as a WebAssembly (WASM) module via `wasm-bindgen`, making it seamlessly integrable into Node.js environments and browser applications.

RPGX is fully compatible with [Dioxus](https://dioxuslabs.com/), a powerful Rust-based UI framework, enabling smooth rendering and interactive frontend experiences for games built on top of the engine.

This combination of Rust’s performance, WASM’s portability, and Dioxus’s reactive UI system provides a robust foundation for developing cross-platform RPG games that can run natively, on the web, or in hybrid contexts.

## 🚀 Getting Started

> ⚠️ **Note:** RPGX is still under active development and not yet published to public registries. To use it, you may need to reference the source directly from GitHub.

---

### 🦀 Rust (Native)

Install the core engine crate:

```sh
cargo add rpgx
```

Or add it manually in your Cargo.toml:

```toml
[dependencies]
rpgx = { git = "https://github.com/LorenzoRottigni/rpgx", package = "rpgx" }
```

Docs: https://github.com/LorenzoRottigni/rpgx/tree/master/packages/rpgx

### 🌐 Node.js / WebAssembly

Install the WebAssembly driver for use in JS/TS:

```sh
npm install rpgx-wasm
# or
yarn add rpgx-wasm
# or
pnpm install rpgx-wasm
```

Docs: https://github.com/LorenzoRottigni/rpgx/tree/master/packages/drivers/wasm

### 🧱 C++ (WASM interop)

🚧 Planned: A C++-friendly wrapper using wasm-bindgen-cxx or cxx, to allow integration with C++ game engines and UIs like Qt or Unreal.

## Glossary

- [Effect](./packages/core/docs/effect.md)
- [Tile](./packages/core/docs/tile.md)
- [Mask](./packages/core/docs/mask.md)
- [Layer](./packages/core/docs/layer.md)
- [Map](./packages/core/docs/map.md)
- [Scene](./packages/core/docs/scene.md)
- [Engine](./packages/core/docs/engine.md)


## Contributing

### 🛠 Development Setup

Requirements:

- [Rust](https://rustup.rs) (with `wasm32-unknown-unknown` target)
- [Node.js](https://nodejs.org)
- [wasm-bindgen CLI](https://rustwasm.github.io/docs/wasm-bindgen/):

  ```sh
  cargo install wasm-bindgen-cli
  ```

- [Dioxus CLI](https://dioxuslabs.com/):

  ```sh
  cargo install --locked dioxus-cli
  ```

Clone the repo and initialize your environment using the provided `Makefile`.

---

### 🚀 Common Development Workflows

#### Run RPGX in Vue Playground

```sh
make dev-vue
```

- Builds the `rpgx-wasm` driver bundle.
- Generates JS bindings using `wasm-bindgen`.
- Launches the Vue.js playground.

#### Run RPGX in Node.js Playground

```sh
make dev-node
```

- Builds and binds the WASM module.
- Starts the Node.js script using TypeScript.

#### Run RPGX in Dioxus (Web/Desktop)

```sh
make dev-dioxus-web     # For web browser rendering
make dev-dioxus-desktop # For native desktop app
```

---

### 🧪 Testing

Run all unit tests for the Rust core and WASM crates:

```sh
make test-core     # Tests for core engine (rpgx)
make test-wasm     # Tests for WASM driver (rpgx-wasm)
```

---

### 🧹 Clean Up Build Artifacts

```sh
make clean
```

This removes compiled files, WASM output, and `node_modules` from playgrounds.

---

### 📦 Production Builds

#### Core Engine (Native)

```sh
make build-core
```

#### WASM Package

```sh
make build-wasm
```

#### Vue Production Build

```sh
make build-vue
```

#### Dioxus Builds

```sh
make build-dioxus-web
make build-dioxus-desktop
```

## License

RPGX is licensed under the [MIT License](LICENSE). You are free to use, modify, and distribute it under the terms of this license.