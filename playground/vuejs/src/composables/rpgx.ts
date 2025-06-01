// import * as wasm from '../wasm/rpgx_js';
import { WasmLibrary, WasmBlockSelector, WasmCoordinates, WasmEffect, WasmEngine, WasmLayer, WasmLayerType, WasmMap, WasmMask, WasmPawn, WasmScene, WasmSelector, WasmShape, WasmSingleSelector, WasmTile} from '@rpgx/js'

export function useLibrary(): WasmLibrary {
    const library = new WasmLibrary();
    library.insert(
        "floor_1",
        "https://s3.rottigni.tech/rpgx/spaceship_floor_1.webp"
    )
    library.insert(
        "floor_2",
        "https://s3.rottigni.tech/rpgx/spaceship_floor_2.webp"
    )
    library.insert(
        "building_1",
        "https://s3.rottigni.tech/rpgx/processor_9.webp"
    )
    library.insert(
        "building_2",
        "https://s3.rottigni.tech/rpgx/processor_8.webp"
    )
    library.insert(
        "character_1",
        "https://s3.rottigni.tech/rpgx/character_1.webp"
    )
    library.insert(
        "logit",
        () => {
            console.log("logit")
            return ""
        }
    )
    return library
}

export function useEngine(library: WasmLibrary): WasmEngine {
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
                
                WasmSelector.block(
                    new WasmBlockSelector(new WasmSingleSelector(0, 0), new WasmSingleSelector(grid_size - 1, grid_size - 1))
                ),
                new WasmEffect(library.get_id("floor_1"), undefined, undefined, false, false, null),
            ),
            new WasmMask(
                "floor_alt",
                WasmSelector.block(
                    new WasmBlockSelector(new WasmSingleSelector(0, 0), new WasmSingleSelector(0, grid_size - 1)),
                ),
                new WasmEffect(library.get_id("floor_2"), undefined, undefined, false, false, null),
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
                WasmSelector.block(
                    new WasmBlockSelector(new WasmSingleSelector(1, 6), new WasmSingleSelector(4, 11)),
                ),
                new WasmEffect(library.get_id("building_1"), undefined, undefined, true, true, null),
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
                WasmSelector.block(
                    new WasmBlockSelector(new WasmSingleSelector(10, 0), new WasmSingleSelector(11, 0)),
                ),
                new WasmEffect(library.get_id("floor_2"), library.get_id("logit"), undefined, false, false, null),
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
            new WasmEffect(undefined, undefined, undefined, false, false, null),
            new WasmCoordinates(0, 0),
            new WasmShape(1, 1),
        ),
        library.get_id("character_1") || NaN,
    );

    const scene = new WasmScene(
        "default",
        map,
        pawn
    )

    const engine = new WasmEngine(scene)

    return engine
}