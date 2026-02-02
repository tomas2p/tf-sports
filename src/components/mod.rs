//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to define common UI elements like buttons, cards, forms, and modals.

pub mod ui;
pub mod event_card;
pub mod event_card_with_image;
pub mod page_header;
pub mod filter_section;
pub mod empty_state;
pub mod breadcrumb;
pub mod pagination;
pub mod calendar;

pub use event_card::EventCard;
pub use event_card_with_image::EventCardWithImage;
pub use page_header::PageHeader;
pub use filter_section::{FilterSection, FilterConfig};
pub use empty_state::EmptyState;
pub use breadcrumb::{Breadcrumb, BreadcrumbItem};
pub use pagination::Pagination;
pub use calendar::Calendar;