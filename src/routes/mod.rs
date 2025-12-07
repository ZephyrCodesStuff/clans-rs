//! Route handlers for API endpoints.

pub mod announcements;
pub mod blacklist;
pub mod clans;
pub mod invites;
pub mod members;

#[cfg(feature = "admin")]
pub mod admin;
