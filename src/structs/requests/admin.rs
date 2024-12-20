//! Request structs for the Admin endpoints
use serde::Deserialize;

use crate::structs::entities::{clan::{Clan, Platform}, player::{Jid, Player, Role, Status}};

/// Request to create a clan.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateClan {
    /// The username of the user creating the clan.
    pub username: String,

    /// The desired clan's name
    pub clan_name: String,

    /// The desired clan's tag
    pub clan_tag: String,

    /// The platform the clan is on
    pub clan_platform: Platform,
}

impl From<(CreateClan, Jid)> for Clan {
    fn from((request, author): (CreateClan, Jid)) -> Self {
        let mut clan = Self::default();

        clan.name = request.clan_name;
        clan.tag = request.clan_tag;
        clan.platform = request.clan_platform;

        clan.members = vec![
            Player {
                jid: author,
                role: Role::Leader,
                status: Status::Member,
                ..Default::default()
            }
        ];

        clan
    }
}