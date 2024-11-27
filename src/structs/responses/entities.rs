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

use crate::{
    structs::entities::{
        announcement::{Announcement, Id as AnnouncementId},
        clan::{Clan, Id as ClanId},
        player::{Jid, Player, Role, Status},
    },
    utils::{self, xml_format::ToXML},
};

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
    id: ClanId,
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
            (
                "date-created",
                &utils::date_format::iso8601(&self.date_created),
            ),
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
    id: ClanId,
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
    id: ClanId,
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
        let role = clan.role_of(&player).unwrap_or(&Role::NonMember);

        let status = clan.status_of(&player).unwrap_or(&Status::Unknown);

        let allow_msg = clan
            .members
            .iter()
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

/// XML entity for a player's full info.
/// See: [`Player`]
/// 
/// ### Used for:
/// - `/get_member_info`.
/// 
/// ### XML format:
/// ```xml
/// <info jid="{jid}">
///     <role>{role}</role>
///     <status>{status}</status>
///     <onlinename>{online_name}</onlinename>
///     <description>{description}</description>
///     <allowmsg>{allow_msg}</allowmsg>
///     <bin-atrr1>{bin_data}</bin-atrr1>
///     <size>{size}</size>
/// </info>
/// ```
#[derive(Debug, Clone)]
pub struct PlayerInfo {
    jid: String,
    role: u32,
    status: u32,
    online_name: String,
    description: String,
    allow_msg: u8,
    bin_data: String,
    size: u32,
}

impl From<Player> for PlayerInfo {
    fn from(player: Player) -> Self {
        Self {
            jid: player.jid.to_string(),
            role: player.role as u32,
            status: player.status as u32,
            online_name: player.online_name,
            description: player.description,
            allow_msg: u8::from(player.allow_msg),
            bin_data: player.bin_data,
            size: player.size,
        }
    }
}

impl ToXML for PlayerInfo {
    fn to_xml(&self) -> String {
        let mut writer = EmitterConfig::new()
            .perform_indent(false)
            .write_document_declaration(false)
            .create_writer(Vec::new());

        let element = XmlEvent::start_element("info").attr("jid", &self.jid);
        writer.write(element).ok();

        for (elem, value) in [
            ("role", &self.role.to_string()),
            ("status", &self.status.to_string()),
            ("onlinename", &self.online_name),
            ("description", &self.description),
            ("allowmsg", &self.allow_msg.to_string()),
            ("bin-atrr1", &self.bin_data),
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

/// XML entity for a player's basic info.
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
pub struct PlayerBasicInfo {
    jid: String,
    role: u32,
    status: u32,
    description: String,
}

impl From<Player> for PlayerBasicInfo {
    fn from(player: Player) -> Self {
        Self {
            jid: player.jid.to_string(),
            role: player.role as u32,
            status: player.status as u32,
            description: player.description,
        }
    }
}

impl ToXML for PlayerBasicInfo {
    fn to_xml(&self) -> String {
        let mut writer = EmitterConfig::new()
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

impl From<Jid> for BlacklistEntry {
    fn from(jid: Jid) -> Self {
        Self {
            jid: jid.to_string(),
        }
    }
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
        let mut writer = EmitterConfig::new()
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
pub struct IdEntity {
    id: ClanId,
}

impl From<Clan> for IdEntity {
    fn from(clan: Clan) -> Self {
        Self { id: clan.id() }
    }
}

impl From<u32> for IdEntity {
    fn from(id: ClanId) -> Self {
        Self { id }
    }
}

impl ToXML for IdEntity {
    fn to_xml(&self) -> String {
        let mut writer = EmitterConfig::new()
            .perform_indent(false)
            .write_document_declaration(false)
            .create_writer(Vec::new());

        writer.write(XmlEvent::start_element("id")).ok();
        writer
            .write(XmlEvent::characters(&self.id.to_string()))
            .ok();
        writer.write(XmlEvent::end_element()).ok();

        let result = writer.into_inner();
        String::from_utf8(result).unwrap()
    }
}

/// XML entity for an announcement.
///
/// ### Used for:
/// - `/retrieve_announcements`
///
/// ### XML format:
/// ```xml
/// <msg-info id="{id}">
///     <subject>{subject}</subject>
///     <msg>{msg}</msg>
///     <jid>{jid}</jid>
///     <msg-date>{msg_date}</msg-date>
///     <bin-data>{bin_data}</bin-data>
///     <from-id>{from_id}</from-id>
/// </msg-info>
#[derive(Debug, Clone)]
pub struct AnnouncementInfo {
    id: AnnouncementId,
    subject: String,
    msg: String,
    jid: String,
    msg_date: DateTime<Utc>,
    bin_data: String,
    from_id: AnnouncementId,
}

impl From<Announcement> for AnnouncementInfo {
    fn from(announcement: Announcement) -> Self {
        Self {
            id: announcement.id(),
            subject: announcement.subject,
            msg: announcement.msg,
            jid: announcement.author.to_string(),
            msg_date: announcement.date_created,
            bin_data: announcement.bin_data,
            from_id: announcement.from_id,
        }
    }
}

impl ToXML for AnnouncementInfo {
    fn to_xml(&self) -> String {
        let mut writer = EmitterConfig::new()
            .perform_indent(false)
            .write_document_declaration(false)
            .create_writer(Vec::new());

        let id = self.id.to_string();
        let element = XmlEvent::start_element("msg-info").attr("id", &id);
        writer.write(element).ok();

        for (elem, value) in [
            ("subject", &self.subject),
            ("msg", &self.msg),
            ("jid", &self.jid),
            ("msg-date", &utils::date_format::iso8601(&self.msg_date)),
            ("bin-data", &self.bin_data),
            ("from-id", &self.from_id.to_string()),
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
