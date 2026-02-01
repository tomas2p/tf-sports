//! The views module contains the components for all Layouts and Routes for our app. Each layout and route in our [`Route`]
//! enum will render one of these components.
//!
//! The [`Home`], [`Details`], [`Events`], and [`Sport`] components will be rendered when the current route matches.
//!
//! The [`Navbar`] component will be rendered on all pages of our app since every page is under the layout. The layout defines
//! a common wrapper around all child routes.

mod home;
pub use home::Home;

mod details;
pub use details::Details;

mod events;
pub use events::Events;

mod sport;
pub use sport::Sport;

mod navbar;
pub use navbar::Navbar;
