//! A ``PlayStation Network`` ticket for authenticating requests.
//!
//! The ticket is cryptographically signed and contains the user's
//! username, alongside other data, which we can use to identify them.

use base64::Engine;
use openssl::{ec::EcKey, hash::MessageDigest, pkey::PKey, sign::Verifier};
use serde::{Deserialize, Deserializer};

/// Default domain RPCN sets for players.
pub const DEFAULT_DOMAIN: &str = "un";

/// Default region RPCN sets for players.
pub const DEFAULT_REGION: &str = "br";

/// The version of the ticket format.
///
/// It's either:
/// - 0x2100 for Version 2.0
/// - 0x2101 for Version 2.1
/// - 0x3100 for Version 3.0
/// - 0x4100 for Version 4.0
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    /// Version 2.0
    V2 = 0x2100,

    /// Version 2.1
    V2_1 = 0x2101,

    /// Version 3.0
    V3 = 0x3100,

    /// Version 4.0
    V4 = 0x4100,
}

impl Version {
    /// Get the version from a u16.
    pub const fn from_u16(version: u16) -> Option<Self> {
        match version {
            0x2100 => Some(Self::V2),
            0x2101 => Some(Self::V2_1),
            0x3100 => Some(Self::V3),
            0x4100 => Some(Self::V4),
            _ => None,
        }
    }

    /// Get the expected length of the ticket for this version.
    pub const fn ticket_length(self) -> usize {
        match self {
            Self::V2 | Self::V2_1 => 212,
            Self::V3 => 220,
            Self::V4 => 320,
        }
    }

    /// Length of the signature.
    pub fn signature_length(self, signature: &Signature) -> usize {
        match signature {
            // PS3 uses SHA-1 for V2 to V3, and SHA-256 for V4.
            Signature::Console(_) => match self {
                Self::V2 | Self::V2_1 | Self::V3 => 16,
                Self::V4 => 32,
            },

            Signature::Emulator(_) => unimplemented!(),
        }
    }
}

/// The signature ID of the ticket.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Signature {
    /// ``PlayStation Network``, real PS3
    Console(Vec<u8>),

    /// ``RPCN``, RPCS3 emulator
    Emulator(Vec<u8>),
}

impl Default for Signature {
    fn default() -> Self {
        Self::Emulator(Vec::new())
    }
}

impl Signature {
    /// Get the data.
    pub fn signed_data(&self) -> &[u8] {
        match self {
            Self::Console(data) | Self::Emulator(data) => data,
        }
    }

    /// Length of the data to verify the signature against.
    pub fn signed_data_length(&self, ticket_version: Version) -> usize {
        match self {
            Self::Console(_) => {
                ticket_version.ticket_length() - (ticket_version.signature_length(self) + 16)
            }

            // The emulator only signs from 0x08 to 0xB0, skipping the first 8 bytes.
            // This is the entirety of the `user_data` section.
            Self::Emulator(_) => unimplemented!(),
        }
    }

    /// Deserialize a signature from a byte slice.
    ///
    /// `RPCN` signatures have the ID `RPCN`.
    /// All other signatures are considered `PSN` signatures.
    pub fn from_bytes(id: [u8; 4], data: &[u8]) -> Self {
        match &id {
            b"RPCN" => Self::Emulator(data.to_vec()),
            _ => Self::Console(data.to_vec()),
        }
    }
}

/// A ``PlayStation Network`` ticket for authenticating requests.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Ticket {
    /// The ticket's serial number.
    pub serial: String,

    /// The issuer's ID.
    pub issuer_id: u32,

    /// The issued date as a UNIX timestamp.
    pub issued_at: u64,

    /// The expiration date as a UNIX timestamp.
    pub expires_at: u64,

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

    /// Status of the ticket (seems to be always 0)
    pub status: u32,

    /// The ticket's signature
    pub signature: Signature,
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
        let mut decoded = engine.decode(base64).map_err(serde::de::Error::custom)?;

        // Deserialize the ticket from the decoded bytes.
        let ticket = Self::from_bytes(&mut decoded).map_err(serde::de::Error::custom)?;

        Ok(ticket)
    }
}

impl Ticket {
    /// Decode a string from a byte slice.
    fn decode_string(bytes: &[u8]) -> String {
        String::from_utf8_lossy(bytes)
            .trim_end_matches('\0')
            .to_string()
    }

