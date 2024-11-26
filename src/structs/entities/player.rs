//! Player entity module.

use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// A JID is a string composed of a ``username`` and the
/// ``PlayStation`` region server the account is based on.
/// 
/// Example: ``username@a1.us.np.playstation.net``
pub type Jid = String;

/// A player's role in the clan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

impl Player {
    /// The player's username.
    pub fn username(&self) -> &str {
        self.jid.split('@').next().unwrap_or_default()
    }
}