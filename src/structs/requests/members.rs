//! Request structs for member related requests.

use serde::Deserialize;

use crate::structs::{entities::clan::Id, ticket::Ticket};

/// Request to get a list of members.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct GetMemberList {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: Id,

    /// How many members to skip.
    pub start: i32,

    /// How many members to return.
    pub max: i32,
}

/// Request to get info about a member.
#[derive(Debug, Deserialize)]
pub struct GetMemberInfo {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: Id,

    /// The JID of the player.
    pub jid: String,
}

/// Request to kick a member from a clan.
#[derive(Debug, Deserialize)]
pub struct KickMember {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: Id,

    /// The JID of the player to kick.
    pub jid: String,
}

/// Request to change a member's role.
#[derive(Debug, Deserialize)]
pub struct ChangeMemberRole {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: Id,

    /// The JID of the player to change the role of.
    pub jid: String,

    /// The new role for the player.
    pub role: u32,
}

/// Request to update a member's information.
#[derive(Debug, Deserialize)]
pub struct UpdateMemberInfo {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: Id,

    /// The JID of the player to update.
    pub jid: String,

    /// The new display name for the player.
    pub onlinename: String,

    /// The new description for the player.
    pub description: String,
    
    /// Whether the player allows system messages.
    pub allowmsg: bool,

    /// Unknown use.
    pub bin_attr1: String,

    /// Unknown use.
    pub size: u32,
}

/// Request to join a clan.
#[derive(Debug, Deserialize)]
pub struct JoinClan {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: Id,
}

/// Request to leave a clan.
#[derive(Debug, Deserialize)]
pub struct LeaveClan {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: Id,
}