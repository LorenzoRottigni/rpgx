use rpgx::{
    library::ResourceLibrary,
    map::Map,
    prelude::{Coordinates, Shape},
};

pub fn use_map1(library: ResourceLibrary) -> Map {
    let mut single_map = rpgx::factory::map::presets::building::building_2x3(
        Shape {
            width: 4,
            height: 6,
        },
        library.get_key_id("building_1"),
        library.get_key_id("consolelog"),
    );
    single_map.load_layer(rpgx::factory::layer::presets::ground::ground_layer(
        Shape {
            width: 6,
            height: 8,
        },
        library.get_key_id("floor_1"),
    ));
    single_map.load_layer(rpgx::factory::layer::presets::street::street_layer_around(
        Shape {
            width: 4,
            height: 6,
        },
        library.get_key_id("floor_2"),
    ));
    single_map.load_layer(rpgx::factory::layer::presets::street::street_layer_around(
        Shape {
            width: 6,
            height: 8,
        },
        library.get_key_id("floor_2"),
    ));

    let mut map = single_map.clone();
    map.merge_at(&single_map.clone(), Coordinates { x: 8, y: 0 });
    map.merge_at(&single_map.clone(), Coordinates { x: 0, y: 10 });
    map.merge_at(&single_map.clone(), Coordinates { x: 8, y: 10 });

    let portal = rpgx::factory::map::presets::building::building_2x3(
        Shape {
            width: 4,
            height: 6,
        },
        library.get_key_id("portal_1"),
        library.get_key_id("consolelog"),
    );
    map.merge_at(&portal, Coordinates { x: 6, y: 0 });
    map.merge_at(&map.clone(), Coordinates { x: 16, y: 0 });
    map.merge_at(&map.clone(), Coordinates { x: 8, y: 20 });
    map.merge_at(&map.clone(), Coordinates { x: 32, y: 0 });

    map
}
