//! Request structs from the client.

pub mod announcements;
pub mod base;
pub mod blacklist;
pub mod clans;
pub mod invites;
pub mod members;

#[cfg(feature = "admin")]
pub mod admin;
