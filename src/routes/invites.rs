//! TODO: document this

use actix_web::{post, web::Data};
use mongodb::bson::doc;

use crate::{
    database::Database,
    structs::{
        entities::{clan::Clan, player::{Jid, Player, Role, Status}},
        requests::{
            base::Request,
            invites::{CancelInvitation, SendInvitation},
        },
        responses::{base::{Content, Response}, error::ErrorCode},
    },
};

/// Invite a player to a clan.
#[post("/clan_manager_update/sec/send_invitation")]
pub async fn send_invitation(
    database: Data<Database>,
    req: Request<SendInvitation>,
    mut clan: Clan,
    user: Jid
) -> Response<()> {
    // Check if the user has been blacklisted
    if clan.is_blacklisted(&user) {
        return Response::error(ErrorCode::Blacklisted);
    }

    // Check if the player is already a member
    if clan.is_member(&req.request.jid) {
        return Response::error(ErrorCode::MemberStatusInvalid);
    }

    // Invite the player
    let player = Player {
        jid: req.request.jid,
        role: Role::NonMember,
        status: Status::Invited,
        ..Default::default()
    };

    clan.members.push(player);

    // Update the clan
    if database
        .clans
        .replace_one(doc! { "id": clan.id() }, clan)
        .await
        .is_err()
    {
        return Response::error(ErrorCode::InternalServerError);
    }

    Response::success(Content::Empty)
}

/// Cancel an invitation to a player.
#[post("/clan_manager_update/sec/cancel_invitation")]
pub async fn cancel_invitation(
    database: Data<Database>,
    req: Request<CancelInvitation>,
    mut clan: Clan
) -> Response<()> {
    // Check if there actually is an invitation
    let member = clan.members.iter().find(|m| m.jid == req.request.jid);
    if member.is_none() {
        return Response::error(ErrorCode::NoSuchClanMember);
    }

    if member.unwrap().status != Status::Invited {
        return Response::error(ErrorCode::MemberStatusInvalid);
    }

    // Cancel the invitation
    clan.members.retain(|p| p.jid != req.request.jid);

    // Update the clan
    if let Err(e) = clan.save(&database).await { return Response::error(e); }

    Response::success(Content::Empty)
}

/// Request to join a clan.
#[post("/clan_manager_update/sec/request_membership")]
pub async fn request_membership(
    database: Data<Database>,
    mut clan: Clan,
    user: Jid
) -> Response<()> {
    // Check if the user has already been invited or is already pending approval
    let member = clan.members.iter().find(|m| m.jid == user);
    if let Some(member) = member {
        if member.status == Status::Invited || member.status == Status::Pending {
            return Response::error(ErrorCode::BadRequest);
        }
    }

    // Check if the user has been blacklisted
    if clan.is_blacklisted(&user) {
        return Response::error(ErrorCode::Blacklisted);
    }

    // Request membership
    let player = Player {
        jid: user,
        role: Role::NonMember,
        status: Status::Pending,
        ..Default::default()
    };

    clan.members.push(player);

    // Update the clan
    if let Err(e) = clan.save(&database).await { return Response::error(e); }

    Response::success(Content::Empty)
}
