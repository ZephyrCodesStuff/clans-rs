//! Request structs for blacklist related requests.

use serde::Deserialize;

/// Request to get a clan's blacklist.
#[derive(Debug, Deserialize)]
#[allow(clippy::module_name_repetitions)]
#[allow(dead_code)]
pub struct GetBlacklist {
    /// How many members to skip.
    pub start: u32,

    /// How many members to return.
    pub max: u32,
}

/// Request to add a player to a clan's blacklist.
#[derive(Debug, Deserialize)]
pub struct RecordBlacklistEntry {
    /// The JID of the player to add to the blacklist.
    pub jid: String,
}

/// Request to remove a player from a clan's blacklist.
#[derive(Debug, Deserialize)]
pub struct DeleteBlacklistEntry {
    /// The JID of the player to remove from the blacklist.
    pub jid: String,
}