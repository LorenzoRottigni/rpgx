import * as wasm from '../wasm/rpgxw.js';

export function useMasks() {
    return [
            new wasm.Mask(
                "default_floor",
                new wasm.Effect("https://s3.rottigni.tech/rpgx/spaceship_floor_1.webp", false, false),
                wasm.Selector.new_block(
                    new wasm.Coordinates(0, 0),
                    new wasm.Coordinates(25 - 1, 25 - 1),
                ),
            ),
            new wasm.Mask(
                "floor_alt",
                new wasm.Effect("https://s3.rottigni.tech/rpgx/spaceship_floor_2.webp", false, false),
                wasm.Selector.new_block(
                    new wasm.Coordinates(0, 0),
                    new wasm.Coordinates(0, 25 - 1),
                ),
            )
        ]
}

export function useLayer() {
    return new wasm.Layer(
        "ground",
        wasm.LayerType.Texture,
        new wasm.Shape(25, 25),
        [
            new wasm.Mask(
                "default_floor",
                new wasm.Effect("https://s3.rottigni.tech/rpgx/spaceship_floor_1.webp", false, false),
                wasm.Selector.new_block(
                    new wasm.Coordinates(0, 0),
                    new wasm.Coordinates(25 - 1, 25 - 1),
                ),
            ),
            new wasm.Mask(
                "floor_alt",
                new wasm.Effect("https://s3.rottigni.tech/rpgx/spaceship_floor_2.webp", false, false),
                wasm.Selector.new_block(
                    new wasm.Coordinates(0, 0),
                    new wasm.Coordinates(0, 25 - 1),
                ),
            )
        ]
    )
}

export function useMap() {
    return new wasm.Map(
        "test_map",
        [
            useLayer()
        ]
    )
}

export function useRpgxwEngine() {
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
                new wasm.Effect("https://s3.rottigni.tech/rpgx/spaceship_floor_1.webp", false, false),
                wasm.Selector.new_block(
                    new wasm.Coordinates(0, 0),
                    new wasm.Coordinates(grid_size - 1, grid_size - 1),
                ),
            ),
            new wasm.Mask(
                "floor_alt",
                new wasm.Effect("https://s3.rottigni.tech/rpgx/spaceship_floor_2.webp", false, false),
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
                new wasm.Effect("https://s3.rottigni.tech/rpgx/k8sville_1.webp", true, true),
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
        "https://s3.rottigni.tech/rpgx/character_1.webp"
    );

    const engine = new wasm.WasmEngine(map, pawn)

    return engine
}