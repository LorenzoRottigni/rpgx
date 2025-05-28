// import * as wasm from '../wasm/rpgx_js';
import { ResourceLibrary, WasmCoordinates, WasmEffect, WasmEngine, WasmLayer, WasmLayerType, WasmMap, WasmMask, WasmPawn, WasmSelector, WasmShape, WasmTile} from '@rpgx/js'

export function useLibrary(): ResourceLibrary {
    const library = new ResourceLibrary();
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

export function useEngine(library: ResourceLibrary): WasmEngine {
    const grid_size = 25;

    // const defaultLayer = new WasmLayer(
    //     "base",
    //     WasmLayerType.Base,
    //     new WasmShape(grid_size, grid_size),
    //     [],
    //     1
    // )

    const groundLayer = new WasmLayer(
        "ground",
        WasmLayerType.Texture,
        new WasmShape(grid_size, grid_size),
        [
            new WasmMask(
                "default_floor",
                new WasmEffect(library.get_key_id("floor_1"), undefined, false, false, null),
                WasmSelector.new_block(
                    new WasmCoordinates(0, 0),
                    new WasmCoordinates(grid_size - 1, grid_size - 1),
                ),
            ),
            new WasmMask(
                "floor_alt",
                new WasmEffect(library.get_key_id("floor_2"), undefined, false, false, null),
                WasmSelector.new_block(
                    new WasmCoordinates(0, 0),
                    new WasmCoordinates(0, grid_size - 1),
                ),
            )
        ],
        1
    )

    const buildingLayer = new WasmLayer(
        "building",
        WasmLayerType.Block,
        new WasmShape(grid_size, grid_size),
        [
            new WasmMask(
                "default_building",
                new WasmEffect(library.get_key_id("building_1"), undefined, true, true, null),
                WasmSelector.new_block(
                    new WasmCoordinates(1, 6),
                    new WasmCoordinates(4, 11),
                ),
            ),
        ],
        5
    )

    const actionLayer = new WasmLayer(
        "action",
        WasmLayerType.Action,
        new WasmShape(grid_size, grid_size),
        [
            new WasmMask(
                "logit",
                new WasmEffect(library.get_key_id("floor_2"), library.get_key_id("logit"), false, false, null),
                WasmSelector.new_block(
                    new WasmCoordinates(10, 0),
                    new WasmCoordinates(11, 0),
                ),
            )
        ],
        6
    )

    const map = new WasmMap(
        "test_map",
        [
            groundLayer,
            buildingLayer,
            actionLayer
        ]
    )

    const pawn = new WasmPawn(
        new WasmTile(
            0,
            new WasmEffect(undefined, undefined, false, false, null),
            new WasmCoordinates(0, 0),
            new WasmShape(1, 1),
        ),
        library.get_key_id("character_1"),
    );

    const engine = new WasmEngine(map, pawn)

    return engine
}