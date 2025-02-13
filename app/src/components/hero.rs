use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            class: "m-0 flex flex-col justify-center items-center",
            img { src: HEADER_SVG, id: "header" }
            div {
                class: "w-sm flex flex-col text-2xl text-left",
                for item in [
                    ("https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" ),
                    ("https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" ),
                    ("https://github.com/dioxus-community/", "📡 Community Libraries" ),
                    ("https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" ),
                    ("https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" ),
                    ("https://discord.gg/XgGxMSkvUM", "👋 Community Discord" ),
                ] {
                    a {
                        class: "mt-5 mb-2 mx-0 p-2 border rounded-sm no-underline hover:bg-neutral-800 hover:cursor-pointer",
                        href: item.0,
                        "{item.1}"
                    }
                }
            }
        }
    }
}
