use dioxus::prelude::*;

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        div {
            class: "w-sm mx-auto mt-12 p-5 bg-zinc-800 rounded-lg",
            h4 {
                class: "m-0 mr-4",
                "ServerFn Echo"
            }
            input {
                class: "block w-full p-0 pr-1 border-b bg-transparent transition-property-[border-bottom-color] duration-200 ease-in outline-none focus:border-b-slate-400",
                placeholder: "Type here to echo...",
                oninput:  move |event| async move {
                    let data = echo_server(event.value()).await.unwrap();
                    response.set(data);
                },
            }

            if !response().is_empty() {
                p {
                    class: "m-0 mt-5 mr-auto",
                    "Server echoed: "
                    i { "{response}" }
                }
            }
        }
    }
}

/// Echo the user input on the server.
#[server(EchoServer)]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
