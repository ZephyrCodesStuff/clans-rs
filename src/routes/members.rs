//! Routes pertaining to a clan's members, such as:
//!
//! - Getting a clan's members
//! - Inviting a player to a clan
//! - Removing a player from a clan
//! - ...

use actix_web::{post, web::Data};
use mongodb::bson::doc;

use crate::{database::Database, structs::{
    entities::{clan::Clan, player::{Jid, Player, Role, Status}}, requests::{base::Request, members::{ChangeMemberRole, GetMemberInfo, GetMemberList, JoinClan, KickMember, LeaveClan, UpdateMemberInfo}}, responses::{
        base::{Content, List, Response},
        entities::{PlayerBasicInfo, PlayerInfo}, error::ErrorCode,
    }
}};

/// Get a clan's members.
#[post("/clan_manager_view/sec/get_member_list")]
#[allow(clippy::cast_possible_truncation)]
pub async fn get_member_list(database: Data<Database>, req: Request<GetMemberList>) -> Response<PlayerBasicInfo> {
    let clan = match Clan::resolve(req.request.id, &database).await {
        Ok(clan) => clan,
        Err(e) => return Response::error(e),
    };

    // Collect all valid entries
    let items = clan.members
        .iter()
        .skip((req.request.start - 1).max(0) as usize)
        .take(req.request.max as usize)
        .map(|m| PlayerBasicInfo::from(m.to_owned()))
        .collect::<Vec<PlayerBasicInfo>>();

    let list = List {
        results: items.len() as u32,
        total: clan.members.len() as u32,

        items,
    };

    Response::success(Content::List(list))
}

/// Get info about a clan member.
/// 
/// The author needs to:
///    - Be a member of the clan
#[post("/clan_manager_view/sec/get_member_info")]
pub async fn get_member_info(database: Data<Database>, req: Request<GetMemberInfo>) -> Response<PlayerInfo> {
    let author = Jid::from(req.request.ticket);

    let clan = match Clan::resolve(req.request.id, &database).await {
        Ok(clan) => clan,
        Err(e) => return Response::error(e),
    };

    // Check if the user is allowed to view the player's info
    if !clan.status_of(&author).map_or(false, |status| status == &Status::Member) {
        return Response::error(ErrorCode::PermissionDenied);
    }
    
    // Find the player
    let Ok(target) = Jid::try_from(req.request.jid.clone())
    else { return Response::error(ErrorCode::InvalidNpId) };

    let Some(player) = clan.members.iter().find(|p| p.jid == target)
    else { return Response::error(ErrorCode::NoSuchClanMember) };

    Response::success(Content::Item(PlayerInfo::from(player.to_owned())))
}

/// Kick a member from a clan.
/// 
/// The author needs to:
///     - Be a SubLeader or higher
/// 
/// The player needs to:
///     - Not be a SubLeader or higher
#[post("/clan_manager_update/sec/kick_member")]
pub async fn kick_member(database: Data<Database>, req: Request<KickMember>) -> Response<()> {
    let author = Jid::from(req.request.ticket);
    let Ok(target) = Jid::try_from(req.request.jid.clone())
    else { return Response::error(ErrorCode::InvalidNpId) };

    let mut clan = match Clan::resolve(req.request.id, &database).await {
        Ok(clan) => clan,
        Err(e) => return Response::error(e),
    };

    // Check if the user is allowed to kick the player
    if !clan.role_of(&author).map_or(false, |role| role >= &Role::SubLeader) {
        return Response::error(ErrorCode::PermissionDenied);
    }

    // Check if the player is a member of the clan
    if clan.status_of(&target).map_or(false, |status| status != &Status::Member) {
        return Response::error(ErrorCode::MemberStatusInvalid);
    }

    // Check if the player is allowed to be kicked
    if clan.role_of(&target).map_or(false, |role| role >= &Role::SubLeader) {
        return Response::error(ErrorCode::PermissionDenied);
    }

    // Remove the player
    clan.members.retain(|p| p.jid != target);

    // Update the clan
    if let Err(e) = clan.save(&database).await { return Response::error(e); }

    Response::success(Content::Empty)
}

