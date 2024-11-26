//! Base XML structs for the responses of the API,
//! wrapping the XML entities inside.

use actix_web::{http::StatusCode, Responder};
use serde::Serialize;
use xml::{writer::XmlEvent, EmitterConfig};

use crate::utils::xml_format::ToXML;

/// Default headers for the response.
const HEADERS: [(&str, &str); 3] = [
	("Message-Type", "x-ps3-clan"),
	("Version", "1.00"),
	("Content-Type", "application/x-ps3-clan"),
];

/// A generic clan response, with a status code and content.
/// 
/// ```xml
/// <clan result="{status}">
/// ...
/// </clan>
/// ```
#[derive(Debug)]
pub struct Response<T: ToXML> {
    /// Status code of the response.
    status: Status,

    /// Content of the response.
    content: Content<T>
}

impl<T: ToXML> ToXML for Response<T> {
	fn to_xml(&self) -> String {
		let mut writer = EmitterConfig::new()
			.perform_indent(false)
			.write_document_declaration(true);

		// Disable escaping to write the nested XML elements.
		writer.perform_escaping = false;

		let mut writer = writer.create_writer(Vec::new());

		writer.write(XmlEvent::start_element("clan").attr("result", &self.status.to_string())).ok();
		writer.write(XmlEvent::characters(&self.content.to_xml())).ok();
		writer.write(XmlEvent::end_element()).ok();

		let result = writer.into_inner();
		String::from_utf8(result).unwrap()
	}
}

impl<T: ToXML> Responder for Response<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let status_code = match self.status {
			Status::Ok => StatusCode::OK,
			Status::Err(code) => code.into()
		};

		let mut builder = actix_web::HttpResponse::build(status_code);
		for (key, value) in &HEADERS {
			builder.append_header((*key, *value));
		}

		builder.body::<String>(self.to_xml())
    }
}

impl<T: ToXML> Response<T> {
	/// Create a new successful response.
	pub const fn success(content: Content<T>) -> Self {
		Self {
			status: Status::Ok,
			content
		}
	}

	/// Create a new error response.
	pub const fn error(error_code: ErrorCode) -> Self {
		Self {
			status: Status::Err(error_code),
			content: Content::Empty
		}
	}
}

/// Custom type for the result of a response.
#[derive(Debug)]
pub enum Status {
	/// Success response.
	Ok,

	/// Error response containing an [`ErrorCode`](enum.ErrorCode.html).
	Err(ErrorCode)
}

impl std::fmt::Display for Status {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{:02}", match self {
			Self::Ok => SCE_NP_CLANS_SERVER_SUCCESS,
			Self::Err(code) => *code as u8
		})
	}
}

/// Success response code.
pub const SCE_NP_CLANS_SERVER_SUCCESS: u8 = 0x00;

