//! TODO: document this

use actix_web::{post, web::Data};
use futures_util::StreamExt;
use mongodb::bson::doc;

use crate::{database::Database, structs::{entities::{clan::{Clan, Platform, MAX_CLAN_MEMBERSHIP}, player::{Jid, Player, Role, Status}}, requests::{base::Request, invites::{AcceptInvitation, AcceptMembershipRequest, CancelInvitation, CancelRequestMembership, DeclineInvitation, DeclineMembershipRequest, RequestMembership, SendInvitation}}, responses::{base::{Content, Response}, error::ErrorCode}}};

/// Invite a player to a clan.
/// 
/// The author needs to:
///     - Be a member of the clan
/// 
/// The player needs to:
///     - Not be a member of the clan
///     - Not have been invited to the clan
///     - Not be blacklisted
#[post("/clan_manager_update/sec/send_invitation")]
pub async fn send_invitation(database: Data<Database>, req: Request<SendInvitation>) -> Response<()> {
    let jid = Jid::from(req.request.ticket);

    let mut clan = match Clan::resolve(req.request.id, &database).await {
        Ok(clan) => clan,
        Err(e) => return Response::error(e),
    };

    // Check if the author has permissions to invite the player
    if !clan.role_of(&jid).map_or(false, |role| role >= &Role::Member) {
        return Response::error(ErrorCode::PermissionDenied);
    }

    // Check if the player is already a member or has been invited
    if clan.status_of(&req.request.jid).map_or(false, |status| status == &Status::Member || status == &Status::Invited) {
        return Response::error(ErrorCode::MemberStatusInvalid);
    }

    // Check if the user has been blacklisted
    if clan.is_blacklisted(&jid) {
        return Response::error(ErrorCode::Blacklisted);
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
    if let Err(e) = clan.save(&database).await { return Response::error(e); }

    Response::success(Content::Empty)
}

/// Cancel an invitation to a player.
/// 
/// The author needs to:
///     - Be a member of the clan
/// 
/// The player needs to:
///     - Have been invited
#[post("/clan_manager_update/sec/cancel_invitation")]
pub async fn cancel_invitation(database: Data<Database>, req: Request<CancelInvitation>) -> Response<()> {
    let jid = Jid::from(req.request.ticket);

    let mut clan = match Clan::resolve(req.request.id, &database).await {
        Ok(clan) => clan,
        Err(e) => return Response::error(e),
    };

    // Check if the author has permissions to cancel the invitation
    if !clan.role_of(&jid).map_or(false, |role| role >= &Role::Member) {
        return Response::error(ErrorCode::PermissionDenied);
    }

    // Check if the user has been invited
    if clan.status_of(&req.request.jid).map_or(false, |status| status != &Status::Invited) {
        return Response::error(ErrorCode::MemberStatusInvalid);
    }

    // Cancel the invitation
    clan.members.retain(|p| p.jid != req.request.jid);

    // Update the clan
    if let Err(e) = clan.save(&database).await { return Response::error(e); }

    Response::success(Content::Empty)
}

/// Accept an invitation to join a clan.
/// 
/// The player needs to:
///     - Have been invited
///     - Not be blacklisted
///     - Not be a member of the clan
#[post("/clan_manager_update/sec/accept_invitation")]
pub async fn accept_invitation(database: Data<Database>, req: Request<AcceptInvitation>) -> Response<()> {
    let jid = Jid::from(req.request.ticket.clone());
    let platform = Platform::from(req.request.ticket);

    let mut clan = match Clan::resolve(req.request.id, &database).await {
        Ok(clan) => clan,
        Err(e) => return Response::error(e),
    };

    // Check if the user has been blacklisted
    if clan.is_blacklisted(&jid) {
        return Response::error(ErrorCode::Blacklisted);
    }

    // Check if the user has been invited
    if !clan.status_of(&jid).map_or(false, |status| status == &Status::Invited) {
        return Response::error(ErrorCode::MemberStatusInvalid);
    }

    // Check if the clan was created for the same platform as the player
    if clan.platform != platform {
        return Response::error(ErrorCode::InvalidEnvironment);
    }

    // Check if the player is in too many clans
    let clans = match database.clans.find(doc! { "$and": [{ "members.jid": jid.to_string() }, { "members.status": Status::Member.to_string() }] })
        .await.map_err(|_| ErrorCode::InternalServerError)
    {
        Ok(clans) => clans,
        Err(e) => return Response::error(e),
    };

    // If the player is in 5 or more clans, return an error
    if clans.count().await >= MAX_CLAN_MEMBERSHIP {
        return Response::error(ErrorCode::ClanJoinedLimitReached);
    }

    // Accept the invitation
    let player = clan.members.iter_mut().find(|p| p.jid == jid).unwrap();
    player.status = Status::Member;
    player.role = Role::Member;

    // Update the clan
    if let Err(e) = clan.save(&database).await { return Response::error(e); }

    Response::success(Content::Empty)
}

/// Decline an invitation to join a clan.
/// 
/// The player needs to:
///     - Have been invited
#[post("/clan_manager_update/sec/decline_invitation")]
pub async fn decline_invitation(database: Data<Database>, req: Request<DeclineInvitation>) -> Response<()> {
    let jid = Jid::from(req.request.ticket);

    let mut clan = match Clan::resolve(req.request.id, &database).await {
        Ok(clan) => clan,
        Err(e) => return Response::error(e),
    };

    // Check if the user has been invited
    if !clan.status_of(&jid).map_or(false, |status| status == &Status::Invited) {
        return Response::error(ErrorCode::MemberStatusInvalid);
    }

    // Decline the invitation
    clan.members.retain(|p| p.jid != jid);

    // Update the clan
    if let Err(e) = clan.save(&database).await { return Response::error(e); }

    Response::success(Content::Empty)
}

/// Request to join a clan.
/// 
/// The player needs to:
///     - Not be a member of the clan
///     - Not have requested to join
///     - Not be blacklisted
#[post("/clan_manager_update/sec/request_membership")]
pub async fn request_membership(database: Data<Database>, req: Request<RequestMembership>) -> Response<()> {
    let jid = Jid::from(req.request.ticket.clone());
    let platform = Platform::from(req.request.ticket);

    let mut clan = match Clan::resolve(req.request.id, &database).await {
        Ok(clan) => clan,
        Err(e) => return Response::error(e),
    };

    // Check if the user has already been invited or is already pending approval
    if clan.status_of(&jid).map_or(false, |status| status == &Status::Member || status == &Status::Invited || status == &Status::Pending) {
        return Response::error(ErrorCode::MemberStatusInvalid);
    }

    // Check if the user has been blacklisted
    if clan.is_blacklisted(&jid) {
        return Response::error(ErrorCode::Blacklisted);
    }

    // Check if the clan was created for the same platform as the player
    if clan.platform != platform {
        return Response::error(ErrorCode::InvalidEnvironment);
    }

    // Check if the player is in too many clans
    let clans = match database.clans.find(doc! { "$and": [{ "members.jid": jid.to_string() }, { "members.status": Status::Member.to_string() }] })
        .await.map_err(|_| ErrorCode::InternalServerError)
    {
        Ok(clans) => clans,
        Err(e) => return Response::error(e),
    };

    // If the player is in 5 or more clans, return an error
    if clans.count().await >= MAX_CLAN_MEMBERSHIP {
        return Response::error(ErrorCode::ClanJoinedLimitReached);
    }

    // Determine the player's role and status based on the clan's auto-accept setting
    let (role, status) = if clan.auto_accept { (Role::Member, Status::Member) } else { (Role::NonMember, Status::Pending) };

    // Request membership
    let player = Player {
        jid,
        role,
        status,
        ..Default::default()
    };

    clan.members.push(player);

    // Update the clan
    if let Err(e) = clan.save(&database).await { return Response::error(e); }

    Response::success(Content::Empty)
}

/// Cancel a request to join a clan.
/// 
/// The player needs to:
///     - Be a pending member
#[post("/clan_manager_update/sec/cancel_request_membership")]
pub async fn cancel_request_membership(database: Data<Database>, req: Request<CancelRequestMembership>) -> Response<()> {
    let jid = Jid::from(req.request.ticket);

    let mut clan = match Clan::resolve(req.request.id, &database).await {
        Ok(clan) => clan,
        Err(e) => return Response::error(e),
    };

    // Check if the user has already been invited or is already pending approval
    if !clan.status_of(&jid).map_or(false, |status| status == &Status::Pending) {
        return Response::error(ErrorCode::MemberStatusInvalid);
    }

    // Cancel the request
    clan.members.retain(|p| p.jid != jid);

    // Update the clan
    if let Err(e) = clan.save(&database).await { return Response::error(e); }

    Response::success(Content::Empty)
}

/// Approve a player's request to join a clan.
/// 
/// The author needs to:
///     - Be a member of the clan
/// 
/// The player needs to:
///     - Have requested to join
///     - Not be blacklisted
#[post("/clan_manager_update/sec/accept_membership_request")]
pub async fn accept_membership_request(database: Data<Database>, req: Request<AcceptMembershipRequest>) -> Response<()> {
    let jid = Jid::from(req.request.ticket.clone());

    let mut clan = match Clan::resolve(req.request.id, &database).await {
        Ok(clan) => clan,
        Err(e) => return Response::error(e),
    };

    // Check if the author has permissions to accept the player
    if !clan.role_of(&jid).map_or(false, |role| role >= &Role::Member) {
        return Response::error(ErrorCode::PermissionDenied);
    }

    // Check if the user has been blacklisted
    if clan.is_blacklisted(&jid) {
        return Response::error(ErrorCode::Blacklisted);
    }

    // Check if the user has requested to join
    if !clan.status_of(&req.request.jid).map_or(false, |status| status == &Status::Pending) {
        return Response::error(ErrorCode::MemberStatusInvalid);
    }

    // Accept the request
    let player = clan.members.iter_mut().find(|p| p.jid == req.request.jid).unwrap();
    player.status = Status::Member;
    player.role = Role::Member;

    // Update the clan
    if let Err(e) = clan.save(&database).await { return Response::error(e); }

    Response::success(Content::Empty)
}

/// Decline a player's request to join a clan.
#[post("/clan_manager_update/sec/decline_membership_request")]
pub async fn decline_membership_request(database: Data<Database>, req: Request<DeclineMembershipRequest>) -> Response<()> {
    let jid = Jid::from(req.request.ticket);

    let mut clan = match Clan::resolve(req.request.id, &database).await {
        Ok(clan) => clan,
        Err(e) => return Response::error(e),
    };

    // Check if the author has permissions to decline the player
    if !clan.role_of(&jid).map_or(false, |role| role >= &Role::Member) {
        return Response::error(ErrorCode::PermissionDenied);
    }

    // Check if the user has requested to join
    if !clan.status_of(&req.request.jid).map_or(false, |status| status != &Status::Pending) {
        return Response::error(ErrorCode::MemberStatusInvalid);
    }

    // Decline the request
    clan.members.retain(|p| p.jid != req.request.jid);

    // Update the clan
    if let Err(e) = clan.save(&database).await { return Response::error(e); }

    Response::success(Content::Empty)
}