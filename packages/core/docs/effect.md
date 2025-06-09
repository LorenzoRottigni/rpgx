# Effect

`Effect` represents visual and interactive properties applied to a [`Tile`](./tile.md) or UI element.

## Fields

- `action_id: Option<u32>`  
  Optional ID linking the tile to an action.

- `texture_id: Option<u32>`  
  Optional ID for a texture attached to the tile.

- `render_id: Option<u32>`  
  Optional ID for a rendering callback associated with the tile.

- `block: bool`  
  Indicates if the tile blocks movement or interaction.

- `group: bool`  
  Marks if the tile belongs to a group of contiguous tiles.

- `shrink: Option<BlockSelector>`  
  Optional bounding box limiting the blocking region on the tile.

## Methods

- `shrink_contains(&self, point: Coordinates) -> bool`  
  Returns whether a given coordinate is within the `shrink` region, if defined.  
  If no shrink is defined, always returns `true`.

---

See also: [`Tile`](./tile.md), [`Coordinates`](./coordinates.md), [`Shape`](./shape.md), [`BlockSelector`](./block_selector.md)
