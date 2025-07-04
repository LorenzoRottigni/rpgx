# RPGX Dioxus Plugin

This package provides the Dioxus integration for the RPGX game engine, enabling reactive user interfaces for grid-based RPGs using the [Dioxus](https://dioxuslabs.com/) Rust framework.

## Features

- Seamless integration with RPGX core engine
- Reactive UI components for RPGs
- Support for both desktop and web platforms via Dioxus

## Example

```rust
use rpgx::prelude::*;
use rpgx_dioxus::components::engine::Engine as EngineComponent;
let library = Library::new(/* Configure library */)
let engine = Engine::new(/* Configure engine */)
rsx! {
    div { class: "rpgx",
        EngineComponent {
            engine: engine.clone(),
            library: library.clone(),
            square_size: 30,
        }
    }
}
```

## Prerequisites

- Rust toolchain
- [`dioxus-cli`](https://github.com/DioxusLabs/cli) for development and building

Install Dioxus CLI:

```sh
cargo install --locked dioxus-cli
```

## Development

To start the Dioxus application in development mode:

```sh
make dev-dioxus-desktop
# or for web
make dev-dioxus-web
```

## Building

To build the Dioxus application for production:

```sh
make build-dioxus-desktop
# or for web
make build-dioxus-web
```

## Usage

Import and use the Dioxus driver components in your Dioxus app to render and interact with RPGX game state.

## License

RPGX is licensed under the [MIT License](../../../LICENSE).