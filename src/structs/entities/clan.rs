//! These structs represent the data of a clan in the game,
//! as completely as possible.
//! 
//! They are what's stored into the database.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::player::{Jid, Player};

/// Maximum number of clans that can exist in the game.
const MAX_CLAN_COUNT: u32 = 1_000_000;

/// A clan ID.
/// 
/// Should be limited to [`MAX_CLAN_COUNT`], as the game
/// rejects any ID that surpasses it.
pub type Id = u32;

/// Represents a clan in the game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clan {
    /// Unique identifier of the clan.
    /// 
    /// See: [`Id`](type.Id.html)
    pub id: Id,
    
    /// Displayed name of the clan.
    /// 
    /// This appears below the player's name.
    pub name: String,

    /// Tag used to identify the clan.
    /// 
    /// NOTE: This is **not** what appears below the player's name.
    pub tag: String,

    /// Description of the clan.
    /// 
    /// This appears when the player clicks on the clan,
    /// and selects to view the clan's information.
    pub description: String,

    /// Members currenty in the clan.
    /// 
    /// This will always include, at least, the clan's leader.
    pub members: Vec<Player>,

    /// Players that are banned from joining the clan.
    pub blacklist: Vec<Jid>,

    /// Creation date of the clan, in UTC.
    #[serde(with = "chrono::serde::ts_seconds")]
    pub date_created: DateTime<Utc>,

    /// Last time the clan was updated, in UTC.
    /// 
    /// NOTE: This isn't needed by the game, but it's useful for statistics.
    #[serde(with = "chrono::serde::ts_seconds")]
    pub last_updated: DateTime<Utc>,
    
    /// If this flag is `true`, then the clan should
    /// automatically accept any player that requests to join.
    pub auto_accept: bool,

    /// Unknown use.
    pub int_attr1: u32,

    /// Unknown use.
    pub int_attr2: u32,

    /// Unknown use.
    pub int_attr3: u32,
}