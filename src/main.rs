// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use views::{Details, Events, Home, Navbar, Sport, Sports};

/// Define a components module that contains all shared components for our app.
mod components;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;
/// Define a models module that contains data structures.
mod models;
/// Define a theme module that contains theme management.
mod theme;

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
        // Dynamic route for event/sport details
        #[route("/details/:id")]
        Details { id: i32 },
        // Events listing page
        #[route("/events")]
        Events {},
        // Sports overview page
        #[route("/sports")]
        Sports {},
        // Dynamic route for sport categories
        #[route("/sport/:category")]
        Sport { category: String },
}

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundles smaller
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const EVENTOS_JSON: Asset = asset!("/data/agenda-de-eventos-deportivos-en-tenerife.json");

fn main() {
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
    
    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
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
