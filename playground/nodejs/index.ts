import wasm from '../../pkg/rpgxw.js';

const grid_size = 25;

const defaultLayer = new wasm.Layer(
    "base",
    wasm.LayerType.Default,
    new wasm.Shape(grid_size, grid_size),
    []
)

const groundLayer = new wasm.Layer(
    "ground",
    wasm.LayerType.Texture,
    new wasm.Shape(grid_size, grid_size),
    [
        new wasm.Mask(
            "default_floor",
            new wasm.Effect("texture.png", false, false),
            wasm.Selector.new_block(
                new wasm.Coordinates(0, 0),
                new wasm.Coordinates(grid_size - 1, grid_size - 1),
            ),
        ),
        new wasm.Mask(
            "floor_alt",
            new wasm.Effect("texture.png", false, false),
            wasm.Selector.new_block(
                new wasm.Coordinates(0, 0),
                new wasm.Coordinates(0, grid_size - 1),
            ),
        )
    ]
)

const buildingLayer = new wasm.Layer(
    "building",
    wasm.LayerType.Block,
    new wasm.Shape(grid_size, grid_size),
    [
        new wasm.Mask(
            "default_building",
            new wasm.Effect("texture.png", true, true),
            wasm.Selector.new_block(
                new wasm.Coordinates(1, 6),
                new wasm.Coordinates(4, 11),
            ),
        ),
    ]
)

const map = new wasm.Map(
    "test_map",
    [
        defaultLayer,
        groundLayer,
        buildingLayer,
    ]
)

const pawn = new wasm.Pawn(
    new wasm.Tile(
        0,
        new wasm.Effect(null, false, false),
        new wasm.Coordinates(0, 0),
        new wasm.Shape(1, 1),
    ),
    ""
);

const engine = new wasm.WasmEngine(map, pawn)

console.dir(engine)