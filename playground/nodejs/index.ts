import { WasmLayer, WasmMask, WasmShape, WasmEffect, WasmSelector, WasmLayerType, WasmCoordinates, WasmMap, WasmEngine } from '@rpgx/js';
/* TODO: remake nodejs playground
const grid_size = 25;

const defaultLayer = new WasmLayer(
    "base",
    LayerType.Default,
    new WasmShape(grid_size, grid_size),
    []
)

const groundLayer = new WasmLayer(
    "ground",
    WasmLayerType.Texture,
    new WasmShape(grid_size, grid_size),
    [
        new WasmMask(
            "default_floor",
            new WasmEffect("texture.png", false, false),
            WasmSelector.new_block(
                new WasmCoordinates(0, 0),
                new WasmCoordinates(grid_size - 1, grid_size - 1),
            ),
        ),
        new WasmMask(
            "floor_alt",
            new WasmEffect("texture.png", false, false),
            WasmSelector.new_block(
                new WasmCoordinates(0, 0),
                new WasmCoordinates(0, grid_size - 1),
            ),
        )
    ]
)

const buildingLayer = new WasmLayer(
    "building",
    WasmLayerType.Block,
    new WasmShape(grid_size, grid_size),
    [
        new WasmMask(
            "default_building",
            new WasmEffect("texture.png", true, true),
            WasmSelector.new_block(
                new WasmCoordinates(1, 6),
                new WasmCoordinates(4, 11),
            ),
        ),
    ]
)

const map = new WasmMap(
    "test_map",
    [
        defaultLayer,
        groundLayer,
        buildingLayer,
    ]
)

const pawn = new WasmPawn(
    new WasmTile(
        0,
        new WasmEffect(null, false, false),
        new WasmCoordinates(0, 0),
        new WasmShape(1, 1),
    ),
    ""
);

const engine = new WasmEngine(map, pawn)

console.log(engine.pawn_position)
console.dir(engine)
engine.move_to(3,3)
console.log(engine.pawn_position)
console.dir(Object.keys(engine))
*/