use dioxus::prelude::*;

#[component]
pub fn SearchBar(mut value: Signal<String>, placeholder: String) -> Element {
    rsx! {
            input {
                r#type: "text",
                placeholder: "{placeholder}",
                value: value,
                class: "w-full px-4 py-2 rounded-lg border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-950 text-zinc-950 dark:text-zinc-50 placeholder:text-zinc-500 focus:outline-none focus:ring-2 focus:ring-zinc-950 dark:focus:ring-zinc-300",
                oninput: move |evt| value.set(evt.value()),
            }
    }
}
