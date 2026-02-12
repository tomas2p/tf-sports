use dioxus::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum ButtonVariant {
    Default,
    Outline,
    Ghost,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ButtonSize {
    Default,
    Sm,
    Lg,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ButtonShape {
    Default,
    // Circular,
}

#[component]
pub fn Button(
    children: Element,
    #[props(default=None)] is_icon: Option<bool>,
    #[props(default=None)] aria_label: Option<String>,
    #[props(default = ButtonVariant::Default)] variant: ButtonVariant,
    #[props(default = ButtonSize::Default)] size: ButtonSize,
    #[props(default = ButtonShape::Default)] shape: ButtonShape,
    #[props(default = "".to_string())] class: String,
    #[props(default = false)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let base_classes = "inline-flex items-center justify-center font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 dark:focus-visible:ring-zinc-300 disabled:opacity-50 disabled:pointer-events-none";

    let variant_classes = match variant {
        ButtonVariant::Default => "bg-zinc-900 dark:bg-zinc-50 text-white dark:text-zinc-900 hover:bg-zinc-800 dark:hover:bg-zinc-200",
        ButtonVariant::Outline => "border border-zinc-300 dark:border-zinc-700 bg-white dark:bg-zinc-950 hover:bg-zinc-50 dark:hover:bg-zinc-900 text-zinc-900 dark:text-zinc-50",
        ButtonVariant::Ghost => "hover:bg-zinc-100 dark:hover:bg-zinc-800 text-zinc-900 dark:text-zinc-50",
    };

    let size_classes = match (size, is_icon) {
        (ButtonSize::Sm, Some(true)) => "h-8 w-8 text-xs",
        (ButtonSize::Sm, _) => "h-8 px-3 text-xs",
        (ButtonSize::Default, Some(true)) => "h-10 w-10 text-sm",
        (ButtonSize::Default, _) => "h-10 px-4 py-2 text-sm",
        (ButtonSize::Lg, Some(true)) => "h-12 w-12 text-base",
        (ButtonSize::Lg, _) => "h-12 px-6 text-base",
    };

    let shape_classes = match shape {
        ButtonShape::Default => "rounded-md",
        // ButtonShape::Circular => "rounded-full",
    };

    rsx! {
        button {
            class: "{base_classes} {variant_classes} {size_classes} {shape_classes} {class}",
            aria_label: aria_label.clone().unwrap_or_default(),
            disabled,
            onclick: move |evt| {
                if let Some(handler) = &onclick {
                    handler.call(evt);
                }
            },
            {children}
        }
    }
}
