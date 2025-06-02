use dioxus::prelude::*;
use rpgx::library::Library;
use std::any::Any;

pub fn use_library() -> Library<Box<dyn Any>> {
    let mut library: Library<Box<dyn Any>> = Library::new();

    // Platform-agnostic logger
    fn log_message(message: &str) {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&message.into());

        #[cfg(not(target_arch = "wasm32"))]
        println!("{message}");
    }

    library.insert(
        "floor_1",
        Box::new("https://s3.rottigni.tech/rpgx/spaceship_floor_1.webp".to_string()),
    );
    library.insert(
        "floor_2",
        Box::new("https://s3.rottigni.tech/rpgx/spaceship_floor_2.webp".to_string()),
    );
    library.insert(
        "floor_3",
        Box::new("https://s3.rottigni.tech/rpgx/spaceship_floor_3.webp".to_string()),
    );
    library.insert(
        "building_1",
        Box::new("https://s3.rottigni.tech/rpgx/processor_8.webp".to_string()),
    );
    library.insert(
        "building_2",
        Box::new("https://s3.rottigni.tech/rpgx/processor_9.webp".to_string()),
    );
    library.insert(
        "portal_1",
        Box::new("https://s3.rottigni.tech/rpgx/portal_1.webp".to_string()),
    );
    library.insert(
        "character_1",
        Box::new("https://s3.rottigni.tech/rpgx/character_1.webp".to_string()),
    );
    // Platform-agnostic action
    library.insert(
        "consolelog",
        Box::new(|| {
            log_message("Hello from Rust!");
        }),
    );

    library.insert(
        "sign",
        Box::new(Box::new(|| {
            println!("Invoked render closure for sign");
            rsx! {
                div {
                    class: "sign",
                    style: "width: 100%; height: 100%; background-color: red;",
                    "this is sign"
                }
            }
            .unwrap()
        }) as Box<dyn Fn() -> VNode>) as Box<dyn Any>,
    );

    library
}