    /// Make sure `issued_at` and `expires_at` make sense.
    ///
    /// - `issued_at` and `expires_at` must be non-zero.
    /// - `expires_at` must be after `issued_at`.
    /// - `issued_at` must not be in the future (with a 5 minute leeway).
    /// - `issued_at` and `expires_at` must not be more than 1 year in the future.
    ///
    /// The dates are assumed to be in milliseconds.
    fn validate_dates(issued_at: u64, expires_at: u64) -> Result<(), &'static str> {
        if issued_at == 0 || expires_at == 0 {
            return Err("Invalid issued or expiration date");
        }

        if expires_at <= issued_at {
            return Err("Expiration date is before issued date");
        }

        // Check if issued_at is in the future (with a 5 minute leeway)
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| "System time before UNIX EPOCH")?
            .as_secs();

        if issued_at > now * 1000 + 300 * 1000 {
            return Err("Issued date is in the future");
        }

        // Check if expires_at is too far in the future (more than 1 year)
        if expires_at > now * 1000 + 31_536_000 * 1000 {
            return Err("Issued or expiration date is too far in the future");
        }

        Ok(())
    }

    /// Deserialize a ticket from a byte slice.
    /// This will also verify the ticket's signature.
    ///
    /// `bytes` must be mutable, as the function may modify it
    /// to fix endianness issues with the timestamps.
    #[allow(clippy::too_many_lines)]
    pub fn from_bytes(bytes: &mut [u8]) -> Result<Self, &'static str> {
        let mut ticket = Self::default();

        if bytes.is_empty() {
            return Err("Empty buffer");
        }

        let version = u16::from_be_bytes([bytes[0], bytes[1]]);
        let version = Version::from_u16(version).ok_or("Unsupported version")?;

        if bytes.len() < 212 || bytes.len() > 400 {
            return Err("Invalid buffer length");
        }

        // Helper function to handle endianness issues with timestamps
        let parse_timestamps = |bytes: &mut [u8],
                                issued_range: std::ops::Range<usize>,
                                expires_range: std::ops::Range<usize>|
         -> Result<(u64, u64), &'static str> {
            let mut issued_at = u64::from_be_bytes(bytes[issued_range.clone()].try_into().unwrap());
            let mut expires_at =
                u64::from_be_bytes(bytes[expires_range.clone()].try_into().unwrap());

            // If validation fails, try swapping endianness
            if Self::validate_dates(issued_at, expires_at).is_err() {
                bytes[issued_range.clone()].reverse();
                bytes[expires_range.clone()].reverse();

                issued_at = u64::from_be_bytes(bytes[issued_range].try_into().unwrap());
                expires_at = u64::from_be_bytes(bytes[expires_range].try_into().unwrap());

                Self::validate_dates(issued_at, expires_at)?;
            }

            Ok((issued_at, expires_at))
        };

        match version {
            // Both platforms may send these versions.
            Version::V2 | Version::V2_1 | Version::V3 => {
                ticket.serial = Self::decode_string(&bytes[0x10..0x24]);
                ticket.issuer_id = u32::from_be_bytes(bytes[0x28..0x2C].try_into().unwrap());

                (ticket.issued_at, ticket.expires_at) =
                    parse_timestamps(bytes, 0x30..0x38, 0x3C..0x44)?;

                ticket.account_id = u64::from_be_bytes(bytes[0x48..0x50].try_into().unwrap());
                ticket.username = Self::decode_string(&bytes[0x54..0x74]);
                ticket.region = Self::decode_string(&bytes[0x78..0x7A]);
                ticket.domain = Self::decode_string(&bytes[0x80..0x82]);
                ticket.service_id = Self::decode_string(&bytes[0x88..0x9B]);
                ticket.status = u32::from_be_bytes(bytes[0xA4..0xA8].try_into().unwrap());

                // If empty, default to RPCN defaults: `un` and `br`
                if ticket.domain.is_empty() {
                    ticket.domain = DEFAULT_DOMAIN.to_string();
                }
                if ticket.region.is_empty() {
                    ticket.region = DEFAULT_REGION.to_string();
                }

                let signature_id: &[u8; 4] = &bytes[0xB8..0xBC].try_into().unwrap();
                let signature = Signature::from_bytes(*signature_id, &Vec::new());

                let signed_data = match signature {
                    Signature::Console(_) => {
                        let data_length = signature.signed_data_length(version);
                        bytes[0x08..data_length].to_vec()
                    }
                    Signature::Emulator(_) => bytes[0x08..0xB0].to_vec(),
                };

                ticket.signature = Signature::from_bytes(*signature_id, &signed_data);
            }

            // Only the console uses version 4 tickets. The emulator does not support them.
            Version::V4 => {
                ticket.serial = Self::decode_string(&bytes[0x14..0x28]);
                ticket.issuer_id = u32::from_be_bytes(bytes[0x2C..0x30].try_into().unwrap());

                (ticket.issued_at, ticket.expires_at) =
                    parse_timestamps(bytes, 0x34..0x3C, 0x40..0x48)?;

                ticket.account_id = u64::from_be_bytes(bytes[0x4C..0x54].try_into().unwrap());
                ticket.username = Self::decode_string(&bytes[0x58..0x78]);
                ticket.region = Self::decode_string(&bytes[0x7C..0x7E]);
                ticket.domain = Self::decode_string(&bytes[0x84..0x86]);
                ticket.service_id = Self::decode_string(&bytes[0x8C..0x9F]);

                let signature_id: &[u8; 4] = &bytes[0xC0..0xC4].try_into().unwrap();
                let signature = Signature::from_bytes(*signature_id, &Vec::new());

                let signed_data = match signature {
                    Signature::Console(_) => {
                        let start = 0x08;
                        let end = bytes.len() - version.signature_length(&signature) - 16;
                        println!("V4 signed data range: {start:#X}..{end:#X}");
                        bytes[start..end].to_vec()
                    }
                    Signature::Emulator(_) => {
                        unimplemented!("Emulator does not support version 4 tickets")
                    }
                };
                ticket.signature = Signature::from_bytes(*signature_id, &signed_data);
            }
        }

        let ec_key_name = match &ticket.signature {
            Signature::Console(_) => "psn",
            Signature::Emulator(_) => "rpcn",
        };

        let ec_key_bytes = std::fs::read_to_string(format!("keys/{ec_key_name}.pem"))
            .map_err(|_| "Failed to read public key")?;

        let ec_key = EcKey::public_key_from_pem(ec_key_bytes.as_bytes())
            .map_err(|_| "Failed to load public key")?;

        let keypair = PKey::from_ec_key(ec_key).map_err(|_| "Failed to load public key")?;

        // Verify the signature.
        let digest = match ticket.signature {
            Signature::Console(_) => match version {
                Version::V2 | Version::V2_1 | Version::V3 => MessageDigest::sha1(),
                Version::V4 => MessageDigest::sha256(),
            },

            Signature::Emulator(_) => match version {
                Version::V2 | Version::V2_1 | Version::V3 => MessageDigest::sha224(),
                Version::V4 => return Err("Ticket version 4 is not supported in the emulator"),
            },
        };

        let data = ticket.signature.signed_data();
        let signature = match ticket.signature {
            Signature::Console(_) => {
                &bytes[bytes.len() - version.signature_length(&ticket.signature)..]
            }
            Signature::Emulator(_) => &bytes[0xC0..],
        };

        let mut verifier =
            Verifier::new(digest, &keypair).map_err(|_| "Failed to create verifier")?;

        verifier
            .update(data)
            .map_err(|_| "Failed to update verifier")?;

        // # Verifying the signature of NP tickets
        //
        // For the time being, only RPCN signatures can be verified.
        //
        // This is because:
        // - The `psn.pem` public key might be wrong
        //    - It claims to be prime192v1, but the signature doesn't match that curve
        // - The signature is only 32 bytes, which only matches secp128r1
        //    - The point is not on that curve, so it can't be secp128r1
        //    - This is also why verifying PSN will not only give `false`; it will error out
        // - Every PSN game has a different private key for signing NP tickets, and we don't
        //   know PlayStation Home's public key.
        // - Nobody has successfully verified a Version 4 PSN ticket yet
        // - Versions below 4 aren't sent anymore by PSN.
        if let Signature::Emulator(_) = ticket.signature {
            let result = verifier
                .verify(signature)
                .map_err(|_| "Failed to verify signature!")?;

            if !result {
                return Err("Invalid signature");
            }
        }

        Ok(ticket)
    }
}
