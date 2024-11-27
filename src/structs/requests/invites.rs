//! Request structs for invite related requests.

use serde::Deserialize;

use crate::structs::{entities::{clan::Id, player::Jid}, ticket::Ticket};

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
pub struct CancelInvitation {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: Id,

    /// The JID of the player to cancel the invitation for.
    pub jid: Jid,
}

/// Request to accept an invitation to join a clan.
#[derive(Debug, Deserialize)]
pub struct AcceptInvitation {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: Id,
}

/// Request to decline an invitation to join a clan.
#[derive(Debug, Deserialize)]
pub struct DeclineInvitation {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: Id,
}

/// Request to join a clan.
#[derive(Debug, Deserialize)]
pub struct RequestMembership {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: Id,
}

/// Cancel a request to join a clan.
#[derive(Debug, Deserialize)]
pub struct CancelRequestMembership {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: Id,
}

/// Request to accept a player's request to join a clan.
#[derive(Debug, Deserialize)]
pub struct AcceptMembershipRequest {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: Id,

    /// The JID of the player to approve.
    pub jid: Jid,
}

/// Request to decline a player's request to join a clan.
#[derive(Debug, Deserialize)]
pub struct DeclineMembershipRequest {
    /// A PSN ticket for authenticating the request.
    pub ticket: Ticket,

    /// The ID of the clan.
    pub id: Id,

    /// The JID of the player to decline.
    pub jid: Jid,
}