//! XML entities for the responses, as the game
//! expects them.
//! 
//! These provide implementations for the `From` trait
//! for the structs in the `entities` module, so that
//! they can be converted from our schema, to the XML
//! format that the game expects.

#![allow(clippy::missing_docs_in_private_items)]

use chrono::{DateTime, Utc};
use xml::{writer::XmlEvent, EmitterConfig};

use crate::{structs::entities::{clan::{Clan, Id}, player::{Jid, Player, Role, Status}}, utils::{self, xml_format::ToXML}};

/// Full XML entity for a clan.
/// See: [`Clan`]
/// 
/// ### Used for:
/// - `/get_clan_info`.
/// 
/// ### XML format:
/// ```xml
/// <info id="{id}">
///     <name>{name}</name>
///     <tag>{tag}</tag>
///     <members>{members}</members>
///     <date-created>{date_created}</date-created>
///     <description>{description}</description>
///     <auto-accept>{auto_accept}</auto-accept>
///     <int-attr1>{int_attr1}</int-attr1>
///     <int-attr2>{int_attr2}</int-attr2>
///     <int-attr3>{int_attr3}</int-attr3>
///     <size>{size}</size>
/// </info>
#[derive(Debug, Clone)]
pub struct ClanInfo {
    id: Id,
    name: String,
    tag: String,
    description: String,
    members: u32,
    date_created: DateTime<Utc>,
    auto_accept: u8,
    int_attr1: u32,
    int_attr2: u32,
    int_attr3: u32,
    size: u32,
}

#[allow(clippy::cast_possible_truncation)]
impl From<Clan> for ClanInfo {
    fn from(clan: Clan) -> Self {
        Self {
            id: clan.id(),
            name: clan.name,
            tag: clan.tag,
            description: clan.description,
            members: clan.members.len() as u32,
            date_created: clan.date_created,
            auto_accept: u8::from(clan.auto_accept),
            int_attr1: clan.int_attr1,
            int_attr2: clan.int_attr2,
            int_attr3: clan.int_attr3,
            size: clan.size,
        }
    }
}

impl ToXML for ClanInfo {
    fn to_xml(&self) -> String {
        let mut writer = EmitterConfig::new()
            .perform_indent(false)
            .write_document_declaration(false)
            .create_writer(Vec::new());

        let clan_id = self.id.to_string();
        let element = XmlEvent::start_element("info").attr("id", &clan_id);
        writer.write(element).ok();

        for (elem, value) in [
            ("name", &self.name),
            ("tag", &self.tag),
            ("members", &self.members.to_string()),
            ("date-created", &utils::date_format::iso8601(&self.date_created)),
            ("description", &self.description),
            ("auto-accept", &self.auto_accept.to_string()),
            ("int-attr1", &self.int_attr1.to_string()),
            ("int-attr2", &self.int_attr2.to_string()),
            ("int-attr3", &self.int_attr3.to_string()),
            ("size", &self.size.to_string()),
        ] {
            writer.write(XmlEvent::start_element(elem)).ok();
            writer.write(XmlEvent::characters(value)).ok();
            writer.write(XmlEvent::end_element()).ok();
        }

        writer.write(XmlEvent::end_element()).ok();

        let result = writer.into_inner();
        String::from_utf8(result).unwrap()
    }
}

/// Short XML entity for a clan.
/// See: [`Clan`]
/// 
/// ### Used for:
/// - `/clan_search`.
/// 
/// ### XML format:
/// ```xml
/// <info id="{id}">
///     <name>{name}</name>
///     <tag>{tag}</tag>
///     <members>{members}</members>
/// </info>
/// ```
#[derive(Debug, Clone)]
pub struct ClanSearchInfo {
    id: Id,
    name: String,
    tag: String,
    members: u32,
}

#[allow(clippy::cast_possible_truncation)]
impl From<Clan> for ClanSearchInfo {
    fn from(clan: Clan) -> Self {
        Self {
            id: clan.id(),
            name: clan.name,
            tag: clan.tag,
            members: clan.members.len() as u32,
        }
    }
}

impl ToXML for ClanSearchInfo {
    fn to_xml(&self) -> String {
        let mut writer = EmitterConfig::new()
            .perform_indent(false)
            .write_document_declaration(false)
            .create_writer(Vec::new());

        let clan_id = self.id.to_string();
        let element = XmlEvent::start_element("info").attr("id", &clan_id);
        writer.write(element).ok();

        for (elem, value) in [
            ("name", &self.name),
            ("tag", &self.tag),
            ("members", &self.members.to_string()),
        ] {
            writer.write(XmlEvent::start_element(elem)).ok();
            writer.write(XmlEvent::characters(value)).ok();
            writer.write(XmlEvent::end_element()).ok();
        }

        writer.write(XmlEvent::end_element()).ok();

        let result = writer.into_inner();
        String::from_utf8(result).unwrap()
    }
}

/// XML entity for a clan, from the perspective of a player.
/// See: [`Clan`]
/// 
/// ### Used for:
/// - `/get_clan_list`.
/// 
/// ### XML format:
/// ```xml
/// <info id="{id}">
///     <name>{name}</name>
///     <tag>{tag}</tag>
///     <role>{role}</role>
///     <status>{status}</status>
///     <onlinename>{online_name}</onlinename>
///     <allowmsg>{allow_msg}</allowmsg>
///     <members>{members}</members>
/// </info>
/// ```
#[derive(Debug, Clone)]
pub struct ClanPlayerInfo {
    id: Id,
    name: String,
    tag: String,
    role: u32,
    status: u32,
    online_name: String,
    allow_msg: u8,
    members: u32,
}

