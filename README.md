# RPGX

<img src="https://s3.rottigni.tech/rpgx/rpgx_logo_transparent.webp" alt="RPGX Logo" width="400" />

RPGX is a lightweight, modular, and extensible RPG game engine written in Rust, designed for flexibility and ease of use. It provides a rich grid-based architecture with layered maps, tile effects, pathfinding, and entity movement logic.

Built with modern Rust paradigms, RPGX is distributed both as a native Rust crate and as a WebAssembly (WASM) module via `wasm-bindgen`, making it seamlessly integrable into Node.js environments and browser applications.

RPGX is fully compatible with [Dioxus](https://dioxuslabs.com/), a powerful Rust-based UI framework, enabling smooth rendering and interactive frontend experiences for games built on top of the engine.

This combination of Rust’s performance, WASM’s portability, and Dioxus’s reactive UI system provides a robust foundation for developing cross-platform RPG games that can run natively, on the web, or in hybrid contexts.

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
- Returning detailed movement errors (`MoveError`) on failure.
- Enabling asynchronous walking for smooth movement sequences.

Typical usage involves creating an `Engine` with a map and pawn, then calling movement methods like `move_to`, `step_to`, and `walk_to`.

---

