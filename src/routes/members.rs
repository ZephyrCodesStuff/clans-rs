//! Routes pertaining to a clan's members, such as:
//!
//! - Getting a clan's members
//! - Inviting a player to a clan
//! - Removing a player from a clan
//! - ...

use actix_web::{post, web::Data};
use mongodb::bson::doc;

use crate::{database::Database, structs::{
    requests::{base::Request, members::GetMemberList}, responses::{
        base::{Content, ErrorCode, List, Response},
        entities::PlayerInfo,
    }
}};

/// Get a clan's members.
#[post("/clan_manager_view/sec/get_member_list")]
#[allow(clippy::cast_possible_truncation)]
pub async fn get_member_list(database: Data<Database>, req: Request<GetMemberList>) -> Response<PlayerInfo> {
    // Find the clan
    let Ok(clan) = database.clans.find_one(doc! { "id": req.request.clan_id }).await
    else { return Response::error(ErrorCode::SCE_NP_CLANS_SERVER_ERROR_INTERNAL_SERVER_ERROR) };

    if clan.is_none() {
        return Response::error(ErrorCode::SCE_NP_CLANS_SERVER_ERROR_BAD_REQUEST);
    }

    let clan = clan.unwrap();

    // Collect all valid entries
    let items = clan.members
        .iter()
        .skip(req.request.start as usize)
        .take(req.request.max as usize)
        .map(|m| PlayerInfo::from(m.to_owned()))
        .collect::<Vec<PlayerInfo>>();

    let list = List {
        results: items.len() as u32,
        total: clan.members.len() as u32,

        items,
    };

    Response::success(Content::List(list))
}