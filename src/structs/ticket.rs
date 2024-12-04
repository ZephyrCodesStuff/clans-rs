//! A ``PlayStation Network`` ticket for authenticating requests.
//! 
//! The ticket is cryptographically signed and contains the user's
//! username, alongside other data, which we can use to identify them.


use std::fs;

use base64::Engine;
use openssl::{ec::EcKey, hash::MessageDigest, pkey::PKey, sign::Verifier};
use serde::{Deserialize, Deserializer};

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
            // PS3 uses SHA-1, and supposedly SHA-256 for V4.
            Signature::Console(_) => match self {
                Self::V2 | Self::V2_1 | Self::V3 => 16,
                Self::V4 => 32,
            },

            // The emulator uses SHA-224 for V2 and V3 (without implementing V4), meaning the signature is 28 bytes long.
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
    pub fn data(&self) -> &[u8] {
        match self {
            Self::Console(data) | Self::Emulator(data) => data,
        }
    }

    /// Length of the data to verify the signature against.
    pub fn data_length(&self, ticket_version: Version) -> usize {
        match self {
            Self::Console(_) => ticket_version.ticket_length() - (ticket_version.signature_length(self) + 16),

            // Emulator skips the first 8 bytes... for whatever reason.
            // Also, the emulator uses SHA-224 for V2 and V3 (without implementing V4), meaning the signature is 28 bytes long.
            Self::Emulator(_) => unimplemented!(),
        }
    }

    /// Deserialize a signature from a byte slice.
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

    /// The ticket's signature
    pub signature: Signature
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
        let version = Version::from_u16(version)
            .ok_or("Unsupported version")?;

        if bytes.len() < 212 || bytes.len() > 400 {
            return Err("Invalid buffer length");
        }

        match version {
            Version::V2 | Version::V2_1 | Version::V3 => {
                ticket.account_id = Self::decode_u64(&bytes[0x48..0x54]);
                ticket.username = Self::decode_string(&bytes[0x54..0x74]);
                ticket.region = Self::decode_string(&bytes[0x78..0x7A]);
                ticket.domain = Self::decode_string(&bytes[0x80..0x82]);
                ticket.service_id = Self::decode_string(&bytes[0x88..0x9B]);

                let signature_id: &[u8; 4] = &bytes[0xB8..0xBC].try_into().unwrap();
                let mut data: Vec<u8> = Vec::new();

                let signature = Signature::from_bytes(*signature_id, &Vec::new());

                match signature {
                    Signature::Console(_) => {
                        let data_length = signature.data_length(version);
                        data.extend_from_slice(&bytes[..data_length]);
                    },
                    Signature::Emulator(_) => data.extend_from_slice(&bytes[0x08..0xB0]),
                }

                ticket.signature = Signature::from_bytes(*signature_id, &data);
            },

            Version::V4 => {
                ticket.account_id = Self::decode_u64(&bytes[0x4C..0x58]);
                ticket.username = Self::decode_string(&bytes[0x58..0x78]);
                ticket.region = Self::decode_string(&bytes[0x7C..0x7E]);
                ticket.domain = Self::decode_string(&bytes[0x84..0x86]);
                ticket.service_id = Self::decode_string(&bytes[0x8C..0x9F]);

                let signature_id: &[u8; 4] = &bytes[0xC0..0xC4].try_into().unwrap();
                let mut data: Vec<u8> = Vec::new();

                let signature = Signature::from_bytes(*signature_id, &Vec::new());

                match signature {
                    Signature::Console(_) => {
                        let data_length = signature.data_length(version);
                        data.extend_from_slice(&bytes[..data_length]);
                    },
                    Signature::Emulator(_) => return Err("Ticket version 4 is not supported in the emulator"),
                }

                ticket.signature = Signature::from_bytes(*signature_id, &data);
            }
        };

        // Load the public key for the signature.
        let public_key_path = format!("keys/{}.pem", match ticket.signature {
            Signature::Console(_) => "psn",
            Signature::Emulator(_) => "rpcn",
        });

        let public_key_data = fs::read(public_key_path)
            .map_err(|_| "Failed to read public key")?;

        let ec_key = EcKey::public_key_from_pem(&public_key_data)
            .map_err(|_| "Failed to load public key")?;

        let keypair = PKey::from_ec_key(ec_key)
            .map_err(|_| "Failed to load public key")?;

        // Verify the signature.
        let digest = match ticket.signature {
            Signature::Console(_) => match version {
                Version::V2 | Version::V2_1 | Version::V3 => MessageDigest::sha1(),
                Version::V4 => MessageDigest::sha256(),
            },

            Signature::Emulator(_) => match version {
                Version::V2 | Version::V2_1 | Version::V3 => MessageDigest::sha224(),
                Version::V4 => return Err("Ticket version 4 is not supported in the emulator"),
            }
        };

        let data = ticket.signature.data();
        let signature = match ticket.signature {
            Signature::Console(_) => &bytes[bytes.len() - version.signature_length(&ticket.signature)..],
            Signature::Emulator(_) => &bytes[0xC0..],
        };

        let mut verifier = Verifier::new(digest, &keypair)
            .map_err(|_| "Failed to create verifier")?;

        verifier.update(data)
            .map_err(|_| "Failed to update verifier")?;

        // TODO:    The PSN public key has probably changed, and now we need to verify V4.
        //          Data and signature are correct, but the verification fails.
        
        if let Signature::Emulator(_) = ticket.signature {
            verifier.verify(signature)
                .map_err(|_| "Failed to verify signature")?;
        }

        Ok(ticket)
    }
}