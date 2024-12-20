//! Base XML structs for the responses of the API,
//! wrapping the XML entities inside.

use std::fmt::Debug;

use actix_web::Responder;
use xml::{writer::XmlEvent, EmitterConfig};

use crate::{structs::responses::error::SUCCESS, utils::xml_format::ToXML};

use super::error::ErrorCode;

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

impl<T: ToXML + Debug> Responder for Response<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
		let mut builder = actix_web::HttpResponse::Ok();
		for (key, value) in &HEADERS {
			builder.append_header((*key, *value));
		}

		// DEBUG: print the XML's contents
		log::debug!("Response: {:#?}", self);

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
		write!(f, "{:02X}", match self {
			Self::Ok => SUCCESS,
			Self::Err(code) => *code as u8
		})
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