//! Represents an announcement posted to a clan.

use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::structs::requests::announcements::PostAnnouncement;

use super::player::Jid;

/// Maximum number of announcements that can exist in the game.
const MAX_ANNOUNCEMENT_COUNT: u32 = 1_000_000;

/// An announcement ID.
/// 
/// Should be limited to [`MAX_ANNOUNCEMENT_COUNT`], as the game
/// rejects any ID that surpasses it.
pub type Id = u32;

/// Represents an announcement posted to a clan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcement {
    /// The ID of the announcement.
    id: Id,

    /// The title of the announcement.
    pub subject: String,

    /// The body of the announcement.
    pub msg: String,

    /// The player who posted the announcement.
    pub author: Jid,

    /// The date the announcement was posted.
    #[serde(with = "chrono::serde::ts_seconds")]
    pub date_created: DateTime<Utc>,

    /// The date the announcement should expire.
    #[serde(with = "chrono::serde::ts_seconds")]
    pub date_expire: DateTime<Utc>,

    /// Unknown use.
    pub bin_data: String,

    /// Unknown use.
    pub from_id: Id,
}

impl Default for Announcement {
    fn default() -> Self {
        let range = 1..MAX_ANNOUNCEMENT_COUNT;
        let rng = &mut rand::thread_rng();

        Self {
            id: rng.gen_range(range),
            subject: String::new(),
            msg: String::new(),
            author: Jid::default(),
            date_created: Utc::now(),
            date_expire: Utc::now(),
            bin_data: String::new(),
            from_id: 1 // 0 would break the game
        }
    }
}

#[allow(clippy::cast_possible_wrap)]
impl From<PostAnnouncement> for Announcement {
    fn from(request: PostAnnouncement) -> Self {
        Self {
            subject: request.subject,
            msg: request.msg,
            date_expire: Utc::now() + chrono::Duration::seconds(request.expire_date as i64),
            ..Default::default()
        }
    }
}

impl Announcement {
    /// Returns the ID of the announcement.
    pub const fn id(&self) -> Id {
        self.id
    }
}