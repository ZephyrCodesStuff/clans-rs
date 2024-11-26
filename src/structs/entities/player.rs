//! Player entity module.

use std::fmt::Display;

use crate::utils::xml_format::ToXml;

/// A JID is a string composed of a ``username`` and the
/// ``PlayStation`` region server the account is based on.
/// 
/// Example: ``username@a1.us.np.playstation.net``
pub type Jid = String;

/// Represents the basic information for a player,
/// needed to display it in a search result.
pub struct BasicInfo {
    /// The player's JID.
    pub jid: Jid,
}

impl ToXml for BasicInfo {
    fn to_xml(&self, name: Option<&str>) -> String {
        format!(
            r#"<{}><jid>{}</jid></{}>"#,
            name.unwrap_or("entry"), self.jid, name.unwrap_or("entry")
        )
    }
}

/// A player's role in the clan.
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
#[derive(Debug)]
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
}

impl ToXml for Player {
    fn to_xml(&self, name: Option<&str>) -> String {
        format!(
            r#"<{} jid="{}"><role>{}</role><status>{}</status><description>{}</description></{}>"#,
            name.unwrap_or("player"), self.jid, self.role, self.status, self.description, name.unwrap_or("player")
        )
    }
}