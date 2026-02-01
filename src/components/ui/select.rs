use dioxus::prelude::*;

#[component]
pub fn Select(
    children: Element,
    #[props(default = "".to_string())] class: String,
    value: String,
    onchange: EventHandler<String>,
) -> Element {
    rsx! {
        select {
            class: "h-10 w-full rounded-md border border-zinc-300 dark:border-zinc-700 bg-white dark:bg-zinc-950 text-zinc-950 dark:text-zinc-50 px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-zinc-950 dark:focus:ring-zinc-300 focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 cursor-pointer {class}",
            value: "{value}",
            onchange: move |evt| onchange.call(evt.value()),
            {children}
        }
    }
}
