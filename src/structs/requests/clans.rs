//! Request structs for clan related requests.

use serde::Deserialize;

use crate::structs::entities::{clan::{Clan, Id}, player::{Jid, Player, Role, Status}, ticket::Ticket};

/// Request to create a clan.
#[derive(Debug, Deserialize)]
pub struct CreateClan {
    /// A PSN ticket for authenticating the request
    pub ticket: Ticket,

    /// The name of the clan.
    pub name: String,

    /// The clan's tag.
    pub tag: String,
}

impl From<CreateClan> for Clan {
    fn from(request: CreateClan) -> Self {
        let mut clan = Self::default();

        clan.name = request.name;
        clan.tag = request.tag;

        clan.members = vec![
            Player {
                jid: Jid::from(request.ticket),
                role: Role::Leader,
                status: Status::Member,
                ..Default::default()
            }
        ];

        clan
    }
}

/// Request to get a list of clans.
#[derive(Debug, Deserialize)]
pub struct GetClanList {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// How many clans to skip.
    pub start: u32,

    /// How many clans to return.
    pub max: u32,
}

/// Request to get a list of clans.
#[derive(Debug, Deserialize)]
pub struct ClanSearch {
    /// How many clans to skip.
    pub start: u32,

    /// How many clans to return.
    pub max: u32,
}

/// Request to get info about a clan.
#[derive(Debug, Deserialize)]
pub struct GetClanInfo {
    /// The ID of the clan.
    pub id: Id,
}

/// Request to update a clan's info.
#[derive(Debug, Deserialize)]
pub struct UpdateClanInfo {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: Id,

    /// The new description of the clan.
    pub description: String,
}

/// Request to disband a clan.
#[derive(Debug, Deserialize)]
pub struct DisbandClan {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: Id,
}