#[allow(clippy::cast_possible_truncation)]
impl From<(Clan, Jid)> for ClanPlayerInfo {
    fn from((clan, player): (Clan, Jid)) -> Self {
        let role = clan.role_of(&player)
            .unwrap_or(&Role::NonMember);

        let status = clan.status_of(&player)
            .unwrap_or(&Status::Unknown);

        let allow_msg = clan.members.iter()
            .find(|p| p.jid == player)
            .map_or(0, |p| u8::from(p.allow_msg));

        Self {
            id: clan.id(),
            name: clan.name.clone(),
            tag: clan.tag.clone(),
            role: *role as u32,
            status: *status as u32,
            online_name: player.username.clone(),
            allow_msg,
            members: clan.members.len() as u32,
        }
    }
}

impl ToXML for ClanPlayerInfo {
    fn to_xml(&self) -> String {
        let mut writer = EmitterConfig::new()
            .perform_indent(false)
            .write_document_declaration(false)
            .create_writer(Vec::new());

        let clan_id = self.id.to_string();
        let element = XmlEvent::start_element("info").attr("id", &clan_id);
        writer.write(element).ok();

        for (elem, value) in [
            ("name", &self.name),
            ("tag", &self.tag),
            ("role", &self.role.to_string()),
            ("status", &self.status.to_string()),
            ("onlinename", &self.online_name),
            ("allowmsg", &self.allow_msg.to_string()),
            ("members", &self.members.to_string()),
        ] {
            writer.write(XmlEvent::start_element(elem)).ok();
            writer.write(XmlEvent::characters(value)).ok();
            writer.write(XmlEvent::end_element()).ok();
        }

        writer.write(XmlEvent::end_element()).ok();

        let result = writer.into_inner();
        String::from_utf8(result).unwrap()
    }
}

/// XML entity for a player.
/// See: [`Player`]
/// 
/// ### Used for:
/// - `/get_member_list`.
/// 
/// ### XML format:
/// ```xml
/// <info jid="{jid}">
///     <role>{role}</role>
///     <status>{status}</status>
///     <description>{description}</description>
/// </info>
/// ```
#[derive(Debug, Clone)]
pub struct PlayerInfo {
    jid: String,
    role: u32,
    status: u32,
    description: String,
}

impl From<Player> for PlayerInfo {
    fn from(player: Player) -> Self {
        Self {
            jid: player.jid.to_string(),
            role: player.role as u32,
            status: player.status as u32,
            description: player.description,
        }
    }
}

impl ToXML for PlayerInfo {
    fn to_xml(&self) -> String {
        let mut writer =
            EmitterConfig::new()
            .perform_indent(false)
            .write_document_declaration(false)
            .create_writer(Vec::new());

        let element = XmlEvent::start_element("info").attr("jid", &self.jid);
        writer.write(element).ok();

        for (elem, value) in [
            ("role", &self.role.to_string()),
            ("status", &self.status.to_string()),
            ("description", &self.description),
        ] {
            writer.write(XmlEvent::start_element(elem)).ok();
            writer.write(XmlEvent::characters(value)).ok();
            writer.write(XmlEvent::end_element()).ok();
        }

        writer.write(XmlEvent::end_element()).ok();

        let result = writer.into_inner();
        String::from_utf8(result).unwrap()
    }
}

/// Blacklist entry for a clan.
/// See: [`Player`]
/// 
/// ### Used for:
/// - `/get_blacklist`
///
/// ### XML format:
/// ```xml
/// <entry>
///     <jid>{jid}</jid>
/// </entry>
/// ```
#[derive(Debug, Clone)]
pub struct BlacklistEntry {
    jid: String,
}

impl From<Player> for BlacklistEntry {
    fn from(player: Player) -> Self {
        Self {
            jid: player.jid.to_string(),
        }
    }
}

impl ToXML for BlacklistEntry {
    fn to_xml(&self) -> String {
        let mut writer =
            EmitterConfig::new()
            .perform_indent(false)
            .write_document_declaration(false)
            .create_writer(Vec::new());

        let element = XmlEvent::start_element("entry");
        writer.write(element).ok();

        writer.write(XmlEvent::start_element("jid")).ok();
        writer.write(XmlEvent::characters(&self.jid)).ok();
        writer.write(XmlEvent::end_element()).ok();

        writer.write(XmlEvent::end_element()).ok();

        let result = writer.into_inner();
        String::from_utf8(result).unwrap()
    }
}

/// XML entity for an ID.
/// See: [`Id`]
/// 
/// ### Used for:
/// - `/create_clan`
/// 
/// ### XML format:
/// ```xml
/// <id>{id}</id>
/// ```
#[derive(Debug, Clone)]
pub struct ClanId {
    id: Id,
}

impl From<Clan> for ClanId {
    fn from(clan: Clan) -> Self {
        Self {
            id: clan.id(),
        }
    }
}

impl From<Id> for ClanId {
    fn from(id: Id) -> Self {
        Self {
            id,
        }
    }
}

impl ToXML for ClanId {
    fn to_xml(&self) -> String {
        let mut writer =
            EmitterConfig::new()
            .perform_indent(false)
            .write_document_declaration(false)
            .create_writer(Vec::new());

        writer.write(XmlEvent::start_element("id")).ok();
        writer.write(XmlEvent::characters(&self.id.to_string())).ok();
        writer.write(XmlEvent::end_element()).ok();

        let result = writer.into_inner();
        String::from_utf8(result).unwrap()
    }
}