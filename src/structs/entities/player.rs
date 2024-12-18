//! Player entity module.

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{routes::admin::{FORGED_JID_DOMAIN, FORGED_JID_REGION}, structs::{requests::admin::CreateClan, ticket::Ticket}};

/// A JID is an identifier composed of:
/// 
/// - The player's username.
/// - The region's ``PlayStation Network`` domain (a0, a1, ...)
/// - The player's account region.
/// - ``PlayStation Network``'s domain.
/// 
/// Example: ``username@a1.us.np.playstation.net``
#[derive(Debug, Default, Clone)]
pub struct Jid {
    /// The player's username.
    pub username: String,

    /// The region's ``PlayStation Network`` domain.
    pub domain: String,

    /// The player's account region.
    pub region: String,
}

impl PartialEq for Jid {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username
    }
}

impl TryFrom<String> for Jid {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut parts = value.split('@');

        if parts.clone().count() != 2 { // username, a1.us.np.playstation.net
            return Err("Invalid JID format.");
        }

        let username = parts.next().unwrap_or_default().to_string();
        let psn = parts.next().unwrap_or_default();

        let mut parts = psn.split('.');

        if parts.clone().count() != 5 { // a1, us, np, playstation, net
            return Err("Invalid JID format.");
        }

        let domain = parts.next().unwrap_or_default().to_string();
        let region = parts.next().unwrap_or_default().to_string();

        Ok(Self { username, domain, region })
    }
}

impl From<CreateClan> for Jid {
    fn from(request: CreateClan) -> Self {
        Self {
            username: request.username,
            domain: FORGED_JID_DOMAIN.to_string(),
            region: FORGED_JID_REGION.to_string(),
        }
    }
}

impl Display for Jid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}.{}.np.playstation.net", self.username, self.domain, self.region)
    }
}

impl Serialize for Jid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Jid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let jid = String::deserialize(deserializer)?;
        Self::try_from(jid).map_err(serde::de::Error::custom)
    }
}

impl From<Ticket> for Jid {
    fn from(ticket: Ticket) -> Self {
        Self {
            username: ticket.username,
            region: ticket.region,
            domain: ticket.domain,
        }
    }
}

/// A player's role in the clan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Role {
    /// The player's role is unknown.
    /// 
    /// ⚠️ **WARNING**: This will hide the player from the clan's member list.
    Unknown = 0,

    /// The player is not a member of the clan.
    NonMember = 1,

    /// The player is a normal member of the clan.
    Member = 2,

    /// The player is a sub-leader of the clan.
    SubLeader = 3,

    /// The player is the leader of the clan.
    /// 
    /// NOTE: This will make them unable to leave the clan.
    Leader = 4,
}

impl From<u32> for Role {
    fn from(role: u32) -> Self {
        match role {
            1 => Self::NonMember,
            2 => Self::Member,
            3 => Self::SubLeader,
            4 => Self::Leader,
            _ => Self::Unknown,
        }
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "0"),
            Self::NonMember => write!(f, "1"),
            Self::Member => write!(f, "2"),
            Self::SubLeader => write!(f, "3"),
            Self::Leader => write!(f, "4"),
        }
    }
}

/// A player's status pertaining to the clan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Status {
    /// The player's status is unknown.
    /// 
    /// NOTE: This can be used in clan searches as a wildcard for any status.
    /// 
    /// ⚠️ **WARNING**: This will hide the player from the clan's member list.
    Unknown = 0,

    /// The player is a normal member of the clan.
    Member = 1,

    /// The player has been invited to join the clan.
    Invited = 2,

    /// The player has requested to join the clan and is waiting for approval.
    Pending = 3,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "0"),
            Self::Member => write!(f, "1"),
            Self::Invited => write!(f, "2"),
            Self::Pending => write!(f, "3"),
        }
    }
}

/// Represents a player in the game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    /// The player's JID.
    pub jid: Jid,

    /// The player's role in the clan.
    pub role: Role,

    /// The player's status pertaining to the clan.
    pub status: Status,

    /// The player's display name.
    pub online_name: String,

    /// The player's description.
    /// 
    /// Currently it is unknown where this is displayed.
    pub description: String,

    /// The ``allowMsg`` flag determines whether the member allows receiving
    /// messages viewable in the system software, whenever a post has been
    /// made to the clan's announcement board.
    /// 
    /// The default value for ``allowMsg`` is ``false``.
    pub allow_msg: bool,

    /// Unknown use.
    pub bin_data: String,

    /// Unknown use.
    pub size: u32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            jid: Jid::default(),
            role: Role::Unknown,
            status: Status::Unknown,
            online_name: String::new(),
            description: String::new(),
            allow_msg: false,
            bin_data: String::new(),
            size: 0,
        }
    }
}