/// Error codes for the response.
#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Serialize)]
#[repr(u8)]
pub enum ErrorCode {
	SCE_NP_CLANS_SERVER_ERROR_BAD_REQUEST                   = 0x01,
	SCE_NP_CLANS_SERVER_ERROR_INVALID_TICKET                = 0x02,
	SCE_NP_CLANS_SERVER_ERROR_INVALID_SIGNATURE             = 0x03,
	SCE_NP_CLANS_SERVER_ERROR_TICKET_EXPIRED                = 0x04,
	SCE_NP_CLANS_SERVER_ERROR_INVALID_NPID                  = 0x05,
	SCE_NP_CLANS_SERVER_ERROR_FORBIDDEN                     = 0x06,
	SCE_NP_CLANS_SERVER_ERROR_INTERNAL_SERVER_ERROR         = 0x07,
	SCE_NP_CLANS_SERVER_ERROR_BANNED                        = 0x0a,
	SCE_NP_CLANS_SERVER_ERROR_BLACKLISTED                   = 0x11,
	SCE_NP_CLANS_SERVER_ERROR_INVALID_ENVIRONMENT           = 0x1d,
	SCE_NP_CLANS_SERVER_ERROR_NO_SUCH_CLAN_SERVICE          = 0x2f,
	SCE_NP_CLANS_SERVER_ERROR_NO_SUCH_CLAN                  = 0x30,
	SCE_NP_CLANS_SERVER_ERROR_NO_SUCH_CLAN_MEMBER           = 0x31,
	SCE_NP_CLANS_SERVER_ERROR_BEFORE_HOURS                  = 0x32,
	SCE_NP_CLANS_SERVER_ERROR_CLOSED_SERVICE                = 0x33,
	SCE_NP_CLANS_SERVER_ERROR_PERMISSION_DENIED             = 0x34,
	SCE_NP_CLANS_SERVER_ERROR_CLAN_LIMIT_REACHED            = 0x35,
	SCE_NP_CLANS_SERVER_ERROR_CLAN_LEADER_LIMIT_REACHED     = 0x36,
	SCE_NP_CLANS_SERVER_ERROR_CLAN_MEMBER_LIMIT_REACHED     = 0x37,
	SCE_NP_CLANS_SERVER_ERROR_CLAN_JOINED_LIMIT_REACHED     = 0x38,
	SCE_NP_CLANS_SERVER_ERROR_MEMBER_STATUS_INVALID         = 0x39,
	SCE_NP_CLANS_SERVER_ERROR_DUPLICATED_CLAN_NAME          = 0x3a,
	SCE_NP_CLANS_SERVER_ERROR_CLAN_LEADER_CANNOT_LEAVE      = 0x3b,
	SCE_NP_CLANS_SERVER_ERROR_INVALID_ROLE_PRIORITY         = 0x3c,
	SCE_NP_CLANS_SERVER_ERROR_ANNOUNCEMENT_LIMIT_REACHED    = 0x3d,
	SCE_NP_CLANS_SERVER_ERROR_CLAN_CONFIG_MASTER_NOT_FOUND  = 0x3e,
	SCE_NP_CLANS_SERVER_ERROR_DUPLICATED_CLAN_TAG           = 0x3f,
	SCE_NP_CLANS_SERVER_ERROR_EXCEEDS_CREATE_CLAN_FREQUENCY = 0x40,
	SCE_NP_CLANS_SERVER_ERROR_CLAN_PASSPHRASE_INCORRECT     = 0x41,
	SCE_NP_CLANS_SERVER_ERROR_CANNOT_RECORD_BLACKLIST_ENTRY = 0x42,
	SCE_NP_CLANS_SERVER_ERROR_NO_SUCH_CLAN_ANNOUNCEMENT     = 0x43,
	SCE_NP_CLANS_SERVER_ERROR_VULGAR_WORDS_POSTED           = 0x44,
	SCE_NP_CLANS_SERVER_ERROR_BLACKLIST_LIMIT_REACHED       = 0x45,
	SCE_NP_CLANS_SERVER_ERROR_NO_SUCH_BLACKLIST_ENTRY       = 0x46,
	SCE_NP_CLANS_SERVER_ERROR_INVALID_NP_MESSAGE_FORMAT     = 0x4b,
	SCE_NP_CLANS_SERVER_ERROR_FAILED_TO_SEND_NP_MESSAGE     = 0x4c,
}

