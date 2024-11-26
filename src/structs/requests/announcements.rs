//! Request structs for announcements related requests.

use serde::Deserialize;

use crate::structs::{entities::clan::Id, ticket::Ticket};

/// Request to post a new announcement for a clan.
#[derive(Debug, Deserialize)]
pub struct PostAnnouncement {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: Id,

    /// The title of the announcement.
    pub subject: String,

    /// The body of the announcement.
    pub msg: String,

    /// The date the announcement will expire, expressed in
    /// seconds into the future, starting from right now.
    pub expire_date: u64,

    // TODO: add `xmpp-msg` structs and fields
}