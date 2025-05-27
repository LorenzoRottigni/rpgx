use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "home",
            a { class: "home-link", href: "/map1", "Map 1" }
            a { class: "home-link", href: "/map2", "Map 2" }
            a { class: "home-link", href: "/map3", "Map 3" }
        }
    }
}