#[allow(clippy::from_over_into)]
#[allow(clippy::match_same_arms)]
impl Into<StatusCode> for ErrorCode {
	fn into(self) -> StatusCode {
		match self {
			Self::SCE_NP_CLANS_SERVER_ERROR_BAD_REQUEST                   => StatusCode::BAD_REQUEST,
			Self::SCE_NP_CLANS_SERVER_ERROR_INVALID_TICKET                => StatusCode::UNAUTHORIZED,
			Self::SCE_NP_CLANS_SERVER_ERROR_INVALID_SIGNATURE             => StatusCode::UNAUTHORIZED,
			Self::SCE_NP_CLANS_SERVER_ERROR_TICKET_EXPIRED                => StatusCode::UNAUTHORIZED,
			Self::SCE_NP_CLANS_SERVER_ERROR_INVALID_NPID                  => StatusCode::BAD_REQUEST,
			Self::SCE_NP_CLANS_SERVER_ERROR_FORBIDDEN                     => StatusCode::FORBIDDEN,
			Self::SCE_NP_CLANS_SERVER_ERROR_INTERNAL_SERVER_ERROR         => StatusCode::INTERNAL_SERVER_ERROR,
			Self::SCE_NP_CLANS_SERVER_ERROR_BANNED                        => StatusCode::FORBIDDEN,
			Self::SCE_NP_CLANS_SERVER_ERROR_BLACKLISTED                   => StatusCode::FORBIDDEN,
			Self::SCE_NP_CLANS_SERVER_ERROR_INVALID_ENVIRONMENT           => StatusCode::INTERNAL_SERVER_ERROR,
			Self::SCE_NP_CLANS_SERVER_ERROR_NO_SUCH_CLAN_SERVICE          => StatusCode::NOT_FOUND,
			Self::SCE_NP_CLANS_SERVER_ERROR_NO_SUCH_CLAN                  => StatusCode::NOT_FOUND,
			Self::SCE_NP_CLANS_SERVER_ERROR_NO_SUCH_CLAN_MEMBER           => StatusCode::NOT_FOUND,
			Self::SCE_NP_CLANS_SERVER_ERROR_BEFORE_HOURS                  => StatusCode::FORBIDDEN,
			Self::SCE_NP_CLANS_SERVER_ERROR_CLOSED_SERVICE                => StatusCode::FORBIDDEN,
			Self::SCE_NP_CLANS_SERVER_ERROR_PERMISSION_DENIED             => StatusCode::FORBIDDEN,
			Self::SCE_NP_CLANS_SERVER_ERROR_CLAN_LIMIT_REACHED            => StatusCode::FORBIDDEN,
			Self::SCE_NP_CLANS_SERVER_ERROR_CLAN_LEADER_LIMIT_REACHED     => StatusCode::FORBIDDEN,
			Self::SCE_NP_CLANS_SERVER_ERROR_CLAN_MEMBER_LIMIT_REACHED     => StatusCode::FORBIDDEN,
			Self::SCE_NP_CLANS_SERVER_ERROR_CLAN_JOINED_LIMIT_REACHED     => StatusCode::FORBIDDEN,
			Self::SCE_NP_CLANS_SERVER_ERROR_MEMBER_STATUS_INVALID         => StatusCode::BAD_REQUEST,
			Self::SCE_NP_CLANS_SERVER_ERROR_DUPLICATED_CLAN_NAME          => StatusCode::CONFLICT,
			Self::SCE_NP_CLANS_SERVER_ERROR_CLAN_LEADER_CANNOT_LEAVE      => StatusCode::FORBIDDEN,
			Self::SCE_NP_CLANS_SERVER_ERROR_INVALID_ROLE_PRIORITY         => StatusCode::BAD_REQUEST,
			Self::SCE_NP_CLANS_SERVER_ERROR_ANNOUNCEMENT_LIMIT_REACHED    => StatusCode::FORBIDDEN,
			Self::SCE_NP_CLANS_SERVER_ERROR_CLAN_CONFIG_MASTER_NOT_FOUND  => StatusCode::NOT_FOUND,
			Self::SCE_NP_CLANS_SERVER_ERROR_DUPLICATED_CLAN_TAG           => StatusCode::CONFLICT,
			Self::SCE_NP_CLANS_SERVER_ERROR_EXCEEDS_CREATE_CLAN_FREQUENCY => StatusCode::FORBIDDEN,
			Self::SCE_NP_CLANS_SERVER_ERROR_CLAN_PASSPHRASE_INCORRECT     => StatusCode::FORBIDDEN,
			Self::SCE_NP_CLANS_SERVER_ERROR_CANNOT_RECORD_BLACKLIST_ENTRY => StatusCode::FORBIDDEN,
			Self::SCE_NP_CLANS_SERVER_ERROR_NO_SUCH_CLAN_ANNOUNCEMENT     => StatusCode::NOT_FOUND,
			Self::SCE_NP_CLANS_SERVER_ERROR_VULGAR_WORDS_POSTED           => StatusCode::FORBIDDEN,
			Self::SCE_NP_CLANS_SERVER_ERROR_BLACKLIST_LIMIT_REACHED       => StatusCode::FORBIDDEN,
			Self::SCE_NP_CLANS_SERVER_ERROR_NO_SUCH_BLACKLIST_ENTRY       => StatusCode::NOT_FOUND,
			Self::SCE_NP_CLANS_SERVER_ERROR_INVALID_NP_MESSAGE_FORMAT     => StatusCode::BAD_REQUEST,
			Self::SCE_NP_CLANS_SERVER_ERROR_FAILED_TO_SEND_NP_MESSAGE     => StatusCode::INTERNAL_SERVER_ERROR,
		}
	}
}

/// An XML abstraction for the content of a response.
#[derive(Debug)]
pub enum Content<T: ToXML> {
	/// A single item.
	Item(T),

	/// A list of items.
	List(List<T>),

	/// Nothing.
	Empty
}

impl ToXML for () {
	fn to_xml(&self) -> String {
		String::new()
	}
}

impl<T: ToXML> ToXML for Content<T> {
	fn to_xml(&self) -> String {
		match self {
			Self::Item(item) => item.to_xml(),
			Self::List(list) => list.to_xml(),
			Self::Empty => String::new()
		}
	}
}

/// An XML abstraction for a generic list of items.
/// 
/// ```xml
/// <list results="{results}" total="{total}">
///     ...
/// </list>
/// ```
#[derive(Debug)]
pub struct List<T: ToXML> {
    /// Number of items in the current response.
    pub results: u32,

    /// Total number of items existing in the server.
    pub total: u32,

    /// List of items.
    pub items: Vec<T>
}

impl<T: ToXML> ToXML for List<T> {
	fn to_xml(&self) -> String {
		let mut writer = EmitterConfig::new()
			.perform_indent(false)
			.write_document_declaration(false);

		// Disable escaping to write the nested XML elements.
		writer.perform_escaping = false;

		let mut writer = writer.create_writer(Vec::new());

		let results = self.results.to_string();
		let total = self.total.to_string();

		let element = XmlEvent::start_element("list")
			.attr("results", &results)
			.attr("total", &total);
		writer.write(element).ok();

		for item in &self.items {
			writer.write(XmlEvent::characters(&item.to_xml())).ok();
		}

		writer.write(XmlEvent::end_element()).ok();

		let result = writer.into_inner();
		String::from_utf8(result).unwrap()
	}
}