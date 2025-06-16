import { Coordinates, Effect, Engine, Layer, Mask, Rect, Shape, Library, Map, Pawn, Scene } from "@rpgx/js";

export function useLibrary(): Library {
    const library = new Library();
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

export function useEngine(library: Library): Engine {
    const grid_size = 25;

    const groundLayer = new Layer(
        "ground",
        [
            new Mask(
                "default_floor",
                new Rect(new Coordinates(0,0), new Shape(grid_size - 1, grid_size - 1)).asMany(),
                new Effect(undefined, library.getId("floor_1"), undefined, null),
            ),
            new Mask(
                "floor_alt",
                new Rect(new Coordinates(0,0), new Shape(1,grid_size - 1)).asMany(),
                new Effect(undefined, library.getId("floor_2"), undefined, null),
            )
        ],
        1
    )

    const buildingLayer = new Layer(
        "building",
        [
            new Mask(
                "default_building",
                new Rect(new Coordinates(1,6), new Shape(4,6)).asBlock(),
                new Effect(undefined, library.getId("building_1"), undefined, null),
            ),
        ],
        5
    )

    const actionLayer = new Layer(
        "action",
        [
            new Mask(
                "logit",
                new Rect(new Coordinates(10,0), new Shape(2,1)).asMany(),
                new Effect(library.getId("logit"), library.getId("floor_2"), undefined, null),
            )
        ],
        6
    )

    const map = new Map(
        "test_map",
        [
            groundLayer,
            buildingLayer,
            actionLayer
        ],
        new Coordinates(0,0)
    )

    const pawn = new Pawn(
        new Coordinates(0, 0),
        library.getId("character_1") || NaN,
    );

    const scene = new Scene(
        "default",
        map,
        pawn
    )

    const engine = new Engine(scene)

    return engine
}