//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to define common UI elements like buttons, cards, forms, and modals.

pub mod breadcrumb;
pub mod calendar;
pub mod category_card;
pub mod empty_state;
pub mod event_card;
pub mod filter_section;
pub mod page_header;
pub mod paginated_listing;
pub mod pagination;
pub mod ui;

pub use crate::breadcrumb_items;
pub use breadcrumb::Breadcrumb;
pub use calendar::Calendar;
pub use category_card::CategoryCard;
pub use empty_state::EmptyState;
pub use event_card::EventCard;
pub use filter_section::{FilterConfig, FilterSection};
pub use page_header::PageHeader;
pub use paginated_listing::PaginatedListing;
pub use pagination::Pagination;