/// Change a player's role in a clan.
///
/// The author needs to:
///     - Be a SubLeader or higher
/// 
/// The player needs to:
///     - Be a member of the clan
#[post("/clan_manager_update/sec/change_member_role")]
pub async fn change_member_role(database: Data<Database>, req: Request<ChangeMemberRole>) -> Response<()> {
    let author = Jid::from(req.request.ticket);
    let Ok(target) = Jid::try_from(req.request.jid.clone())
    else { return Response::error(ErrorCode::InvalidNpId) };

    let mut clan = match Clan::resolve(req.request.id, &database).await {
        Ok(clan) => clan,
        Err(e) => return Response::error(e),
    };

    // Check if the user is allowed to change the player's role
    if !clan.role_of(&author).map_or(false, |role| role >= &Role::SubLeader) {
        return Response::error(ErrorCode::PermissionDenied);
    }

    // Check if the player is a member of the clan
    if clan.status_of(&target).map_or(false, |status| status != &Status::Member) {
        return Response::error(ErrorCode::MemberStatusInvalid);
    }

    // Change the player's role
    let role = Role::from(req.request.role);
    let index = clan.members.iter().position(|p| p.jid == target).unwrap();
    clan.members.get_mut(index).unwrap().role = role;

    // Update the clan
    if let Err(e) = clan.save(&database).await { return Response::error(e); }

    Response::success(Content::Empty)
}

/// Update a clan member's info.
///
/// The author needs to:
///     - Be the player
///     - Be a member of the clan
#[post("/clan_manager_update/sec/update_member_info")]
pub async fn update_member_info(database: Data<Database>, req: Request<UpdateMemberInfo>) -> Response<()> {
    let author = Jid::from(req.request.ticket);

    let mut clan = match Clan::resolve(req.request.id, &database).await {
        Ok(clan) => clan,
        Err(e) => return Response::error(e),
    };

    // Check if the user is allowed to update the player's info
    if !clan.status_of(&author).map_or(false, |status| status == &Status::Member) {
        return Response::error(ErrorCode::PermissionDenied);
    }

    // Update the player's info
    let member = clan.members.iter_mut().find(|p| p.jid == author).unwrap();
    member.online_name = req.request.onlinename;
    member.description = req.request.description;
    member.allow_msg = req.request.allowmsg;
    member.bin_data = req.request.bin_attr1;
    member.size = req.request.size;

    // Update the clan
    if let Err(e) = clan.save(&database).await { return Response::error(e); }

    Response::success(Content::Empty)
}

/// Join a clan.
/// 
/// The author needs to:
///     - Not be a member of a clan
/// 
/// The clan needs to:
///     - Have the ``auto_accept`` attribute set to ``true``.
#[post("/clan_manager_update/sec/join_clan")]
pub async fn join_clan(database: Data<Database>, req: Request<JoinClan>) -> Response<()> {
    let author = Jid::from(req.request.ticket);

    let mut clan = match Clan::resolve(req.request.id, &database).await {
        Ok(clan) => clan,
        Err(e) => return Response::error(e),
    };

    // Check if the player is already in a clan
    if clan.members.iter().any(|p| p.jid == author) {
        return Response::error(ErrorCode::MemberStatusInvalid);
    }

    // Check if the clan accepts new members
    if !clan.auto_accept {
        return Response::error(ErrorCode::PermissionDenied);
    }

    // Add the player
    clan.members.push(Player {
        jid: author,
        role: Role::Member,
        ..Default::default()
    });

    // Update the clan
    if let Err(e) = clan.save(&database).await { return Response::error(e); }

    Response::success(Content::Empty)
}

/// Leave a clan.
/// 
/// The author needs to:
///     - Be a member of the clan
#[post("/clan_manager_update/sec/leave_clan")]
pub async fn leave_clan(database: Data<Database>, req: Request<LeaveClan>) -> Response<()> {
    let author = Jid::from(req.request.ticket);

    let mut clan = match Clan::resolve(req.request.id, &database).await {
        Ok(clan) => clan,
        Err(e) => return Response::error(e),
    };

    // Check if the player is a member of the clan
    if !clan.status_of(&author).map_or(false, |status| status == &Status::Member) {
        return Response::error(ErrorCode::MemberStatusInvalid);
    }

    // Remove the player
    clan.members.retain(|p| p.jid != author);

    // Update the clan
    if let Err(e) = clan.save(&database).await { return Response::error(e); }

    Response::success(Content::Empty)
}