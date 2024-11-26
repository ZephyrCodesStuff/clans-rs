//! TODO: document this

use actix_web::{post, web::Data};
use mongodb::bson::doc;

use crate::{database::Database, structs::{entities::player::{Jid, Player, Role, Status}, requests::{base::Request, invites::{CancelInvitation, RequestMembership, SendInvitation}}, responses::base::{Content, ErrorCode, Response}}};

/// Invite a player to a clan.
#[post("/clan_manager_update/sec/send_invitation")]
pub async fn send_invitation(database: Data<Database>, req: Request<SendInvitation>) -> Response<()> {
    let jid = Jid::from(req.request.ticket);

    // Find the clan
    let Ok(clan) = database.clans.find_one(doc! { "id": req.request.id }).await
    else { return Response::error(ErrorCode::SCE_NP_CLANS_SERVER_ERROR_INTERNAL_SERVER_ERROR) };

    if clan.is_none() {
        return Response::error(ErrorCode::SCE_NP_CLANS_SERVER_ERROR_NO_SUCH_CLAN);
    }

    let mut clan = clan.unwrap();

    // Check if the user has been blacklisted
    if clan.is_blacklisted(&jid) {
        return Response::error(ErrorCode::SCE_NP_CLANS_SERVER_ERROR_BLACKLISTED);
    }

    // Check if the player is already a member
    if clan.is_member(&req.request.jid) {
        return Response::error(ErrorCode::SCE_NP_CLANS_SERVER_ERROR_MEMBER_STATUS_INVALID);
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
    if database.clans.replace_one(doc! { "id": clan.id() }, clan).await.is_err() {
        return Response::error(ErrorCode::SCE_NP_CLANS_SERVER_ERROR_INTERNAL_SERVER_ERROR);
    }

    Response::success(Content::Empty)
}

/// Cancel an invitation to a player.
#[post("/clan_manager_update/sec/cancel_invitation")]
pub async fn cancel_invitation(database: Data<Database>, req: Request<CancelInvitation>) -> Response<()> {
    // Find the clan
    let Ok(clan) = database.clans.find_one(doc! { "id": req.request.id }).await
    else { return Response::error(ErrorCode::SCE_NP_CLANS_SERVER_ERROR_INTERNAL_SERVER_ERROR) };

    if clan.is_none() {
        return Response::error(ErrorCode::SCE_NP_CLANS_SERVER_ERROR_NO_SUCH_CLAN);
    }

    let mut clan = clan.unwrap();

    // Check if there actually is an invitation
    let member = clan.members.iter().find(|m| m.jid == req.request.jid);
    if member.is_none() {
        return Response::error(ErrorCode::SCE_NP_CLANS_SERVER_ERROR_NO_SUCH_CLAN_MEMBER);
    }

    if member.unwrap().status != Status::Invited {
        return Response::error(ErrorCode::SCE_NP_CLANS_SERVER_ERROR_MEMBER_STATUS_INVALID);
    }

    // Cancel the invitation
    clan.members.retain(|p| p.jid != req.request.jid);

    // Update the clan
    if database.clans.replace_one(doc! { "id": clan.id() }, clan).await.is_err() {
        return Response::error(ErrorCode::SCE_NP_CLANS_SERVER_ERROR_INTERNAL_SERVER_ERROR);
    }

    Response::success(Content::Empty)
}

/// Request to join a clan.
#[post("/clan_manager_update/sec/request_membership")]
pub async fn request_membership(database: Data<Database>, req: Request<RequestMembership>) -> Response<()> {
    let jid = Jid::from(req.request.ticket);

    // Find the clan
    let Ok(clan) = database.clans.find_one(doc! { "id": req.request.id }).await
    else { return Response::error(ErrorCode::SCE_NP_CLANS_SERVER_ERROR_INTERNAL_SERVER_ERROR) };

    if clan.is_none() {
        return Response::error(ErrorCode::SCE_NP_CLANS_SERVER_ERROR_NO_SUCH_CLAN);
    }

    let mut clan = clan.unwrap();

    // Check if the user has already been invited or is already pending approval
    let member = clan.members.iter().find(|m| m.jid == jid);
    if let Some(member) = member {
        if member.status == Status::Invited || member.status == Status::Pending {
            return Response::error(ErrorCode::SCE_NP_CLANS_SERVER_ERROR_BAD_REQUEST);
        }
    }

    // Check if the user has been blacklisted
    if clan.is_blacklisted(&jid) {
        return Response::error(ErrorCode::SCE_NP_CLANS_SERVER_ERROR_BLACKLISTED);
    }

    // Request membership
    let player = Player {
        jid,
        role: Role::NonMember,
        status: Status::Pending,
        ..Default::default()
    };

    clan.members.push(player);

    // Update the clan
    if database.clans.replace_one(doc! { "id": clan.id() }, clan).await.is_err() {
        return Response::error(ErrorCode::SCE_NP_CLANS_SERVER_ERROR_INTERNAL_SERVER_ERROR);
    }

    Response::success(Content::Empty)
}