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
                new wasm.Effect(library.get_key_id("floor_1"), undefined, false, false),
                wasm.Selector.new_block(
                    new wasm.Coordinates(0, 0),
                    new wasm.Coordinates(grid_size - 1, grid_size - 1),
                ),
            ),
            new wasm.Mask(
                "floor_alt",
                new wasm.Effect(library.get_key_id("floor_2"), undefined, false, false),
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
                new wasm.Effect(library.get_key_id("building_1"), undefined, true, true),
                wasm.Selector.new_block(
                    new wasm.Coordinates(1, 6),
                    new wasm.Coordinates(4, 11),
                ),
            ),
        ]
    )

    const actionLayer = new wasm.Layer(
        "action",
        wasm.LayerType.Action,
        new wasm.Shape(grid_size, grid_size),
        [
            new wasm.Mask(
                "logit",
                new wasm.Effect(library.get_key_id("floor_2"), library.get_key_id("logit"), false, false),
                wasm.Selector.new_block(
                    new wasm.Coordinates(10, 0),
                    new wasm.Coordinates(11, 0),
                ),
            )
        ]
    )

    const map = new wasm.Map(
        "test_map",
        [
            defaultLayer,
            groundLayer,
            buildingLayer,
            actionLayer
        ]
    )

    const pawn = new wasm.Pawn(
        new wasm.Tile(
            0,
            new wasm.Effect(undefined, undefined, false, false),
            new wasm.Coordinates(0, 0),
            new wasm.Shape(1, 1),
        ),
        library.get_key_id("character_1"),
    );

    const engine = new wasm.WasmEngine(map, pawn)

    return engine
}