import * as wasm from '../wasm/rpgxw.js';

export function useLibrary(): wasm.ResourceLibrary {
    const library = new wasm.ResourceLibrary();
    library.insert_texture(
        "floor_1",
        "https://s3.rottigni.tech/rpgx/spaceship_floor_1.webp"
    )
    library.insert_texture(
        "floor_2",
        "https://s3.rottigni.tech/rpgx/spaceship_floor_2.webp"
    )
    library.insert_texture(
        "building_1",
        "https://s3.rottigni.tech/rpgx/processor_9.webp"
    )
    library.insert_texture(
        "building_2",
        "https://s3.rottigni.tech/rpgx/processor_8.webp"
    )
    library.insert_texture(
        "character_1",
        "https://s3.rottigni.tech/rpgx/character_1.webp"
    )
    library.insert_action(
        "logit",
        () => {
            console.log("logit")
        }
    )
    return library
}

export function useEngine(library: wasm.ResourceLibrary): wasm.WasmEngine {
    const grid_size = 25;

    // const defaultLayer = new wasm.WasmLayer(
    //     "base",
    //     wasm.WasmLayerType.Base,
    //     new wasm.WasmShape(grid_size, grid_size),
    //     [],
    //     1
    // )

    const groundLayer = new wasm.WasmLayer(
        "ground",
        wasm.WasmLayerType.Texture,
        new wasm.WasmShape(grid_size, grid_size),
        [
            new wasm.WasmMask(
                "default_floor",
                new wasm.WasmEffect(library.get_key_id("floor_1"), undefined, false, false, null),
                wasm.WasmSelector.new_block(
                    new wasm.WasmCoordinates(0, 0),
                    new wasm.WasmCoordinates(grid_size - 1, grid_size - 1),
                ),
            ),
            new wasm.WasmMask(
                "floor_alt",
                new wasm.WasmEffect(library.get_key_id("floor_2"), undefined, false, false, null),
                wasm.WasmSelector.new_block(
                    new wasm.WasmCoordinates(0, 0),
                    new wasm.WasmCoordinates(0, grid_size - 1),
                ),
            )
        ],
        1
    )

    const buildingLayer = new wasm.WasmLayer(
        "building",
        wasm.WasmLayerType.Block,
        new wasm.WasmShape(grid_size, grid_size),
        [
            new wasm.WasmMask(
                "default_building",
                new wasm.WasmEffect(library.get_key_id("building_1"), undefined, true, true, null),
                wasm.WasmSelector.new_block(
                    new wasm.WasmCoordinates(1, 6),
                    new wasm.WasmCoordinates(4, 11),
                ),
            ),
        ],
        5
    )

    const actionLayer = new wasm.WasmLayer(
        "action",
        wasm.WasmLayerType.Action,
        new wasm.WasmShape(grid_size, grid_size),
        [
            new wasm.WasmMask(
                "logit",
                new wasm.WasmEffect(library.get_key_id("floor_2"), library.get_key_id("logit"), false, false, null),
                wasm.WasmSelector.new_block(
                    new wasm.WasmCoordinates(10, 0),
                    new wasm.WasmCoordinates(11, 0),
                ),
            )
        ],
        6
    )

    const map = new wasm.WasmMap(
        "test_map",
        [
            groundLayer,
            buildingLayer,
            actionLayer
        ]
    )

    const pawn = new wasm.WasmPawn(
        new wasm.WasmTile(
            0,
            new wasm.WasmEffect(undefined, undefined, false, false, null),
            new wasm.WasmCoordinates(0, 0),
            new wasm.WasmShape(1, 1),
        ),
        library.get_key_id("character_1"),
    );

    const engine = new wasm.WasmEngine(map, pawn)

    return engine
}