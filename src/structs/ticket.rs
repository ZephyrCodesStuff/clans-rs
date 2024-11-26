//! A ``PlayStation Network`` ticket for authenticating requests.
//! 
//! The ticket is cryptographically signed and contains the user's
//! username, alongside other data, which we can use to identify them.


use actix_web::{web::Buf, FromRequest};
use base64::Engine;
use serde::{Deserialize, Deserializer};

/// Version 4 of the ticket format.
/// 
/// NOTE: this is **not** supported by the RPCS3 emulator yet,
/// meaning that the ticket will only ever come from a real PS3.
const TICKET_VERSION: u16 = 0x4100;

/// A ``PlayStation Network`` ticket for authenticating requests.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Ticket {
    /// The issuer's ID.
    pub issuer_id: u32,

    /// The account ID of the user.
    pub account_id: u64,

    /// The username of the user.
    pub username: String,

    /// The region the user is in.
    pub region: String,

    /// The domain the user is in.
    pub domain: String,

    /// The service ID the ticket was issued for.
    pub service_id: String,
}

impl<'de> Deserialize<'de> for Ticket {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize from a base64 string.
        let base64 = String::deserialize(deserializer)?;
        
        // Decode the base64 string.
        let engine = base64::engine::general_purpose::STANDARD;
        let decoded = engine.decode(base64)
            .map_err(serde::de::Error::custom)?;

        // Deserialize the ticket from the decoded bytes.
        let ticket = Self::from_bytes(&decoded)
            .map_err(serde::de::Error::custom)?;

        Ok(ticket)
    }
}

/// Extractor helper, to parse the XML body of the request as such:
/// ```xml
/// <clan>
///     <id>{id}</id>
/// </clan>
/// ```
#[derive(Debug, Deserialize)]
pub struct TicketOnly {
    /// The ID of the clan.
    pub ticket: Ticket,
}

impl FromRequest for Ticket {
    type Error = actix_web::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>> + 'static>>;

    /// Get the ticket from the request body.
    fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        let fut = actix_web::web::Bytes::from_request(req, payload);
        Box::pin(async move {
            let bytes = fut.await?;

            let request: TicketOnly = serde_xml_rs::from_reader(bytes.reader())
                .map_err(actix_web::error::ErrorInternalServerError)?;

            Ok(request.ticket)
        })
    }
}

impl Ticket {
    /// Decode a big-endian u64 from a byte slice.
    const fn decode_u64(bytes: &[u8]) -> u64 {
        u64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
        ])
    }

    /// Decode a string from a byte slice.
    fn decode_string(bytes: &[u8]) -> String {
        String::from_utf8_lossy(bytes)
            .trim_end_matches('\0')
            .to_string()
    }

    /// Deserialize a ticket from a byte slice.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        let mut ticket = Self::default();

        if bytes.is_empty() {
            return Err("Empty buffer");
        }

        let version = u16::from_be_bytes([bytes[0], bytes[1]]);
        if version != TICKET_VERSION {
            return Err("Unsupported version");
        }

        ticket.account_id = Self::decode_u64(&bytes[0x4C..0x58]);
        ticket.username = Self::decode_string(&bytes[0x58..0x78]);
        ticket.region = Self::decode_string(&bytes[0x7C..0x7E]);
        ticket.domain = Self::decode_string(&bytes[0x84..0x88]);
        ticket.service_id = Self::decode_string(&bytes[0x8C..0x9F]);

        Ok(ticket)
    }
}