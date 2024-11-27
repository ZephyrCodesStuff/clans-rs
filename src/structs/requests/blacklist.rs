//! Request structs for blacklist related requests.

use serde::Deserialize;

use crate::structs::{entities::clan::Id, ticket::Ticket};

/// Request to get a clan's blacklist.
#[derive(Debug, Deserialize)]
#[allow(clippy::module_name_repetitions)]
#[allow(dead_code)]
pub struct GetBlacklist {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: Id,

    /// How many members to skip.
    pub start: i32,

    /// How many members to return.
    pub max: i32,
}

/// Request to add a player to a clan's blacklist.
#[derive(Debug, Deserialize)]
pub struct RecordBlacklistEntry {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: Id,

    /// The JID of the player to add to the blacklist.
    pub jid: String,
}

/// Request to remove a player from a clan's blacklist.
#[derive(Debug, Deserialize)]
pub struct DeleteBlacklistEntry {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: Id,

    /// The JID of the player to remove from the blacklist.
    pub jid: String,
}