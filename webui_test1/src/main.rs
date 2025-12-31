use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/utilities")]
    Utilities {},
    #[route("/utility/:name")]
    Utility { name: String },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.7/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                    "💫 VSCode Extension"
                }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}

    }
}

/// Utilities list page
#[component]
fn Utilities() -> Element {
    let utility_list = vec![
        "Calculator",
        "Color Picker",
        "Text Converter",
        "QR Code Generator",
        "Unit Converter",
    ];

    rsx! {
        div { id: "utilities",
            h1 { "Utilities" }
            p { "Select a utility from the list below:" }

            div { id: "links",
                for utility_name in utility_list {
                    Link {
                        to: Route::Utility {
                            name: utility_name.to_lowercase().replace(" ", "-"),
                        },
                        "{utility_name}"
                    }
                }
            }
        }
    }
}

/// Individual utility page
#[component]
pub fn Utility(name: String) -> Element {
    let display_name = name.replace("-", " ")
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    rsx! {
        div { id: "utility",
            h1 { "{display_name}" }
            p { "This is the {display_name} utility page." }

            div { class: "utility-content",
                p { "Utility functionality will be implemented here." }
            }

            Link { to: Route::Utilities {}, "← Back to Utilities" }
        }
    }
}

/// Shared navbar component with proper semantic structure
#[component]
fn Navbar() -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column; min-height: 100vh;",
            header {
                nav { id: "navbar",
                    Link { to: Route::Home {}, "Home" }
                    Link { to: Route::Utilities {}, "Utilities" }
                }
            }

            main { style: "flex: 1;", Outlet::<Route> {} }

            Footer {}
        }
    }
}

/// Footer component
#[component]
fn Footer() -> Element {
    rsx! {
        footer { id: "footer",
            p { "Built with Dioxus © 2025" }
        }
    }
}
