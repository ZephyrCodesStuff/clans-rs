//! Player entity module.

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::ticket::Ticket;

/// A JID is an identifier composed of:
/// 
/// - The player's username.
/// - The region's ``PlayStation Network`` domain (a0, a1, ...)
/// - The player's account region.
/// - ``PlayStation Network``'s domain.
/// 
/// Example: ``username@a1.us.np.playstation.net``
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Jid {
    /// The player's username.
    pub username: String,

    /// The region's ``PlayStation Network`` domain.
    pub domain: String,

    /// The player's account region.
    pub region: String,
}

impl From<String> for Jid {
    fn from(jid: String) -> Self {
        let mut parts = jid.split('@');

        let username = parts.next().unwrap_or_default().to_string();
        let psn = parts.next().unwrap_or_default();

        let mut parts = psn.split('.');

        let domain = parts.next().unwrap_or_default().to_string();
        let region = parts.next().unwrap_or_default().to_string();

        Self { username, domain, region }
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
        Ok(Self::from(jid))
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
}

impl Default for Player {
    fn default() -> Self {
        Self {
            jid: Jid::default(),
            role: Role::Unknown,
            status: Status::Unknown,
            description: String::new(),
            allow_msg: false,
        }
    }
}