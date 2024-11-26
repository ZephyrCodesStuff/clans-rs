//! Request structs for member related requests.

use serde::Deserialize;

use crate::structs::entities::{clan::Id, ticket::Ticket};

/// Request to get a list of members.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct GetMemberList {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub clan_id: Id,

    /// How many members to skip.
    pub start: u32,

    /// How many members to return.
    pub max: u32,
}