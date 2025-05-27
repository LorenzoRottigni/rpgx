use rpgx::library::ResourceLibrary;

pub fn use_library() -> ResourceLibrary {
    let mut w_library = ResourceLibrary::new();

    // Platform-agnostic logger
    fn log_message(message: &str) {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&message.into());

        #[cfg(not(target_arch = "wasm32"))]
        println!("{message}");
    }

    w_library.insert_texture(
        "floor_1",
        "https://s3.rottigni.tech/rpgx/spaceship_floor_1.webp".to_string(),
    );
    w_library.insert_texture(
        "floor_2",
        "https://s3.rottigni.tech/rpgx/spaceship_floor_2.webp".to_string(),
    );
    w_library.insert_texture(
        "floor_3",
        "https://s3.rottigni.tech/rpgx/spaceship_floor_3.webp".to_string(),
    );
    w_library.insert_texture(
        "building_1",
        "https://s3.rottigni.tech/rpgx/processor_8.webp".to_string(),
    );
    w_library.insert_texture(
        "building_2",
        "https://s3.rottigni.tech/rpgx/processor_9.webp".to_string(),
    );
    w_library.insert_texture(
        "portal_1",
        "https://s3.rottigni.tech/rpgx/portal_1.webp".to_string(),
    );
    w_library.insert_texture(
        "character_1",
        "https://s3.rottigni.tech/rpgx/character_1.webp".to_string(),
    );
    // Platform-agnostic action
    w_library.insert_action("consolelog", || {
        log_message("Hello from Rust!");
    });
    w_library.insert_action("teleport", || {});

    w_library
}
