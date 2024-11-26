//! Request structs for invite related requests.

use serde::Deserialize;

use crate::structs::entities::player::Jid;

/// Request to send an invitation to a player.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SendInvitation {
    /// The JID of the player to invite.
    pub jid: Jid,
}

/// Request to cancel an invitation to a player.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CancelInvitation {
    /// The JID of the player to cancel the invitation for.
    pub jid: Jid,
}