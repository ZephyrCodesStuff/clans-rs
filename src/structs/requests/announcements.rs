//! Request structs for announcements related requests.

use serde::Deserialize;

use crate::structs::{entities::{announcement::Id as AnnouncementId, clan::Id as ClanId}, ticket::Ticket};

/// Request to get a clan's announcements.
#[derive(Debug, Deserialize)]
#[allow(clippy::module_name_repetitions)]
pub struct RetrieveAnnouncements {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: ClanId,

    /// How many announcements to skip.
    pub start: i32,

    /// How many announcements to return.
    pub max: i32,
}

/// Request to post a new announcement for a clan.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PostAnnouncement {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: ClanId,

    /// The title of the announcement.
    pub subject: String,

    /// The body of the announcement.
    pub msg: String,

    /// The date the announcement will expire, expressed in
    /// seconds into the future, starting from right now.
    pub expire_date: u64,
}

/// Request to delete an announcement.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DeleteAnnouncement {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: ClanId,

    /// The ID of the announcement to delete.
    pub msg_id: AnnouncementId
}