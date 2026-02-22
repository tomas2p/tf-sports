// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use views::{Event, Events, Home, Navbar, Place, Places, Sport, Sports};

/// Define a components module that contains all shared components for our app.
mod components;
/// Centralized data includes (JSON files)
mod data;
/// Define a models module that contains data structures.
mod models;
/// Define a styles module that contains CSS class constants.
mod styles;
/// Define a theme module that contains theme management.
mod theme;
/// Define a utils module that contains utility functions.
mod utils;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;

/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the router to work.
/// 
/// Each variant represents a different URL pattern that can be matched by the router. If that pattern is matched,
/// the components for that route will be rendered.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    // The layout attribute defines a wrapper for all routes under the layout. Layouts are great for wrapping
    // many routes with a common UI like a navbar.
    #[layout(Navbar)]
        // The route attribute defines the URL pattern that a specific route matches. If that pattern matches the URL,
        // the component for that route will be rendered. The component name that is rendered defaults to the variant name.
        #[route("/")]
        Home {},
        // Dynamic route for event/sport event
        #[route("/event/:id")]
        Event { id: i32 },
        // Events listing page
        #[route("/events")]
        Events {},
        // Sports overview page
        #[route("/sports")]
        Sports {},
        // Places page
        #[route("/places")]
        Places {},
        // Dynamic route for place details
        #[route("/place/:id")]
        Place { id: i64 },
        // Dynamic route for sport categories
        #[route("/sport/:category")]
        Sport { category: String },
}

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundles smaller
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // Install panic hooks to get readable stack traces in wasm/android environments
    #[cfg(all(target_arch = "wasm32", feature = "web"))]
    console_error_panic_hook::set_once();

    #[cfg(not(target_arch = "wasm32"))]
    std::panic::set_hook(Box::new(|info| {
        eprintln!("panic: {:?}", info);
    }));

    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    let mut theme = use_signal(|| {
        let t = theme::Theme::from_storage();
        t.apply();
        t
    });

    use_context_provider(|| theme);

    // En Android/desktop el tema no puede leerse síncronamente (web_sys no disponible),
    // así que lo leemos de forma asíncrona tras el montaje del componente.
    #[cfg(not(target_arch = "wasm32"))]
    {
        use crate::theme::Theme;
        use dioxus::prelude::document;
        use_effect(move || {
            let mut eval = document::eval("localStorage.getItem('theme') ?? ''");
            spawn(async move {
                if let Ok(stored) = eval.recv::<String>().await {
                    let t = match stored.trim() {
                        "dark" => Theme::Dark,
                        _ => Theme::Light,
                    };
                    t.apply();
                    *theme.write() = t;
                }
            });
        });
    }

    // Breadcrumb context: permite a páginas hijas (por ejemplo `Sport`) publicar
    // una ruta parcial que `Event` pueda consumir para renderizar el breadcrumb
    use crate::components::breadcrumb::BreadcrumbItem;
    let breadcrumb_ctx = use_signal(|| Option::<Vec<BreadcrumbItem>>::None);
    use_context_provider(|| breadcrumb_ctx);

    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
        // Ensure document language is set early for SEO / a11y
        document::Script {
            "(function() {{ try {{ document.documentElement.lang = 'es'; }} catch(e) {{}} }})();"
        }

        // Script para aplicar tema antes del render
        document::Script {
            "(function() {{ const theme = localStorage.getItem('theme'); console.log('Script inicial - tema:', theme); if (theme === 'dark') {{ console.log('Añadiendo clase dark'); document.documentElement.classList.add('dark'); }} }})();"
        }

        // In addition to element and text (which we will see later), rsx can contain other components. In this case,
        // we are using the `document::Link` component to add a link to our favicon and Tailwind CSS file into the head of our app.
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        // The router component renders the route enum we defined above. It will handle synchronization of the URL and render
        // the layouts and components for the active route.
        Router::<Route> {}
    }
}
