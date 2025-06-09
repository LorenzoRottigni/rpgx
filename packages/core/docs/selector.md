# Selector

`Selector` defines how tiles on a grid are targeted for applying effects or game logic. It supports selecting:

- **Single tiles** by their coordinates.
- **Rectangular blocks** of tiles via two corner coordinates.
- **Custom filtered selections** using user-defined functions.

## Type Aliases

- **SingleSelector**: A single tile coordinate.
- **BlockSelector**: A rectangular area defined by two opposite corner coordinates.
- **FilterSelector**: A function that filters tiles based on coordinates and shape.

## Enum Variants

- `Single(SingleSelector)`: Select a single tile.
- `Block(BlockSelector)`: Select a rectangular block of tiles.
- `Filter(FilterSelector)`: Select tiles based on custom filter logic.

This flexible selector system enables precise targeting of tiles for rendering, effects, interaction, or AI.
