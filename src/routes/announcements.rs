//! TODO: document this

use actix_web::{post, web::Data};

use crate::{database::Database, structs::{entities::{announcement::Announcement, clan::Clan, player::{Jid, Role, Status}}, requests::{announcements::{DeleteAnnouncement, PostAnnouncement, RetrieveAnnouncements}, base::Request}, responses::{base::{Content, List, Response}, entities::{AnnouncementInfo, IdEntity}, error::ErrorCode}}};

/// Retrieve a clan's announcements.
/// 
/// The author needs to:
///   - Be a member of the clan
#[post("/clan_manager_view/sec/retrieve_announcements")]
pub async fn retrieve_announcements(database: Data<Database>, req: Request<RetrieveAnnouncements>) -> Response<AnnouncementInfo> {
    let jid = Jid::from(req.request.ticket.clone());

    let clan = match Clan::resolve(req.request.id, &database).await {
        Ok(clan) => clan,
        Err(e) => return Response::error(e),
    };

    // Check if the author has permissions to view the announcements
    if !clan.status_of(&jid).map_or(false, |status| status == &Status::Member) {
        return Response::error(ErrorCode::PermissionDenied);
    }

    // Collect all valid entries
    let items = clan.announcements
        .iter()
        .skip((req.request.start - 1).max(0) as usize)
        .take(req.request.max as usize)
        .filter(|m| !m.has_expired())
        .map(|m| AnnouncementInfo::from(m.to_owned()))
        .collect::<Vec<AnnouncementInfo>>();

    let list = List {
        results: items.len() as u32,
        total: clan.announcements.len() as u32,

        items,
    };

    Response::success(Content::List(list))
}

/// Publish a new announcement for a clan.
/// 
/// The author needs to:
///     - Be at least a SubLeader of the clan
#[post("/clan_manager_update/sec/post_announcement")]
pub async fn post_announcement(database: Data<Database>, req: Request<PostAnnouncement>) -> Response<IdEntity> {
    let jid = Jid::from(req.request.ticket.clone());

    let mut clan = match Clan::resolve(req.request.id, &database).await {
        Ok(clan) => clan,
        Err(e) => return Response::error(e),
    };

    // Check if the author has permissions to post an announcement
    if !clan.role_of(&jid).map_or(false, |role| role >= &Role::SubLeader) {
        return Response::error(ErrorCode::PermissionDenied);
    }

    // Create the announcement
    let announcement = Announcement::from(req.request);
    let id = announcement.id();

    clan.announcements.push(announcement);

    // Update the clan
    if let Err(e) = clan.save(&database).await { return Response::error(e); }

    Response::success(Content::Item(IdEntity::from(id)))
}

/// Delete an announcement from a clan.
/// 
/// The author needs to:
///     - Be at least a SubLeader of the clan
#[post("/clan_manager_update/sec/delete_announcement")]
pub async fn delete_announcement(database: Data<Database>, req: Request<DeleteAnnouncement>) -> Response<()> {
    let jid = Jid::from(req.request.ticket.clone());

    let mut clan = match Clan::resolve(req.request.id, &database).await {
        Ok(clan) => clan,
        Err(e) => return Response::error(e),
    };

    // Check if the author has permissions to delete the announcement
    if !clan.role_of(&jid).map_or(false, |role| role >= &Role::SubLeader) {
        return Response::error(ErrorCode::PermissionDenied);
    }

    // Remove the announcement
    let Some(index) = clan.announcements.iter().position(|m| m.id() == req.request.msg_id)
    else { return Response::error(ErrorCode::NoSuchClanAnnouncement) };

    clan.announcements.remove(index);

    // Update the clan
    if let Err(e) = clan.save(&database).await { return Response::error(e); }

    Response::success(Content::Empty)
}