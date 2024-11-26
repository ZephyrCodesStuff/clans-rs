//! These structs represent the data of a clan in the game,
//! as completely as possible.
//! 
//! They are what's stored into the database.

use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::player::{Jid, Player, Role, Status};

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
    /// Unique identifier of the clan. Should not be changed manually,
    /// because values outside the range of [`MAX_CLAN_COUNT`] **will**
    /// halt the game.
    /// 
    /// See: [`Id`](type.Id.html)
    id: Id,
    
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

    /// Unknown use.
    pub size: u32,
}

impl Default for Clan {
    fn default() -> Self {
        let range = 1..MAX_CLAN_COUNT;
        let rng = &mut rand::thread_rng();

        Self {
            id: rng.gen_range(range),
            name: String::new(),
            tag: String::new(),
            description: String::new(),
            members: Vec::new(),
            blacklist: Vec::new(),
            date_created: Utc::now(),
            last_updated: Utc::now(),
            auto_accept: false,
            int_attr1: 0,
            int_attr2: 0,
            int_attr3: 0,
            size: 0,
        }
    }
}

impl Clan {
    /// Returns the clan's ID.
    pub const fn id(&self) -> Id {
        self.id
    }

    /// Returns the role of the given player, in the clan.
    pub fn role_of(&self, jid: &Jid) -> Option<&Role> {
        self.members.iter()
            .find(|player| player.jid.username == jid.username)
            .map(|player| &player.role)
    }

    /// Returns the status of the given player, in the clan.
    pub fn status_of(&self, jid: &Jid) -> Option<&Status> {
        self.members.iter()
            .find(|player| player.jid.username == jid.username)
            .map(|player| &player.status)
    }

    /// Returns whether a player is allowed to perform administrative actions.
    pub fn is_mod(&self, jid: &Jid) -> bool {
        self.members.iter()
            .any(|player| player.jid.username == jid.username && player.role >= Role::SubLeader)
    }

    /// Returns whether a player is the owner of the clan.
    pub fn is_owner(&self, jid: &Jid) -> bool {
        self.members.iter()
            .any(|player| player.jid.username == jid.username && player.role == Role::Leader)
    }
}