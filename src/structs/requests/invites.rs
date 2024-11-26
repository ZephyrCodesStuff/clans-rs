//! Request structs for invite related requests.

use serde::Deserialize;

use crate::structs::entities::{clan::Id, player::Jid, ticket::Ticket};

/// Request to send an invitation to a player.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SendInvitation {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: Id,

    /// The JID of the player to invite.
    pub jid: Jid,
}

/// Request to cancel an invitation to a player.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CancelInvitation {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: Id,

    /// The JID of the player to cancel the invitation for.
    pub jid: Jid,
}