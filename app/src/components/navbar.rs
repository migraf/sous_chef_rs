use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            class: "flex flex-row",
            Link {
                class: "mr-5 no-underline transition-color duration-200 ease-in-out hover:cursor-pointer hover:text-slate-400",
                to: Route::Home {},
                "Home"
            }
            Link {
                class: "mr-5 no-underline transition-color duration-200 ease-in-out hover:cursor-pointer hover:text-slate-400",
                to: Route::Blog { id: 1 },
                "Blog"
            }
        }

        Outlet::<Route> {}
    }
}
