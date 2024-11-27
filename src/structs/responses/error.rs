//! Error response structures.

use serde::Serialize;

/// Success response code.
pub const SUCCESS: u8 = 0x00;

/// Error codes for the Clans operations.
#[allow(dead_code, clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, Serialize)]
#[repr(u8)]
pub enum ErrorCode {
    /// The request was malformed or invalid.
    BadRequest = 0x01,

    /// The provided ticket is invalid.
    InvalidTicket = 0x02,

    /// The provided signature is invalid.
    InvalidSignature = 0x03,

    /// The ticket has expired.
    TicketExpired = 0x04,

    /// The specified NP ID is invalid.
    InvalidNpId = 0x05,

    /// The operation is forbidden.
    /// 
    /// This is mostly for API endpoints a user should not access.
    /// For general denial, use [`PermissionDenied`](#variant.PermissionDenied) instead.
    Forbidden = 0x06,

    /// An internal server error occurred.
    InternalServerError = 0x07,

    /// The user is banned from the Clans service.
    /// 
    /// Currently there is nothing that would ban a user.
    Banned = 0x0A,

    /// The user is blacklisted from the clan.
    Blacklisted = 0x11,

    /// The specified environment is invalid.
    /// 
    /// This error's use is unknown.
    InvalidEnvironment = 0x1D,

    /// The clan service does not exist.
    /// 
    /// Used for the 404 Not Found handler.
    NoSuchClanService = 0x2F,

    /// The specified clan does not exist.
    NoSuchClan = 0x30,

    /// The specified clan member does not exist.
    NoSuchClanMember = 0x31,

    /// Operation attempted outside permitted hours.
    /// 
    /// Currently there isn't a defined time frame for operations.
    BeforeHours = 0x32,

    /// The service is currently closed.
    /// 
    /// This is likely for maintenance.
    ClosedService = 0x33,

    /// Permission to perform this operation is denied.
    /// 
    /// This is the preferred error for denying a user's request.
    PermissionDenied = 0x34,

    /// The clan limit has been reached, **globally**.
    /// 
    /// This usually indicates that we're reached ``999_999`` clans.
    ClanLimitReached = 0x35,

    /// The clan leader limit has been reached.
    ClanLeaderLimitReached = 0x36,

    /// The clan member limit has been reached.
    ClanMemberLimitReached = 0x37,

    /// The limit for clans joined by the user has been reached.
    ClanJoinedLimitReached = 0x38,

    /// The specified member status is invalid.
    /// 
    /// This is used when an operation is attempted on a member with
    /// a status that doesn't allow it.
    MemberStatusInvalid = 0x39,

    /// The clan name is already in use.
    DuplicatedClanName = 0x3A,

    /// The clan leader cannot leave the clan.
    /// 
    /// This should never be possible, except for manual API calls.
    ClanLeaderCannotLeave = 0x3B,

    /// The role priority specified is invalid.
    InvalidRolePriority = 0x3C,

    /// The limit for clan announcements has been reached.
    AnnouncementLimitReached = 0x3D,

    /// The clan configuration master was not found.
    ClanConfigMasterNotFound = 0x3E,

    /// The clan tag is already in use.
    DuplicatedClanTag = 0x3F,

    /// Too many clans created within the allowed frequency.
    /// 
    /// This is used for rate limiting.
    ExceedsCreateClanFrequency = 0x40,

    /// The clan passphrase provided is incorrect.
    ClanPassphraseIncorrect = 0x41,

    /// A blacklist entry could not be recorded.
    CannotRecordBlacklistEntry = 0x42,

    /// The specified clan announcement does not exist.
    NoSuchClanAnnouncement = 0x43,

    /// The post contains vulgar or offensive words.
    /// 
    /// Currently, there's nothing stopping vulgar words,
    /// so this error is unused.
    VulgarWordsPosted = 0x44,

    /// The blacklist limit has been reached for the clan.
    BlacklistLimitReached = 0x45,

    /// The specified blacklist entry does not exist.
    NoSuchBlacklistEntry = 0x46,

    /// The NP message format is invalid.
    /// 
    /// This can be used when the Ticket's parsing fails.
    InvalidNpMessageFormat = 0x4B,

    /// Failed to send the NP message.
    /// 
    /// This is never triggered by us, so it's currently unused.
    FailedToSendNpMessage = 0x4C,
}