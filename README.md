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

### Effect

An `Effect` encapsulates visual and interactive properties that can be applied to a tile within the RPGX engine. It provides a unified way to attach optional behavior or presentation logic to game entities.

Key features include:

- Associating an action (e.g., script, command) with a tile or UI element.
- Assigning a texture identifier for visual representation.
- Marking a tile as blocking for movement or interaction.
- Grouping multiple tiles as a logical unit.
- Applying refined collision areas via a `shrink` region.

---

### Selector

A `Selector` provides a flexible mechanism to target tiles within the RPGX engine. It supports addressing individual tiles, rectangular regions, or dynamically filtering tiles based on custom logic.

This system is foundational for applying effects, querying tile zones, and managing user interactions within the grid.

RPGX currently supports three types of selectors:

- `SingleSelector`: Targets a single tile by exact coordinates.
- `BlockSelector`: Selects a rectangular area defined by two opposite corner coordinates.
- `FilterSelector`: Uses a custom filter callback to select tiles by evaluating predicates over their coordinates. Useful for fog of war, zone control, terrain filtering, and more.

---

### Tile

A `Tile` represents a single grid cell in the RPGX engine, holding both spatial and behavioral information.

Each tile includes:

- A unique identifier.
- Visual and interactive `Effect`s.
- Position represented by coordinates.
- Shape and area coverage on the grid.

---

### Mask

A `Mask` defines logical zones over a `Layer` where specific effects are applied via the `Selector` pattern. It enables batch creation or modification of tiles based on spatial or functional rules, useful in both level design and runtime interactions.

---

### Layer

A `Layer` is a visual or logical overlay composed of one or more tiles, covering the dimensions of an abstract grid.

Layers allow modular construction of complex scenes and systems, such as:

- Terrain textures.
- Interactive zones.
- Collision masks.
- Logic triggers.

---

### Map

The `Map` is the central structure representing a named, grid-based environment composed of multiple `Layer`s. Each layer serves a specific purpose (e.g., visuals, collision, logic), and together they form a cohesive game world.

Responsibilities of a `Map` include:

- Managing multiple layers with distinct roles.
- Providing methods to retrieve and manipulate tiles.
- Enabling queries for blocking tiles and pathfinding.
- Serving as the spatial context for gameplay.

---

### Pawn

A `Pawn` represents an entity or character positioned on the map, tied to a specific `Tile`. It encapsulates both the entity’s current position and a visual representation identifier.

---

### Engine

The `Engine` is the core RPG system that manages a `Pawn` navigating across a `Map`. It handles pathfinding, move validation, and progression of the pawn through the game world.

Key responsibilities include:

- Maintaining the current `Map` and active `Pawn`.
- Computing optimal paths and validating moves.
- Supporting step-wise and direct movements with blocking checks.
- Returning detailed movement errors (`RPGXError`) on failure.
- Enabling asynchronous walking for smooth movement sequences.

Typical usage involves creating an `Engine` with a map and pawn, then calling movement methods like `move_to`, `step_to`, and `walk_to`.

---


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