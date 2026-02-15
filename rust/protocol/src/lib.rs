use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use thiserror::Error;

pub const CODE_SM_LOGIN: u32 = 1;
pub const CODE_SM_SET_WAIT_PORT: u32 = 2;
pub const CODE_SM_GET_PEER_ADDRESS: u32 = 3;
pub const CODE_SM_ADD_USER: u32 = 5;
pub const CODE_SM_REMOVE_USER: u32 = 6;
pub const CODE_SM_GET_USER_STATUS: u32 = 7;
pub const CODE_SM_ADD_CHATROOM: u32 = 10;
pub const CODE_SM_IGNORE_USER: u32 = 11;
pub const CODE_SM_UNIGNORE_USER: u32 = 12;
pub const CODE_SM_SAY_CHATROOM: u32 = 13;
pub const CODE_SM_JOIN_ROOM: u32 = 14;
pub const CODE_SM_LEAVE_ROOM: u32 = 15;
pub const CODE_SM_USER_JOINED_ROOM: u32 = 16;
pub const CODE_SM_USER_LEFT_ROOM: u32 = 17;
pub const CODE_SM_CONNECT_TO_PEER: u32 = 18;
pub const CODE_SM_MESSAGE_USER: u32 = 22;
pub const CODE_SM_MESSAGE_ACKED: u32 = 23;
pub const CODE_SM_FILE_SEARCH: u32 = 26;
pub const CODE_SM_SET_STATUS: u32 = 28;
pub const CODE_SM_HEARTBEAT: u32 = 32;
pub const CODE_SM_SEND_CONNECT_TOKEN: u32 = 33;
pub const CODE_SM_ROOM_LIST: u32 = 64;
pub const CODE_SM_FILE_SEARCH_RESPONSE: u32 = CODE_SM_ROOM_LIST;
pub const CODE_SM_PRIVILEGED_LIST: u32 = 69;
pub const CODE_SM_DOWNLOAD_SPEED: u32 = 34;
pub const CODE_SM_SHARED_FOLDERS_FILES: u32 = 35;
pub const CODE_SM_GET_USER_STATS: u32 = 36;
pub const CODE_SM_RELOGGED: u32 = 41;
pub const CODE_SM_SEARCH_USER_FILES: u32 = 42;
pub const CODE_SM_GET_SIMILAR_TERMS: u32 = 50;
pub const CODE_SM_ADD_LIKE_TERM: u32 = 51;
pub const CODE_SM_REMOVE_LIKE_TERM: u32 = 52;
pub const CODE_SM_GET_RECOMMENDATIONS: u32 = 54;
pub const CODE_SM_GET_MY_RECOMMENDATIONS: u32 = 55;
pub const CODE_SM_GET_GLOBAL_RECOMMENDATIONS: u32 = 56;
pub const CODE_SM_GET_USER_RECOMMENDATIONS: u32 = 57;
pub const CODE_SM_COMMAND: u32 = 58;
pub const CODE_SM_PLACE_IN_LINE: u32 = 59;
pub const CODE_SM_PLACE_IN_LINE_RESPONSE: u32 = 60;
pub const CODE_SM_USER_LIST: u32 = 61;
pub const CODE_SM_ROOM_ADDED: u32 = 62;
pub const CODE_SM_ROOM_REMOVED: u32 = 63;
pub const CODE_SM_EXACT_FILE_SEARCH: u32 = 65;
pub const CODE_SM_ADMIN_MESSAGE: u32 = 66;
pub const CODE_SM_GLOBAL_USER_LIST: u32 = 67;
pub const CODE_SM_CONNECT_TO_CLIENT: u32 = 70;
pub const CODE_SM_SEND_DISTRIBUTIONS: u32 = 71;
pub const CODE_SM_NOTE_PARENT: u32 = 73;
pub const CODE_SM_CHILD_PARENT_MAP: u32 = 82;
pub const CODE_SM_SET_PARENT_MIN_SPEED: u32 = 83;
pub const CODE_SM_SET_PARENT_SPEED_CONNECTION_RATIO: u32 = 84;
pub const CODE_SM_SET_PARENT_INACTIVITY_BEFORE_DISCONNECT: u32 = 86;
pub const CODE_SM_SET_SERVER_INACTIVITY_BEFORE_DISCONNECT: u32 = 87;
pub const CODE_SM_NODES_IN_CACHE_BEFORE_DISCONNECT: u32 = 88;
pub const CODE_SM_SET_SECONDS_BEFORE_PING_CHILDREN: u32 = 90;
pub const CODE_SM_ADD_PRIVILEGED_USER: u32 = 91;
pub const CODE_SM_GET_OWN_PRIVILEGES_STATUS: u32 = 92;
pub const CODE_SM_DNET_MESSAGE: u32 = 93;
pub const CODE_SM_CAN_PARENT: u32 = 100;
pub const CODE_SM_POSSIBLE_PARENTS: u32 = 102;
pub const CODE_SM_LOW_PRIORITY_FILE_SEARCH: u32 = 103;
pub const CODE_SM_WISHLIST_WAIT: u32 = 104;
pub const CODE_SM_GET_RECOMMENDED_USERS: u32 = 110;
pub const CODE_SM_GET_TERM_RECOMMENDATIONS: u32 = 111;
pub const CODE_SM_GET_RECOMMENDATION_USERS: u32 = 112;
pub const CODE_SM_GET_ROOM_TICKER: u32 = 113;
pub const CODE_SM_ROOM_TICKER_USER_ADDED: u32 = 114;
pub const CODE_SM_ROOM_TICKER_USER_REMOVED: u32 = 115;
pub const CODE_SM_SET_TICKER: u32 = 116;
pub const CODE_SM_ADD_HATE_TERM: u32 = 117;
pub const CODE_SM_REMOVE_HATE_TERM: u32 = 118;
pub const CODE_SM_SEARCH_ROOM: u32 = 120;
pub const CODE_SM_GET_USER_PRIVILEGES_STATUS: u32 = 122;
pub const CODE_SM_GIVE_PRIVILEGE: u32 = 123;
pub const CODE_SM_INFORM_USER_OF_PRIVILEGES: u32 = 124;
pub const CODE_SM_INFORM_USER_OF_PRIVILEGES_ACK: u32 = 125;
pub const CODE_SM_DNET_LEVEL: u32 = 126;
pub const CODE_SM_DNET_GROUP_LEADER: u32 = 127;
pub const CODE_SM_DNET_DELIVERY_REPORT: u32 = 128;
pub const CODE_SM_DNET_CHILD_DEPTH: u32 = 129;
pub const CODE_SM_DNET_RESET: u32 = 130;
pub const CODE_SM_UPLOAD_SPEED: u32 = 121;
pub const CODE_SM_FLOOD: u32 = 131;
pub const CODE_SM_BAN_USER: u32 = 132;
pub const CODE_SM_ADD_ROOM_MEMBER: u32 = 134;
pub const CODE_SM_REMOVE_ROOM_MEMBER: u32 = 135;
pub const CODE_SM_REMOVE_OWN_ROOM_MEMBERSHIP: u32 = 136;
pub const CODE_SM_GIVE_UP_ROOM: u32 = 137;
pub const CODE_SM_TRANSFER_ROOM_OWNERSHIP: u32 = 138;
pub const CODE_SM_ADD_ROOM_MEMBERSHIP: u32 = 139;
pub const CODE_SM_REMOVE_ROOM_MEMBERSHIP: u32 = 140;
pub const CODE_SM_ENABLE_PRIVATE_ROOM_ADD: u32 = 141;
pub const CODE_SM_CHANGE_PASSWORD: u32 = 142;
pub const CODE_SM_ADD_ROOM_OPERATOR: u32 = 143;
pub const CODE_SM_REMOVE_ROOM_OPERATOR: u32 = 144;
pub const CODE_SM_ADD_ROOM_OPERATORSHIP: u32 = 145;
pub const CODE_SM_REMOVE_ROOM_OPERATORSHIP: u32 = 146;
pub const CODE_SM_REMOVE_OWN_ROOM_OPERATORSHIP: u32 = 147;
pub const CODE_SM_ROOM_MEMBERS: u32 = 133;
pub const CODE_SM_ROOM_OPERATORS: u32 = 148;
pub const CODE_SM_MESSAGE_USERS: u32 = 149;
pub const CODE_SM_JOIN_GLOBAL_ROOM: u32 = 150;
pub const CODE_SM_LEAVE_GLOBAL_ROOM: u32 = 151;
pub const CODE_SM_SAY_GLOBAL_ROOM: u32 = 152;
pub const CODE_SM_SEARCH_CORRELATIONS: u32 = 153;
pub const CODE_SM_PEER_MESSAGE: u32 = 68;
pub const CODE_SM_PEER_MESSAGE_ALT: u32 = 292;

pub const CODE_PM_SAY: u32 = 1;
pub const CODE_PM_GET_SHARED_FILE_LIST: u32 = 4;
pub const CODE_PM_SHARED_FILE_LIST: u32 = 5;
pub const CODE_PM_FILE_SEARCH_REQUEST: u32 = 8;
pub const CODE_PM_FILE_SEARCH_RESULT: u32 = 9;
pub const CODE_PM_INVITE_USER_TO_ROOM: u32 = 10;
pub const CODE_PM_CANCELLED_QUEUED_TRANSFER: u32 = 14;
pub const CODE_PM_USER_INFO_REQUEST: u32 = 15;
pub const CODE_PM_USER_INFO_REPLY: u32 = 16;
pub const CODE_PM_SEND_CONNECT_TOKEN: u32 = 33;
pub const CODE_PM_MOVE_DOWNLOAD_TO_TOP: u32 = 34;
pub const CODE_PM_GET_SHARED_FILES_IN_FOLDER: u32 = 36;
pub const CODE_PM_SHARED_FILES_IN_FOLDER: u32 = 37;
pub const CODE_PM_TRANSFER_REQUEST: u32 = 40;
pub const CODE_PM_TRANSFER_RESPONSE: u32 = 41;
pub const CODE_PM_PLACEHOLD_UPLOAD: u32 = 42;
pub const CODE_PM_QUEUE_UPLOAD: u32 = 43;
pub const CODE_PM_UPLOAD_PLACE_IN_LINE: u32 = 44;
pub const CODE_PM_EXACT_FILE_SEARCH_REQUEST: u32 = 47;
pub const CODE_PM_QUEUED_DOWNLOADS: u32 = 48;
pub const CODE_PM_INDIRECT_FILE_SEARCH_REQUEST: u32 = 49;
pub const CODE_PM_UPLOAD_FAILED: u32 = 46;
pub const CODE_PM_UPLOAD_DENIED: u32 = 50;
pub const CODE_PM_UPLOAD_PLACE_IN_LINE_REQUEST: u32 = 51;
pub const CODE_PM_NOTHING: u32 = 52;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Frame {
    pub code: u32,
    pub payload: Vec<u8>,
}

impl Frame {
    pub fn new(code: u32, payload: Vec<u8>) -> Self {
        Self { code, payload }
    }

    pub fn encode(&self) -> Vec<u8> {
        let body_len = 4 + self.payload.len();
        let mut out = Vec::with_capacity(4 + body_len);
        out.extend_from_slice(&(body_len as u32).to_le_bytes());
        out.extend_from_slice(&self.code.to_le_bytes());
        out.extend_from_slice(&self.payload);
        out
    }

    pub fn decode(buf: &[u8]) -> Result<Self> {
        if buf.len() < 8 {
            bail!("frame too short: {}", buf.len());
        }

        let declared = u32::from_le_bytes(buf[0..4].try_into().context("frame length")?) as usize;
        if declared + 4 != buf.len() {
            bail!(
                "frame length mismatch: declared={} actual={}",
                declared,
                buf.len() - 4
            );
        }

        let code = u32::from_le_bytes(buf[4..8].try_into().context("message code")?);
        Ok(Self {
            code,
            payload: buf[8..].to_vec(),
        })
    }
}

#[derive(Debug, Error)]
pub enum DecoderError {
    #[error("need at least {needed} bytes, have {have}")]
    NotEnough { needed: usize, have: usize },
}

#[derive(Debug, Clone)]
pub struct PayloadWriter {
    inner: Vec<u8>,
}

impl Default for PayloadWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl PayloadWriter {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn write_u32(&mut self, value: u32) {
        self.inner.extend_from_slice(&value.to_le_bytes());
    }

    pub fn write_u8(&mut self, value: u8) {
        self.inner.push(value);
    }

    pub fn write_u64(&mut self, value: u64) {
        self.inner.extend_from_slice(&value.to_le_bytes());
    }

    pub fn write_bool_u32(&mut self, value: bool) {
        self.write_u32(if value { 1 } else { 0 });
    }

    pub fn write_bytes(&mut self, value: &[u8]) {
        self.write_u32(value.len() as u32);
        self.inner.extend_from_slice(value);
    }

    pub fn write_raw_bytes(&mut self, value: &[u8]) {
        self.inner.extend_from_slice(value);
    }

    pub fn write_string(&mut self, value: &str) {
        self.write_u32(value.len() as u32);
        self.inner.extend_from_slice(value.as_bytes());
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.inner
    }
}

#[derive(Debug, Clone)]
pub struct PayloadReader<'a> {
    payload: &'a [u8],
    offset: usize,
}

impl<'a> PayloadReader<'a> {
    pub fn new(payload: &'a [u8]) -> Self {
        Self { payload, offset: 0 }
    }

    pub fn remaining(&self) -> usize {
        self.payload.len().saturating_sub(self.offset)
    }

    fn take(&mut self, len: usize) -> Result<&'a [u8], DecoderError> {
        if self.offset + len > self.payload.len() {
            return Err(DecoderError::NotEnough {
                needed: self.offset + len,
                have: self.payload.len(),
            });
        }

        let start = self.offset;
        self.offset += len;
        Ok(&self.payload[start..start + len])
    }

    pub fn read_u32(&mut self) -> Result<u32, DecoderError> {
        let bytes = self.take(4)?;
        Ok(u32::from_le_bytes(
            bytes.try_into().expect("u32 slice length"),
        ))
    }

    pub fn read_u8(&mut self) -> Result<u8, DecoderError> {
        let bytes = self.take(1)?;
        Ok(bytes[0])
    }

    pub fn read_u64(&mut self) -> Result<u64, DecoderError> {
        let bytes = self.take(8)?;
        Ok(u64::from_le_bytes(
            bytes.try_into().expect("u64 slice length"),
        ))
    }

    pub fn read_bool_u32(&mut self) -> Result<bool, DecoderError> {
        Ok(self.read_u32()? != 0)
    }

    pub fn read_bytes(&mut self) -> Result<Vec<u8>, DecoderError> {
        let len = self.read_u32()? as usize;
        Ok(self.take(len)?.to_vec())
    }

    pub fn read_remaining_bytes(&mut self) -> Vec<u8> {
        if self.remaining() == 0 {
            return Vec::new();
        }
        let start = self.offset;
        self.offset = self.payload.len();
        self.payload[start..].to_vec()
    }

    pub fn read_string(&mut self) -> Result<String, DecoderError> {
        let len = self.read_u32()? as usize;
        let bytes = self.take(len)?;
        Ok(String::from_utf8_lossy(bytes).into_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmptyPayload;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpaquePayload {
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpaqueServerControlPayload {
    pub code: u32,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginRequestPayload {
    pub username: String,
    pub password: String,
    pub client_version: u32,
    pub md5hash: String,
    pub minor_version: u32,
}

pub type LoginPayload = LoginRequestPayload;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoginFailureReason {
    InvalidVersion,
    InvalidPass,
    InvalidUsername,
    Unknown(String),
}

impl LoginFailureReason {
    pub fn as_wire_str(&self) -> &str {
        match self {
            Self::InvalidVersion => "INVALIDVERSION",
            Self::InvalidPass => "INVALIDPASS",
            Self::InvalidUsername => "INVALIDUSERNAME",
            Self::Unknown(reason) => reason.as_str(),
        }
    }

    pub fn from_wire_str(value: &str) -> Self {
        match value {
            "INVALIDVERSION" => Self::InvalidVersion,
            "INVALIDPASS" => Self::InvalidPass,
            "INVALIDUSERNAME" => Self::InvalidUsername,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginResponseSuccessPayload {
    pub greeting: String,
    pub ip_address: String,
    pub md5hash: String,
    pub is_supporter: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginResponseFailurePayload {
    pub reason: LoginFailureReason,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoginResponsePayload {
    Success(LoginResponseSuccessPayload),
    Failure(LoginResponseFailurePayload),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetWaitPortPayload {
    pub listen_port: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserLookupPayload {
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerAddressResponsePayload {
    pub username: String,
    pub ip_address: String,
    pub port: u32,
    pub obfuscation_type: u32,
    pub obfuscated_port: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserStatusResponsePayload {
    pub username: String,
    pub status: u32,
    pub privileged: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserStatsResponsePayload {
    pub username: String,
    pub avg_speed: u32,
    pub download_num: u32,
    pub files: u32,
    pub dirs: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnPrivilegesStatusPayload {
    pub time_left_seconds: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserPrivilegesStatusPayload {
    pub username: String,
    pub privileged: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GivePrivilegePayload {
    pub username: String,
    pub days: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InformUserOfPrivilegesPayload {
    pub token: u32,
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InformUserOfPrivilegesAckPayload {
    pub token: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectToPeerPayload {
    pub username: String,
    pub token: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectToPeerRequestPayload {
    pub token: u32,
    pub username: String,
    pub connection_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectToPeerResponsePayload {
    pub username: String,
    pub connection_type: String,
    pub ip_address: String,
    pub port: u32,
    pub token: u32,
    pub privileged: bool,
    pub obfuscation_type: u32,
    pub obfuscated_port: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileSearchPayload {
    pub search_token: u32,
    pub search_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchRoomPayload {
    pub room: String,
    pub search_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExactFileSearchPayload {
    pub virtual_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchUserFilesPayload {
    pub username: String,
    pub search_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationEntry {
    pub term: String,
    pub score: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoredUserEntry {
    pub username: String,
    pub score: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationsPayload {
    pub recommendations: Vec<RecommendationEntry>,
    pub unrecommendations: Vec<RecommendationEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivilegedListPayload {
    pub users: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendedUsersPayload {
    pub users: Vec<ScoredUserEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TermRecommendationsPayload {
    pub term: String,
    pub recommendations: Vec<RecommendationEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationUsersPayload {
    pub term: String,
    pub users: Vec<ScoredUserEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRecommendationsPayload {
    pub username: String,
    pub recommendations: RecommendationsPayload,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimilarTermsRequestPayload {
    pub term: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimilarTermsPayload {
    pub term: String,
    pub entries: Vec<RecommendationEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoomListPayload {
    pub room_count: u32,
    pub rooms: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JoinRoomPayload {
    pub room: String,
    pub users: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeaveRoomPayload {
    pub room: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoomPresenceEventPayload {
    pub room: String,
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoomMembersPayload {
    pub room: String,
    pub users: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoomOperatorsPayload {
    pub room: String,
    pub operators: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoomModerationPayload {
    pub room: String,
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SayChatRoomPayload {
    pub room: String,
    pub username: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageUserPayload {
    pub username: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageUserIncomingPayload {
    pub message_id: u32,
    pub timestamp: u32,
    pub username: String,
    pub message: String,
    pub is_new: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageAckedPayload {
    pub message_id: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageUsersPayload {
    pub usernames: Vec<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerMessagePayload {
    pub username: String,
    pub message: String,
    pub token: Option<u32>,
    pub code: Option<u32>,
    pub ip_address: Option<String>,
    pub port: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedFoldersFilesPayload {
    pub folder_count: u32,
    pub file_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeedPayload {
    pub bytes_per_sec: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedFileEntry {
    pub virtual_path: String,
    pub size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedFileListPayload {
    pub entries: Vec<SharedFileEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedFilesInFolderRequestPayload {
    pub directory: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedFilesInFolderPayload {
    pub directory: String,
    pub compressed_listing: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileSearchRequestPayload {
    pub token: u32,
    pub query: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileSearchResultPayload {
    pub token: u32,
    pub username: String,
    pub result_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchFileSummary {
    pub file_path: String,
    pub file_size: u64,
    pub extension: String,
    pub attr_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResponseSummary {
    pub username: String,
    pub token: u32,
    pub files_count: u32,
    pub slots_free: u32,
    pub speed: u32,
    pub in_queue: bool,
    pub files: Vec<SearchFileSummary>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferDirection {
    Download = 0,
    Upload = 1,
}

impl TransferDirection {
    pub fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(raw: u32) -> Result<Self> {
        match raw {
            0 => Ok(Self::Download),
            1 => Ok(Self::Upload),
            other => bail!("invalid transfer direction: {other}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferRequestPayload {
    pub direction: TransferDirection,
    pub token: u32,
    pub virtual_path: String,
    pub file_size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferResponsePayload {
    pub token: u32,
    pub allowed: bool,
    pub queue_or_reason: String,
}

pub type TransferResponse = TransferResponsePayload;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueUploadPayload {
    pub username: String,
    pub virtual_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UploadPlaceInLinePayload {
    pub username: String,
    pub virtual_path: String,
    pub place: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UploadStatusPayload {
    pub username: String,
    pub virtual_path: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserInfoRequestPayload;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserInfoReplyPayload {
    pub description: String,
    pub has_picture: bool,
    pub picture: Vec<u8>,
    pub total_uploads: u32,
    pub queue_size: u32,
    pub slots_free: bool,
    pub upload_permissions: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerSearchQueryPayload {
    pub token: Option<u32>,
    pub query: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerRoomInvitePayload {
    pub room: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerVirtualPathPayload {
    pub virtual_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerQueuedDownloadsPayload {
    pub virtual_paths: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UploadPlaceInLineRequestPayload {
    pub virtual_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerMessage {
    Login(LoginPayload),
    LoginResponse(LoginResponsePayload),
    SetWaitPort(SetWaitPortPayload),
    GetPeerAddress(UserLookupPayload),
    GetPeerAddressResponse(PeerAddressResponsePayload),
    AddUser(UserLookupPayload),
    RemoveUser(UserLookupPayload),
    IgnoreUser(UserLookupPayload),
    UnignoreUser(UserLookupPayload),
    SayChatRoom(SayChatRoomPayload),
    JoinRoom(JoinRoomPayload),
    LeaveRoom(LeaveRoomPayload),
    UserJoinedRoom(RoomPresenceEventPayload),
    UserLeftRoom(RoomPresenceEventPayload),
    ConnectToPeerRequest(ConnectToPeerRequestPayload),
    ConnectToPeerResponse(ConnectToPeerResponsePayload),
    ConnectToPeer(ConnectToPeerPayload),
    FileSearch(FileSearchPayload),
    LowPriorityFileSearch(FileSearchPayload),
    SendConnectToken(OpaquePayload),
    PlaceInLine(OpaquePayload),
    PlaceInLineResponse(OpaquePayload),
    RoomList(RoomListPayload),
    FileSearchResponseSummary(SearchResponseSummary),
    PrivilegedList(PrivilegedListPayload),
    AddPrivilegedUser(UserLookupPayload),
    SearchRoom(SearchRoomPayload),
    ExactFileSearch(ExactFileSearchPayload),
    SearchUserFiles(SearchUserFilesPayload),
    BanUser(UserLookupPayload),
    GetSimilarTerms(SimilarTermsRequestPayload),
    GetSimilarTermsResponse(SimilarTermsPayload),
    GetRecommendations(EmptyPayload),
    GetRecommendationsResponse(RecommendationsPayload),
    GetMyRecommendations(EmptyPayload),
    GetMyRecommendationsResponse(RecommendationsPayload),
    GetGlobalRecommendations(EmptyPayload),
    GetGlobalRecommendationsResponse(RecommendationsPayload),
    GetOwnPrivilegesStatus(EmptyPayload),
    OwnPrivilegesStatus(OwnPrivilegesStatusPayload),
    WishlistWait(OpaquePayload),
    GetUserPrivilegesStatus(UserLookupPayload),
    UserPrivilegesStatus(UserPrivilegesStatusPayload),
    GivePrivilege(GivePrivilegePayload),
    InformUserOfPrivileges(InformUserOfPrivilegesPayload),
    InformUserOfPrivilegesAck(InformUserOfPrivilegesAckPayload),
    GetUserRecommendations(UserLookupPayload),
    GetUserRecommendationsResponse(UserRecommendationsPayload),
    GetRecommendedUsers(EmptyPayload),
    GetRecommendedUsersResponse(RecommendedUsersPayload),
    GetTermRecommendations(SimilarTermsRequestPayload),
    GetTermRecommendationsResponse(TermRecommendationsPayload),
    GetRecommendationUsers(SimilarTermsRequestPayload),
    GetRecommendationUsersResponse(RecommendationUsersPayload),
    AddRoomMember(RoomModerationPayload),
    RemoveRoomMember(RoomModerationPayload),
    AddRoomOperator(RoomModerationPayload),
    RemoveRoomOperator(RoomModerationPayload),
    RemoveRoomOperatorship(OpaquePayload),
    RemoveOwnRoomOperatorship(OpaquePayload),
    RoomMembers(RoomMembersPayload),
    RoomOperators(RoomOperatorsPayload),
    JoinGlobalRoom(OpaquePayload),
    LeaveGlobalRoom(OpaquePayload),
    SayGlobalRoom(OpaquePayload),
    SearchCorrelations(OpaquePayload),
    DnetLevel(OpaquePayload),
    DnetGroupLeader(OpaquePayload),
    DnetDeliveryReport(OpaquePayload),
    DnetChildDepth(OpaquePayload),
    Flood(OpaquePayload),
    MessageUserIncoming(MessageUserIncomingPayload),
    MessageUser(MessageUserPayload),
    MessageAcked(MessageAckedPayload),
    MessageUsers(MessageUsersPayload),
    PeerMessage(PeerMessagePayload),
    GetUserStats(UserLookupPayload),
    GetUserStatsResponse(UserStatsResponsePayload),
    GetUserStatus(UserLookupPayload),
    GetUserStatusResponse(UserStatusResponsePayload),
    SharedFoldersFiles(SharedFoldersFilesPayload),
    DownloadSpeed(SpeedPayload),
    UploadSpeed(SpeedPayload),
    OpaqueControl(OpaqueServerControlPayload),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeerMessage {
    Say(OpaquePayload),
    GetSharedFileList(UserLookupPayload),
    SharedFileList(SharedFileListPayload),
    GetSharedFilesInFolder(SharedFilesInFolderRequestPayload),
    SharedFilesInFolder(SharedFilesInFolderPayload),
    FileSearchRequest(FileSearchRequestPayload),
    FileSearchResult(FileSearchResultPayload),
    InviteUserToRoom(PeerRoomInvitePayload),
    CancelledQueuedTransfer(PeerVirtualPathPayload),
    UserInfoRequest(UserInfoRequestPayload),
    UserInfoReply(UserInfoReplyPayload),
    SendConnectToken(OpaquePayload),
    MoveDownloadToTop(PeerVirtualPathPayload),
    TransferRequest(TransferRequestPayload),
    TransferResponse(TransferResponsePayload),
    PlaceholderUpload(OpaquePayload),
    QueueUpload(QueueUploadPayload),
    UploadPlaceInLine(UploadPlaceInLinePayload),
    ExactFileSearchRequest(PeerSearchQueryPayload),
    QueuedDownloads(PeerQueuedDownloadsPayload),
    IndirectFileSearchRequest(PeerSearchQueryPayload),
    UploadFailed(UploadStatusPayload),
    UploadDenied(UploadStatusPayload),
    UploadPlaceInLineRequest(UploadPlaceInLineRequestPayload),
    Nothing(OpaquePayload),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtocolMessage {
    Server(ServerMessage),
    Peer(PeerMessage),
}

fn ensure_payload_consumed(reader: &PayloadReader<'_>) -> Result<()> {
    if reader.remaining() != 0 {
        bail!("unexpected trailing payload bytes: {}", reader.remaining());
    }
    Ok(())
}

pub const OPAQUE_SERVER_CONTROL_CODES: [u32; 40] = [
    CODE_SM_ADD_CHATROOM,
    CODE_SM_SET_STATUS,
    CODE_SM_HEARTBEAT,
    CODE_SM_RELOGGED,
    CODE_SM_ADD_LIKE_TERM,
    CODE_SM_REMOVE_LIKE_TERM,
    CODE_SM_COMMAND,
    CODE_SM_USER_LIST,
    CODE_SM_ROOM_ADDED,
    CODE_SM_ROOM_REMOVED,
    CODE_SM_ADMIN_MESSAGE,
    CODE_SM_GLOBAL_USER_LIST,
    CODE_SM_CONNECT_TO_CLIENT,
    CODE_SM_SEND_DISTRIBUTIONS,
    CODE_SM_NOTE_PARENT,
    CODE_SM_CHILD_PARENT_MAP,
    CODE_SM_SET_PARENT_MIN_SPEED,
    CODE_SM_SET_PARENT_SPEED_CONNECTION_RATIO,
    CODE_SM_SET_PARENT_INACTIVITY_BEFORE_DISCONNECT,
    CODE_SM_SET_SERVER_INACTIVITY_BEFORE_DISCONNECT,
    CODE_SM_NODES_IN_CACHE_BEFORE_DISCONNECT,
    CODE_SM_SET_SECONDS_BEFORE_PING_CHILDREN,
    CODE_SM_DNET_MESSAGE,
    CODE_SM_CAN_PARENT,
    CODE_SM_POSSIBLE_PARENTS,
    CODE_SM_GET_ROOM_TICKER,
    CODE_SM_ROOM_TICKER_USER_ADDED,
    CODE_SM_ROOM_TICKER_USER_REMOVED,
    CODE_SM_SET_TICKER,
    CODE_SM_ADD_HATE_TERM,
    CODE_SM_REMOVE_HATE_TERM,
    CODE_SM_DNET_RESET,
    CODE_SM_REMOVE_OWN_ROOM_MEMBERSHIP,
    CODE_SM_GIVE_UP_ROOM,
    CODE_SM_TRANSFER_ROOM_OWNERSHIP,
    CODE_SM_ADD_ROOM_MEMBERSHIP,
    CODE_SM_REMOVE_ROOM_MEMBERSHIP,
    CODE_SM_ENABLE_PRIVATE_ROOM_ADD,
    CODE_SM_CHANGE_PASSWORD,
    CODE_SM_ADD_ROOM_OPERATORSHIP,
];

pub fn is_opaque_server_control_code(code: u32) -> bool {
    OPAQUE_SERVER_CONTROL_CODES.contains(&code)
}

fn encode_recommendations_payload(writer: &mut PayloadWriter, payload: &RecommendationsPayload) {
    writer.write_u32(payload.recommendations.len() as u32);
    for entry in &payload.recommendations {
        writer.write_string(&entry.term);
        writer.write_u32(entry.score as u32);
    }

    writer.write_u32(payload.unrecommendations.len() as u32);
    for entry in &payload.unrecommendations {
        writer.write_string(&entry.term);
        writer.write_u32(entry.score as u32);
    }
}

pub fn encode_message(message: &ProtocolMessage) -> Frame {
    match message {
        ProtocolMessage::Server(server) => encode_server_message(server),
        ProtocolMessage::Peer(peer) => encode_peer_message(peer),
    }
}

pub fn decode_message(frame: &Frame) -> Result<ProtocolMessage> {
    let server = decode_server_message(frame.code, &frame.payload);
    let peer = decode_peer_message(frame.code, &frame.payload);

    match (server, peer) {
        (Ok(server), Err(_)) => Ok(ProtocolMessage::Server(server)),
        (Err(_), Ok(peer)) => Ok(ProtocolMessage::Peer(peer)),
        (Ok(_), Ok(_)) => bail!(
            "ambiguous message code {} decodes as both server and peer; use scoped decoder",
            frame.code
        ),
        (Err(_), Err(_)) => bail!(
            "unsupported message code {} (payload_len={})",
            frame.code,
            frame.payload.len()
        ),
    }
}

pub fn encode_server_message(message: &ServerMessage) -> Frame {
    let mut writer = PayloadWriter::new();
    let code = match message {
        ServerMessage::Login(payload) => {
            writer.write_string(&payload.username);
            writer.write_string(&payload.password);
            writer.write_u32(payload.client_version);
            writer.write_string(&payload.md5hash);
            writer.write_u32(payload.minor_version);
            CODE_SM_LOGIN
        }
        ServerMessage::LoginResponse(payload) => {
            match payload {
                LoginResponsePayload::Success(success) => {
                    writer.write_u8(1);
                    writer.write_string(&success.greeting);
                    let ip = success
                        .ip_address
                        .parse::<Ipv4Addr>()
                        .unwrap_or(Ipv4Addr::UNSPECIFIED);
                    writer.write_u32(u32::from_le_bytes(ip.octets()));
                    writer.write_string(&success.md5hash);
                    writer.write_u8(u8::from(success.is_supporter));
                }
                LoginResponsePayload::Failure(failure) => {
                    writer.write_u8(0);
                    writer.write_string(failure.reason.as_wire_str());
                    if let Some(detail) = &failure.detail {
                        writer.write_string(detail);
                    }
                }
            }
            CODE_SM_LOGIN
        }
        ServerMessage::SetWaitPort(payload) => {
            writer.write_u32(payload.listen_port);
            CODE_SM_SET_WAIT_PORT
        }
        ServerMessage::GetPeerAddress(payload) => {
            writer.write_string(&payload.username);
            CODE_SM_GET_PEER_ADDRESS
        }
        ServerMessage::GetPeerAddressResponse(payload) => {
            writer.write_string(&payload.username);
            let ip = payload
                .ip_address
                .parse::<Ipv4Addr>()
                .unwrap_or(Ipv4Addr::UNSPECIFIED);
            writer.write_u32(u32::from_le_bytes(ip.octets()));
            writer.write_u32(payload.port);
            writer.write_u32(payload.obfuscation_type);
            writer.write_raw_bytes(&payload.obfuscated_port.to_le_bytes());
            CODE_SM_GET_PEER_ADDRESS
        }
        ServerMessage::AddUser(payload) => {
            writer.write_string(&payload.username);
            CODE_SM_ADD_USER
        }
        ServerMessage::RemoveUser(payload) => {
            writer.write_string(&payload.username);
            CODE_SM_REMOVE_USER
        }
        ServerMessage::IgnoreUser(payload) => {
            writer.write_string(&payload.username);
            CODE_SM_IGNORE_USER
        }
        ServerMessage::UnignoreUser(payload) => {
            writer.write_string(&payload.username);
            CODE_SM_UNIGNORE_USER
        }
        ServerMessage::SayChatRoom(payload) => {
            writer.write_string(&payload.room);
            if let Some(username) = &payload.username {
                writer.write_string(username);
            }
            writer.write_string(&payload.message);
            CODE_SM_SAY_CHATROOM
        }
        ServerMessage::JoinRoom(payload) => {
            writer.write_string(&payload.room);
            if !payload.users.is_empty() {
                writer.write_u32(payload.users.len() as u32);
                for user in &payload.users {
                    writer.write_string(user);
                }
            }
            CODE_SM_JOIN_ROOM
        }
        ServerMessage::LeaveRoom(payload) => {
            writer.write_string(&payload.room);
            CODE_SM_LEAVE_ROOM
        }
        ServerMessage::UserJoinedRoom(payload) => {
            writer.write_string(&payload.room);
            writer.write_string(&payload.username);
            CODE_SM_USER_JOINED_ROOM
        }
        ServerMessage::UserLeftRoom(payload) => {
            writer.write_string(&payload.room);
            writer.write_string(&payload.username);
            CODE_SM_USER_LEFT_ROOM
        }
        ServerMessage::ConnectToPeerRequest(payload) => {
            writer.write_u32(payload.token);
            writer.write_string(&payload.username);
            writer.write_string(&payload.connection_type);
            CODE_SM_CONNECT_TO_PEER
        }
        ServerMessage::ConnectToPeerResponse(payload) => {
            writer.write_string(&payload.username);
            writer.write_string(&payload.connection_type);
            let ip = payload
                .ip_address
                .parse::<Ipv4Addr>()
                .unwrap_or(Ipv4Addr::UNSPECIFIED);
            writer.write_u32(u32::from_le_bytes(ip.octets()));
            writer.write_u32(payload.port);
            writer.write_u32(payload.token);
            writer.write_bool_u32(payload.privileged);
            writer.write_u32(payload.obfuscation_type);
            writer.write_u32(payload.obfuscated_port);
            CODE_SM_CONNECT_TO_PEER
        }
        ServerMessage::ConnectToPeer(payload) => {
            writer.write_string(&payload.username);
            writer.write_u32(payload.token);
            CODE_SM_CONNECT_TO_PEER
        }
        ServerMessage::FileSearch(payload) => {
            writer.write_u32(payload.search_token);
            writer.write_string(&payload.search_text);
            CODE_SM_FILE_SEARCH
        }
        ServerMessage::LowPriorityFileSearch(payload) => {
            writer.write_u32(payload.search_token);
            writer.write_string(&payload.search_text);
            CODE_SM_LOW_PRIORITY_FILE_SEARCH
        }
        ServerMessage::SendConnectToken(payload) => {
            writer.write_raw_bytes(&payload.bytes);
            CODE_SM_SEND_CONNECT_TOKEN
        }
        ServerMessage::PlaceInLine(payload) => {
            writer.write_raw_bytes(&payload.bytes);
            CODE_SM_PLACE_IN_LINE
        }
        ServerMessage::PlaceInLineResponse(payload) => {
            writer.write_raw_bytes(&payload.bytes);
            CODE_SM_PLACE_IN_LINE_RESPONSE
        }
        ServerMessage::RoomList(payload) => {
            writer.write_u32(payload.room_count);
            for room in &payload.rooms {
                writer.write_string(room);
            }
            CODE_SM_ROOM_LIST
        }
        ServerMessage::PrivilegedList(payload) => {
            writer.write_u32(payload.users.len() as u32);
            for user in &payload.users {
                writer.write_string(user);
            }
            CODE_SM_PRIVILEGED_LIST
        }
        ServerMessage::AddPrivilegedUser(payload) => {
            writer.write_string(&payload.username);
            CODE_SM_ADD_PRIVILEGED_USER
        }
        ServerMessage::FileSearchResponseSummary(payload) => {
            writer.write_string(&payload.username);
            writer.write_u32(payload.token);
            writer.write_u32(payload.files_count);
            for file in &payload.files {
                writer.write_string(&file.file_path);
                writer.write_u64(file.file_size);
                writer.write_string(&file.extension);
                writer.write_u32(file.attr_count);
            }
            writer.write_u32(payload.slots_free);
            writer.write_u32(payload.speed);
            writer.write_bool_u32(payload.in_queue);
            CODE_SM_FILE_SEARCH_RESPONSE
        }
        ServerMessage::SearchRoom(payload) => {
            writer.write_string(&payload.room);
            writer.write_string(&payload.search_text);
            CODE_SM_SEARCH_ROOM
        }
        ServerMessage::ExactFileSearch(payload) => {
            writer.write_string(&payload.virtual_path);
            CODE_SM_EXACT_FILE_SEARCH
        }
        ServerMessage::SearchUserFiles(payload) => {
            writer.write_string(&payload.username);
            writer.write_string(&payload.search_text);
            CODE_SM_SEARCH_USER_FILES
        }
        ServerMessage::BanUser(payload) => {
            writer.write_string(&payload.username);
            CODE_SM_BAN_USER
        }
        ServerMessage::GetSimilarTerms(payload) => {
            writer.write_string(&payload.term);
            CODE_SM_GET_SIMILAR_TERMS
        }
        ServerMessage::GetSimilarTermsResponse(payload) => {
            writer.write_string(&payload.term);
            writer.write_u32(payload.entries.len() as u32);
            for entry in &payload.entries {
                writer.write_string(&entry.term);
                writer.write_u32(entry.score as u32);
            }
            CODE_SM_GET_SIMILAR_TERMS
        }
        ServerMessage::GetRecommendations(_) => CODE_SM_GET_RECOMMENDATIONS,
        ServerMessage::GetRecommendationsResponse(payload) => {
            encode_recommendations_payload(&mut writer, payload);
            CODE_SM_GET_RECOMMENDATIONS
        }
        ServerMessage::GetMyRecommendations(_) => CODE_SM_GET_MY_RECOMMENDATIONS,
        ServerMessage::GetMyRecommendationsResponse(payload) => {
            encode_recommendations_payload(&mut writer, payload);
            CODE_SM_GET_MY_RECOMMENDATIONS
        }
        ServerMessage::GetGlobalRecommendations(_) => CODE_SM_GET_GLOBAL_RECOMMENDATIONS,
        ServerMessage::GetGlobalRecommendationsResponse(payload) => {
            encode_recommendations_payload(&mut writer, payload);
            CODE_SM_GET_GLOBAL_RECOMMENDATIONS
        }
        ServerMessage::GetOwnPrivilegesStatus(_) => CODE_SM_GET_OWN_PRIVILEGES_STATUS,
        ServerMessage::OwnPrivilegesStatus(payload) => {
            writer.write_u32(payload.time_left_seconds);
            CODE_SM_GET_OWN_PRIVILEGES_STATUS
        }
        ServerMessage::WishlistWait(payload) => {
            writer.write_raw_bytes(&payload.bytes);
            CODE_SM_WISHLIST_WAIT
        }
        ServerMessage::GetUserPrivilegesStatus(payload) => {
            writer.write_string(&payload.username);
            CODE_SM_GET_USER_PRIVILEGES_STATUS
        }
        ServerMessage::UserPrivilegesStatus(payload) => {
            writer.write_string(&payload.username);
            writer.write_bool_u32(payload.privileged);
            CODE_SM_GET_USER_PRIVILEGES_STATUS
        }
        ServerMessage::GivePrivilege(payload) => {
            writer.write_string(&payload.username);
            writer.write_u32(payload.days);
            CODE_SM_GIVE_PRIVILEGE
        }
        ServerMessage::InformUserOfPrivileges(payload) => {
            writer.write_u32(payload.token);
            writer.write_string(&payload.username);
            CODE_SM_INFORM_USER_OF_PRIVILEGES
        }
        ServerMessage::InformUserOfPrivilegesAck(payload) => {
            writer.write_u32(payload.token);
            CODE_SM_INFORM_USER_OF_PRIVILEGES_ACK
        }
        ServerMessage::GetUserRecommendations(payload) => {
            writer.write_string(&payload.username);
            CODE_SM_GET_USER_RECOMMENDATIONS
        }
        ServerMessage::GetUserRecommendationsResponse(payload) => {
            writer.write_string(&payload.username);
            encode_recommendations_payload(&mut writer, &payload.recommendations);
            CODE_SM_GET_USER_RECOMMENDATIONS
        }
        ServerMessage::GetRecommendedUsers(_) => CODE_SM_GET_RECOMMENDED_USERS,
        ServerMessage::GetRecommendedUsersResponse(payload) => {
            writer.write_u32(payload.users.len() as u32);
            for entry in &payload.users {
                writer.write_string(&entry.username);
                writer.write_u32(entry.score as u32);
            }
            CODE_SM_GET_RECOMMENDED_USERS
        }
        ServerMessage::GetTermRecommendations(payload) => {
            writer.write_string(&payload.term);
            CODE_SM_GET_TERM_RECOMMENDATIONS
        }
        ServerMessage::GetTermRecommendationsResponse(payload) => {
            writer.write_string(&payload.term);
            writer.write_u32(payload.recommendations.len() as u32);
            for entry in &payload.recommendations {
                writer.write_string(&entry.term);
                writer.write_u32(entry.score as u32);
            }
            CODE_SM_GET_TERM_RECOMMENDATIONS
        }
        ServerMessage::GetRecommendationUsers(payload) => {
            writer.write_string(&payload.term);
            CODE_SM_GET_RECOMMENDATION_USERS
        }
        ServerMessage::GetRecommendationUsersResponse(payload) => {
            writer.write_string(&payload.term);
            writer.write_u32(payload.users.len() as u32);
            for entry in &payload.users {
                writer.write_string(&entry.username);
                writer.write_u32(entry.score as u32);
            }
            CODE_SM_GET_RECOMMENDATION_USERS
        }
        ServerMessage::AddRoomMember(payload) => {
            writer.write_string(&payload.room);
            writer.write_string(&payload.username);
            CODE_SM_ADD_ROOM_MEMBER
        }
        ServerMessage::RemoveRoomMember(payload) => {
            writer.write_string(&payload.room);
            writer.write_string(&payload.username);
            CODE_SM_REMOVE_ROOM_MEMBER
        }
        ServerMessage::AddRoomOperator(payload) => {
            writer.write_string(&payload.room);
            writer.write_string(&payload.username);
            CODE_SM_ADD_ROOM_OPERATOR
        }
        ServerMessage::RemoveRoomOperator(payload) => {
            writer.write_string(&payload.room);
            writer.write_string(&payload.username);
            CODE_SM_REMOVE_ROOM_OPERATOR
        }
        ServerMessage::RemoveRoomOperatorship(payload) => {
            writer.write_raw_bytes(&payload.bytes);
            CODE_SM_REMOVE_ROOM_OPERATORSHIP
        }
        ServerMessage::RemoveOwnRoomOperatorship(payload) => {
            writer.write_raw_bytes(&payload.bytes);
            CODE_SM_REMOVE_OWN_ROOM_OPERATORSHIP
        }
        ServerMessage::RoomMembers(payload) => {
            writer.write_string(&payload.room);
            if !payload.users.is_empty() {
                writer.write_u32(payload.users.len() as u32);
                for user in &payload.users {
                    writer.write_string(user);
                }
            }
            CODE_SM_ROOM_MEMBERS
        }
        ServerMessage::RoomOperators(payload) => {
            writer.write_string(&payload.room);
            if !payload.operators.is_empty() {
                writer.write_u32(payload.operators.len() as u32);
                for user in &payload.operators {
                    writer.write_string(user);
                }
            }
            CODE_SM_ROOM_OPERATORS
        }
        ServerMessage::JoinGlobalRoom(payload) => {
            writer.write_raw_bytes(&payload.bytes);
            CODE_SM_JOIN_GLOBAL_ROOM
        }
        ServerMessage::LeaveGlobalRoom(payload) => {
            writer.write_raw_bytes(&payload.bytes);
            CODE_SM_LEAVE_GLOBAL_ROOM
        }
        ServerMessage::SayGlobalRoom(payload) => {
            writer.write_raw_bytes(&payload.bytes);
            CODE_SM_SAY_GLOBAL_ROOM
        }
        ServerMessage::SearchCorrelations(payload) => {
            writer.write_raw_bytes(&payload.bytes);
            CODE_SM_SEARCH_CORRELATIONS
        }
        ServerMessage::DnetLevel(payload) => {
            writer.write_raw_bytes(&payload.bytes);
            CODE_SM_DNET_LEVEL
        }
        ServerMessage::DnetGroupLeader(payload) => {
            writer.write_raw_bytes(&payload.bytes);
            CODE_SM_DNET_GROUP_LEADER
        }
        ServerMessage::DnetDeliveryReport(payload) => {
            writer.write_raw_bytes(&payload.bytes);
            CODE_SM_DNET_DELIVERY_REPORT
        }
        ServerMessage::DnetChildDepth(payload) => {
            writer.write_raw_bytes(&payload.bytes);
            CODE_SM_DNET_CHILD_DEPTH
        }
        ServerMessage::Flood(payload) => {
            writer.write_raw_bytes(&payload.bytes);
            CODE_SM_FLOOD
        }
        ServerMessage::MessageUserIncoming(payload) => {
            writer.write_u32(payload.message_id);
            writer.write_u32(payload.timestamp);
            writer.write_string(&payload.username);
            writer.write_string(&payload.message);
            writer.write_u8(u8::from(payload.is_new));
            CODE_SM_MESSAGE_USER
        }
        ServerMessage::MessageUser(payload) => {
            writer.write_string(&payload.username);
            writer.write_string(&payload.message);
            CODE_SM_MESSAGE_USER
        }
        ServerMessage::MessageAcked(payload) => {
            writer.write_u32(payload.message_id);
            CODE_SM_MESSAGE_ACKED
        }
        ServerMessage::MessageUsers(payload) => {
            writer.write_u32(payload.usernames.len() as u32);
            for username in &payload.usernames {
                writer.write_string(username);
            }
            writer.write_string(&payload.message);
            CODE_SM_MESSAGE_USERS
        }
        ServerMessage::PeerMessage(payload) => {
            writer.write_string(&payload.username);
            if let (Some(ip), Some(port)) = (&payload.ip_address, payload.port) {
                writer.write_u32(payload.code.unwrap_or(0));
                writer.write_u32(payload.token.unwrap_or(0));
                let parsed = ip.parse::<Ipv4Addr>().unwrap_or(Ipv4Addr::UNSPECIFIED);
                writer.write_u32(u32::from_le_bytes(parsed.octets()));
                writer.write_u32(port);
            } else {
                writer.write_u32(payload.token.unwrap_or(0));
                writer.write_u32(payload.code.unwrap_or(0));
            }
            writer.write_string(&payload.message);
            CODE_SM_PEER_MESSAGE
        }
        ServerMessage::GetUserStats(payload) => {
            writer.write_string(&payload.username);
            CODE_SM_GET_USER_STATS
        }
        ServerMessage::GetUserStatsResponse(payload) => {
            writer.write_string(&payload.username);
            writer.write_u32(payload.avg_speed);
            writer.write_u32(payload.download_num);
            writer.write_u32(payload.files);
            writer.write_u32(payload.dirs);
            CODE_SM_GET_USER_STATS
        }
        ServerMessage::GetUserStatus(payload) => {
            writer.write_string(&payload.username);
            CODE_SM_GET_USER_STATUS
        }
        ServerMessage::GetUserStatusResponse(payload) => {
            writer.write_string(&payload.username);
            writer.write_u32(payload.status);
            writer.write_bool_u32(payload.privileged);
            CODE_SM_GET_USER_STATUS
        }
        ServerMessage::SharedFoldersFiles(payload) => {
            writer.write_u32(payload.folder_count);
            writer.write_u32(payload.file_count);
            CODE_SM_SHARED_FOLDERS_FILES
        }
        ServerMessage::DownloadSpeed(payload) => {
            writer.write_u32(payload.bytes_per_sec);
            CODE_SM_DOWNLOAD_SPEED
        }
        ServerMessage::UploadSpeed(payload) => {
            writer.write_u32(payload.bytes_per_sec);
            CODE_SM_UPLOAD_SPEED
        }
        ServerMessage::OpaqueControl(payload) => {
            writer.write_raw_bytes(&payload.bytes);
            payload.code
        }
    };

    Frame::new(code, writer.into_inner())
}

pub fn decode_server_message(code: u32, payload: &[u8]) -> Result<ServerMessage> {
    if code == CODE_SM_LOGIN {
        if let Ok(login) = decode_login_request_payload(payload) {
            return Ok(ServerMessage::Login(login));
        }
        return Ok(ServerMessage::LoginResponse(parse_login_response(payload)?));
    }

    if code == CODE_SM_ROOM_LIST {
        if let Ok(room_list) = parse_room_list_payload(payload) {
            return Ok(ServerMessage::RoomList(room_list));
        }
        return Ok(ServerMessage::FileSearchResponseSummary(
            parse_search_response_summary(payload)?,
        ));
    }

    let mut reader = PayloadReader::new(payload);
    let mut allow_trailing_bytes = false;

    let message = match code {
        CODE_SM_SET_WAIT_PORT => {
            let payload = SetWaitPortPayload {
                listen_port: reader.read_u32()?,
            };
            ServerMessage::SetWaitPort(payload)
        }
        CODE_SM_GET_PEER_ADDRESS => {
            allow_trailing_bytes = true;
            if let Ok(request) = parse_user_lookup_payload(payload) {
                ServerMessage::GetPeerAddress(request)
            } else {
                ServerMessage::GetPeerAddressResponse(parse_peer_address_response_payload(payload)?)
            }
        }
        CODE_SM_ADD_USER => {
            let payload = UserLookupPayload {
                username: reader.read_string()?,
            };
            ServerMessage::AddUser(payload)
        }
        CODE_SM_REMOVE_USER => {
            let payload = UserLookupPayload {
                username: reader.read_string()?,
            };
            ServerMessage::RemoveUser(payload)
        }
        CODE_SM_IGNORE_USER => {
            let payload = UserLookupPayload {
                username: reader.read_string()?,
            };
            ServerMessage::IgnoreUser(payload)
        }
        CODE_SM_UNIGNORE_USER => {
            let payload = UserLookupPayload {
                username: reader.read_string()?,
            };
            ServerMessage::UnignoreUser(payload)
        }
        CODE_SM_SAY_CHATROOM => {
            allow_trailing_bytes = true;
            ServerMessage::SayChatRoom(parse_say_chatroom_payload(payload)?)
        }
        CODE_SM_JOIN_ROOM => {
            allow_trailing_bytes = true;
            ServerMessage::JoinRoom(parse_join_room_payload(payload)?)
        }
        CODE_SM_LEAVE_ROOM => {
            let payload = LeaveRoomPayload {
                room: reader.read_string()?,
            };
            ServerMessage::LeaveRoom(payload)
        }
        CODE_SM_USER_JOINED_ROOM => {
            allow_trailing_bytes = true;
            ServerMessage::UserJoinedRoom(parse_room_presence_event_payload(payload)?)
        }
        CODE_SM_USER_LEFT_ROOM => {
            allow_trailing_bytes = true;
            ServerMessage::UserLeftRoom(parse_room_presence_event_payload(payload)?)
        }
        CODE_SM_CONNECT_TO_PEER => {
            allow_trailing_bytes = true;
            if let Ok(response) = parse_connect_to_peer_response_payload(payload) {
                ServerMessage::ConnectToPeerResponse(response)
            } else if let Ok(request) = parse_connect_to_peer_request_payload(payload) {
                ServerMessage::ConnectToPeerRequest(request)
            } else {
                ServerMessage::ConnectToPeer(parse_connect_to_peer_legacy_payload(payload)?)
            }
        }
        CODE_SM_FILE_SEARCH => {
            let payload = FileSearchPayload {
                search_token: reader.read_u32()?,
                search_text: reader.read_string()?,
            };
            ServerMessage::FileSearch(payload)
        }
        CODE_SM_LOW_PRIORITY_FILE_SEARCH => {
            let payload = FileSearchPayload {
                search_token: reader.read_u32()?,
                search_text: reader.read_string()?,
            };
            ServerMessage::LowPriorityFileSearch(payload)
        }
        CODE_SM_SEND_CONNECT_TOKEN => {
            allow_trailing_bytes = true;
            ServerMessage::SendConnectToken(OpaquePayload {
                bytes: payload.to_vec(),
            })
        }
        CODE_SM_PLACE_IN_LINE => {
            allow_trailing_bytes = true;
            ServerMessage::PlaceInLine(OpaquePayload {
                bytes: payload.to_vec(),
            })
        }
        CODE_SM_PLACE_IN_LINE_RESPONSE => {
            allow_trailing_bytes = true;
            ServerMessage::PlaceInLineResponse(OpaquePayload {
                bytes: payload.to_vec(),
            })
        }
        CODE_SM_PRIVILEGED_LIST => {
            allow_trailing_bytes = true;
            ServerMessage::PrivilegedList(parse_privileged_list_payload(payload)?)
        }
        CODE_SM_ADD_PRIVILEGED_USER => {
            let payload = UserLookupPayload {
                username: reader.read_string()?,
            };
            ServerMessage::AddPrivilegedUser(payload)
        }
        CODE_SM_SEARCH_ROOM => {
            let payload = SearchRoomPayload {
                room: reader.read_string()?,
                search_text: reader.read_string()?,
            };
            ServerMessage::SearchRoom(payload)
        }
        CODE_SM_EXACT_FILE_SEARCH => {
            let payload = ExactFileSearchPayload {
                virtual_path: reader.read_string()?,
            };
            ServerMessage::ExactFileSearch(payload)
        }
        CODE_SM_SEARCH_USER_FILES => {
            let payload = SearchUserFilesPayload {
                username: reader.read_string()?,
                search_text: reader.read_string()?,
            };
            ServerMessage::SearchUserFiles(payload)
        }
        CODE_SM_BAN_USER => {
            let payload = UserLookupPayload {
                username: reader.read_string()?,
            };
            ServerMessage::BanUser(payload)
        }
        CODE_SM_GET_SIMILAR_TERMS => {
            allow_trailing_bytes = true;
            if let Ok(request) = parse_similar_terms_request(payload) {
                ServerMessage::GetSimilarTerms(request)
            } else {
                ServerMessage::GetSimilarTermsResponse(parse_similar_terms_response(payload)?)
            }
        }
        CODE_SM_GET_RECOMMENDED_USERS => {
            allow_trailing_bytes = true;
            if payload.is_empty() {
                ServerMessage::GetRecommendedUsers(EmptyPayload)
            } else {
                ServerMessage::GetRecommendedUsersResponse(parse_recommended_users_payload(
                    payload,
                )?)
            }
        }
        CODE_SM_GET_TERM_RECOMMENDATIONS => {
            allow_trailing_bytes = true;
            if let Ok(request) = parse_similar_terms_request(payload) {
                ServerMessage::GetTermRecommendations(request)
            } else {
                ServerMessage::GetTermRecommendationsResponse(parse_term_recommendations_payload(
                    payload,
                )?)
            }
        }
        CODE_SM_GET_RECOMMENDATION_USERS => {
            allow_trailing_bytes = true;
            if let Ok(request) = parse_similar_terms_request(payload) {
                ServerMessage::GetRecommendationUsers(request)
            } else {
                ServerMessage::GetRecommendationUsersResponse(parse_recommendation_users_payload(
                    payload,
                )?)
            }
        }
        CODE_SM_GET_RECOMMENDATIONS => {
            allow_trailing_bytes = true;
            if payload.is_empty() {
                ServerMessage::GetRecommendations(EmptyPayload)
            } else {
                ServerMessage::GetRecommendationsResponse(parse_recommendations_payload(payload)?)
            }
        }
        CODE_SM_GET_MY_RECOMMENDATIONS => {
            allow_trailing_bytes = true;
            if payload.is_empty() {
                ServerMessage::GetMyRecommendations(EmptyPayload)
            } else {
                ServerMessage::GetMyRecommendationsResponse(parse_recommendations_payload(payload)?)
            }
        }
        CODE_SM_GET_GLOBAL_RECOMMENDATIONS => {
            allow_trailing_bytes = true;
            if payload.is_empty() {
                ServerMessage::GetGlobalRecommendations(EmptyPayload)
            } else {
                ServerMessage::GetGlobalRecommendationsResponse(parse_recommendations_payload(
                    payload,
                )?)
            }
        }
        CODE_SM_GET_OWN_PRIVILEGES_STATUS => {
            allow_trailing_bytes = true;
            if payload.is_empty() {
                ServerMessage::GetOwnPrivilegesStatus(EmptyPayload)
            } else {
                ServerMessage::OwnPrivilegesStatus(parse_own_privileges_status_payload(payload)?)
            }
        }
        CODE_SM_WISHLIST_WAIT => {
            allow_trailing_bytes = true;
            ServerMessage::WishlistWait(OpaquePayload {
                bytes: payload.to_vec(),
            })
        }
        CODE_SM_GET_USER_PRIVILEGES_STATUS => {
            allow_trailing_bytes = true;
            if let Ok(request) = parse_user_lookup_payload(payload) {
                ServerMessage::GetUserPrivilegesStatus(request)
            } else {
                ServerMessage::UserPrivilegesStatus(parse_user_privileges_status_payload(payload)?)
            }
        }
        CODE_SM_GIVE_PRIVILEGE => {
            let payload = GivePrivilegePayload {
                username: reader.read_string()?,
                days: reader.read_u32()?,
            };
            ServerMessage::GivePrivilege(payload)
        }
        CODE_SM_INFORM_USER_OF_PRIVILEGES => {
            let payload = InformUserOfPrivilegesPayload {
                token: reader.read_u32()?,
                username: reader.read_string()?,
            };
            ServerMessage::InformUserOfPrivileges(payload)
        }
        CODE_SM_INFORM_USER_OF_PRIVILEGES_ACK => {
            let payload = InformUserOfPrivilegesAckPayload {
                token: reader.read_u32()?,
            };
            ServerMessage::InformUserOfPrivilegesAck(payload)
        }
        CODE_SM_GET_USER_RECOMMENDATIONS => {
            allow_trailing_bytes = true;
            if let Ok(request) = parse_user_lookup_payload(payload) {
                ServerMessage::GetUserRecommendations(request)
            } else {
                ServerMessage::GetUserRecommendationsResponse(parse_user_recommendations_payload(
                    payload,
                )?)
            }
        }
        CODE_SM_ADD_ROOM_MEMBER => {
            allow_trailing_bytes = true;
            ServerMessage::AddRoomMember(parse_room_moderation_payload(payload)?)
        }
        CODE_SM_REMOVE_ROOM_MEMBER => {
            allow_trailing_bytes = true;
            ServerMessage::RemoveRoomMember(parse_room_moderation_payload(payload)?)
        }
        CODE_SM_ADD_ROOM_OPERATOR => {
            allow_trailing_bytes = true;
            ServerMessage::AddRoomOperator(parse_room_moderation_payload(payload)?)
        }
        CODE_SM_REMOVE_ROOM_OPERATOR => {
            allow_trailing_bytes = true;
            ServerMessage::RemoveRoomOperator(parse_room_moderation_payload(payload)?)
        }
        CODE_SM_REMOVE_ROOM_OPERATORSHIP => {
            allow_trailing_bytes = true;
            ServerMessage::RemoveRoomOperatorship(OpaquePayload {
                bytes: payload.to_vec(),
            })
        }
        CODE_SM_REMOVE_OWN_ROOM_OPERATORSHIP => {
            allow_trailing_bytes = true;
            ServerMessage::RemoveOwnRoomOperatorship(OpaquePayload {
                bytes: payload.to_vec(),
            })
        }
        CODE_SM_ROOM_MEMBERS => {
            allow_trailing_bytes = true;
            ServerMessage::RoomMembers(parse_room_members_payload(payload)?)
        }
        CODE_SM_ROOM_OPERATORS => {
            allow_trailing_bytes = true;
            ServerMessage::RoomOperators(parse_room_operators_payload(payload)?)
        }
        CODE_SM_JOIN_GLOBAL_ROOM => {
            allow_trailing_bytes = true;
            ServerMessage::JoinGlobalRoom(OpaquePayload {
                bytes: payload.to_vec(),
            })
        }
        CODE_SM_LEAVE_GLOBAL_ROOM => {
            allow_trailing_bytes = true;
            ServerMessage::LeaveGlobalRoom(OpaquePayload {
                bytes: payload.to_vec(),
            })
        }
        CODE_SM_SAY_GLOBAL_ROOM => {
            allow_trailing_bytes = true;
            ServerMessage::SayGlobalRoom(OpaquePayload {
                bytes: payload.to_vec(),
            })
        }
        CODE_SM_SEARCH_CORRELATIONS => {
            allow_trailing_bytes = true;
            ServerMessage::SearchCorrelations(OpaquePayload {
                bytes: payload.to_vec(),
            })
        }
        CODE_SM_MESSAGE_USER => {
            allow_trailing_bytes = true;
            if let Ok(incoming) = parse_message_user_incoming_payload(payload) {
                ServerMessage::MessageUserIncoming(incoming)
            } else {
                let request = parse_message_user_request_payload(payload)?;
                ServerMessage::MessageUser(request)
            }
        }
        CODE_SM_MESSAGE_ACKED => {
            let payload = MessageAckedPayload {
                message_id: reader.read_u32()?,
            };
            ServerMessage::MessageAcked(payload)
        }
        CODE_SM_MESSAGE_USERS => {
            allow_trailing_bytes = true;
            ServerMessage::MessageUsers(parse_message_users_payload(payload)?)
        }
        CODE_SM_PEER_MESSAGE | CODE_SM_PEER_MESSAGE_ALT => {
            allow_trailing_bytes = true;
            ServerMessage::PeerMessage(parse_peer_message_payload(payload)?)
        }
        CODE_SM_GET_USER_STATS => {
            allow_trailing_bytes = true;
            if let Ok(request) = parse_user_lookup_payload(payload) {
                ServerMessage::GetUserStats(request)
            } else {
                ServerMessage::GetUserStatsResponse(parse_user_stats_response_payload(payload)?)
            }
        }
        CODE_SM_GET_USER_STATUS => {
            allow_trailing_bytes = true;
            if let Ok(request) = parse_user_lookup_payload(payload) {
                ServerMessage::GetUserStatus(request)
            } else {
                ServerMessage::GetUserStatusResponse(parse_user_status_response_payload(payload)?)
            }
        }
        CODE_SM_SHARED_FOLDERS_FILES => {
            let payload = SharedFoldersFilesPayload {
                folder_count: reader.read_u32()?,
                file_count: reader.read_u32()?,
            };
            ServerMessage::SharedFoldersFiles(payload)
        }
        CODE_SM_DOWNLOAD_SPEED => {
            let payload = SpeedPayload {
                bytes_per_sec: reader.read_u32()?,
            };
            ServerMessage::DownloadSpeed(payload)
        }
        CODE_SM_UPLOAD_SPEED => {
            let payload = SpeedPayload {
                bytes_per_sec: reader.read_u32()?,
            };
            ServerMessage::UploadSpeed(payload)
        }
        CODE_SM_DNET_LEVEL => {
            allow_trailing_bytes = true;
            ServerMessage::DnetLevel(OpaquePayload {
                bytes: payload.to_vec(),
            })
        }
        CODE_SM_DNET_GROUP_LEADER => {
            allow_trailing_bytes = true;
            ServerMessage::DnetGroupLeader(OpaquePayload {
                bytes: payload.to_vec(),
            })
        }
        CODE_SM_DNET_DELIVERY_REPORT => {
            allow_trailing_bytes = true;
            ServerMessage::DnetDeliveryReport(OpaquePayload {
                bytes: payload.to_vec(),
            })
        }
        CODE_SM_DNET_CHILD_DEPTH => {
            allow_trailing_bytes = true;
            ServerMessage::DnetChildDepth(OpaquePayload {
                bytes: payload.to_vec(),
            })
        }
        CODE_SM_FLOOD => {
            allow_trailing_bytes = true;
            ServerMessage::Flood(OpaquePayload {
                bytes: payload.to_vec(),
            })
        }
        other if is_opaque_server_control_code(other) => {
            allow_trailing_bytes = true;
            ServerMessage::OpaqueControl(OpaqueServerControlPayload {
                code: other,
                bytes: payload.to_vec(),
            })
        }
        other => bail!("unsupported server message code {other}"),
    };

    if !allow_trailing_bytes {
        ensure_payload_consumed(&reader)?;
    }
    Ok(message)
}

fn read_optional_string_list(reader: &mut PayloadReader<'_>, max_count: u32) -> Vec<String> {
    let checkpoint = reader.clone();
    let Ok(count) = reader.read_u32() else {
        return Vec::new();
    };

    if count > max_count {
        *reader = checkpoint;
        return Vec::new();
    }

    let mut entries = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let Ok(item) = reader.read_string() else {
            *reader = checkpoint;
            return Vec::new();
        };
        entries.push(item);
    }
    entries
}

pub fn parse_room_list_payload(payload: &[u8]) -> Result<RoomListPayload> {
    if payload.is_empty() {
        return Ok(RoomListPayload {
            room_count: 0,
            rooms: Vec::new(),
        });
    }

    let mut reader = PayloadReader::new(payload);
    let room_count = reader.read_u32()?;
    if room_count > 50_000 {
        bail!("room_count exceeds sanity threshold: {room_count}");
    }

    let mut rooms = Vec::with_capacity(room_count as usize);
    for _ in 0..room_count {
        rooms.push(reader.read_string()?);
    }

    Ok(RoomListPayload { room_count, rooms })
}

pub fn parse_join_room_payload(payload: &[u8]) -> Result<JoinRoomPayload> {
    let mut reader = PayloadReader::new(payload);
    let room = reader.read_string()?;
    let users = read_optional_string_list(&mut reader, 200_000);
    Ok(JoinRoomPayload { room, users })
}

pub fn parse_room_presence_event_payload(payload: &[u8]) -> Result<RoomPresenceEventPayload> {
    let mut reader = PayloadReader::new(payload);
    Ok(RoomPresenceEventPayload {
        room: reader.read_string()?,
        username: reader.read_string()?,
    })
}

pub fn parse_room_members_payload(payload: &[u8]) -> Result<RoomMembersPayload> {
    let mut reader = PayloadReader::new(payload);
    let room = reader.read_string()?;
    let users = read_optional_string_list(&mut reader, 300_000);
    Ok(RoomMembersPayload { room, users })
}

pub fn parse_room_operators_payload(payload: &[u8]) -> Result<RoomOperatorsPayload> {
    let mut reader = PayloadReader::new(payload);
    let room = reader.read_string()?;
    let operators = read_optional_string_list(&mut reader, 300_000);
    Ok(RoomOperatorsPayload { room, operators })
}

pub fn parse_room_moderation_payload(payload: &[u8]) -> Result<RoomModerationPayload> {
    let mut reader = PayloadReader::new(payload);
    let data = RoomModerationPayload {
        room: reader.read_string()?,
        username: reader.read_string()?,
    };
    ensure_payload_consumed(&reader)?;
    Ok(data)
}

pub fn parse_say_chatroom_payload(payload: &[u8]) -> Result<SayChatRoomPayload> {
    let mut reader = PayloadReader::new(payload);
    let room = reader.read_string()?;
    let second = reader.read_string()?;

    if reader.remaining() >= 4 {
        if let Ok(message) = reader.read_string() {
            return Ok(SayChatRoomPayload {
                room,
                username: Some(second),
                message,
            });
        }
    }

    Ok(SayChatRoomPayload {
        room,
        username: None,
        message: second,
    })
}

fn parse_recommendation_entries(
    reader: &mut PayloadReader<'_>,
    count: u32,
    max_count: u32,
) -> Result<Vec<RecommendationEntry>> {
    if count > max_count {
        bail!("recommendation_count exceeds sanity threshold: {count}");
    }

    let mut entries = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let term = reader.read_string()?;
        let score = if reader.remaining() >= 4 {
            reader.read_u32()? as i32
        } else {
            0
        };
        entries.push(RecommendationEntry { term, score });
    }
    Ok(entries)
}

fn parse_scored_user_entries(
    reader: &mut PayloadReader<'_>,
    count: u32,
    max_count: u32,
) -> Result<Vec<ScoredUserEntry>> {
    if count > max_count {
        bail!("scored_user_count exceeds sanity threshold: {count}");
    }

    let mut users = Vec::with_capacity(count as usize);
    for _ in 0..count {
        users.push(ScoredUserEntry {
            username: reader.read_string()?,
            score: reader.read_u32()? as i32,
        });
    }
    Ok(users)
}

pub fn parse_privileged_list_payload(payload: &[u8]) -> Result<PrivilegedListPayload> {
    if payload.is_empty() {
        return Ok(PrivilegedListPayload { users: Vec::new() });
    }

    let mut reader = PayloadReader::new(payload);
    let count = reader.read_u32()?;
    if count > 300_000 {
        bail!("privileged_user_count exceeds sanity threshold: {count}");
    }

    let mut users = Vec::with_capacity(count as usize);
    for _ in 0..count {
        users.push(reader.read_string()?);
    }
    ensure_payload_consumed(&reader)?;
    Ok(PrivilegedListPayload { users })
}

pub fn parse_recommended_users_payload(payload: &[u8]) -> Result<RecommendedUsersPayload> {
    let mut reader = PayloadReader::new(payload);
    let count = reader.read_u32()?;
    let users = parse_scored_user_entries(&mut reader, count, 100_000)?;
    ensure_payload_consumed(&reader)?;
    Ok(RecommendedUsersPayload { users })
}

pub fn parse_term_recommendations_payload(payload: &[u8]) -> Result<TermRecommendationsPayload> {
    let mut reader = PayloadReader::new(payload);
    let term = reader.read_string()?;
    let count = if reader.remaining() >= 4 {
        reader.read_u32()?
    } else {
        0
    };
    let recommendations = parse_recommendation_entries(&mut reader, count, 100_000)?;
    ensure_payload_consumed(&reader)?;
    Ok(TermRecommendationsPayload {
        term,
        recommendations,
    })
}

pub fn parse_recommendation_users_payload(payload: &[u8]) -> Result<RecommendationUsersPayload> {
    let mut reader = PayloadReader::new(payload);
    let term = reader.read_string()?;
    let count = if reader.remaining() >= 4 {
        reader.read_u32()?
    } else {
        0
    };
    let users = parse_scored_user_entries(&mut reader, count, 100_000)?;
    ensure_payload_consumed(&reader)?;
    Ok(RecommendationUsersPayload { term, users })
}

fn parse_recommendations_payload_from_reader(
    reader: &mut PayloadReader<'_>,
) -> Result<RecommendationsPayload> {
    if reader.remaining() == 0 {
        return Ok(RecommendationsPayload {
            recommendations: Vec::new(),
            unrecommendations: Vec::new(),
        });
    }

    let recommendation_count = reader.read_u32()?;
    let recommendations = parse_recommendation_entries(reader, recommendation_count, 100_000)?;
    let unrecommendation_count = if reader.remaining() >= 4 {
        reader.read_u32()?
    } else {
        0
    };
    let unrecommendations = parse_recommendation_entries(reader, unrecommendation_count, 100_000)?;

    Ok(RecommendationsPayload {
        recommendations,
        unrecommendations,
    })
}

pub fn parse_recommendations_payload(payload: &[u8]) -> Result<RecommendationsPayload> {
    let mut reader = PayloadReader::new(payload);
    parse_recommendations_payload_from_reader(&mut reader)
}

pub fn parse_similar_terms_request(payload: &[u8]) -> Result<SimilarTermsRequestPayload> {
    let mut reader = PayloadReader::new(payload);
    let request = SimilarTermsRequestPayload {
        term: reader.read_string()?,
    };
    ensure_payload_consumed(&reader)?;
    Ok(request)
}

pub fn parse_similar_terms_response(payload: &[u8]) -> Result<SimilarTermsPayload> {
    let mut reader = PayloadReader::new(payload);
    let term = reader.read_string()?;
    let count = if reader.remaining() >= 4 {
        reader.read_u32()?
    } else {
        0
    };
    let entries = parse_recommendation_entries(&mut reader, count, 100_000)?;
    Ok(SimilarTermsPayload { term, entries })
}

fn parse_user_lookup_payload(payload: &[u8]) -> Result<UserLookupPayload> {
    let mut reader = PayloadReader::new(payload);
    let request = UserLookupPayload {
        username: reader.read_string()?,
    };
    ensure_payload_consumed(&reader)?;
    Ok(request)
}

fn parse_peer_address_response_payload(payload: &[u8]) -> Result<PeerAddressResponsePayload> {
    let mut reader = PayloadReader::new(payload);
    let username = reader.read_string()?;
    let ip_address = Ipv4Addr::from(reader.read_u32()?.to_le_bytes()).to_string();
    let port = reader.read_u32()?;
    let obfuscation_type = reader.read_u32()?;
    let obfuscated_port_bytes = reader.take(2)?;
    let obfuscated_port = u16::from_le_bytes([obfuscated_port_bytes[0], obfuscated_port_bytes[1]]);
    ensure_payload_consumed(&reader)?;
    Ok(PeerAddressResponsePayload {
        username,
        ip_address,
        port,
        obfuscation_type,
        obfuscated_port,
    })
}

fn parse_user_status_response_payload(payload: &[u8]) -> Result<UserStatusResponsePayload> {
    let mut reader = PayloadReader::new(payload);
    let username = reader.read_string()?;
    let status = reader.read_u32()?;
    let privileged = if reader.remaining() >= 4 {
        reader.read_bool_u32()?
    } else if reader.remaining() >= 1 {
        reader.read_u8()? != 0
    } else {
        false
    };
    ensure_payload_consumed(&reader)?;
    Ok(UserStatusResponsePayload {
        username,
        status,
        privileged,
    })
}

fn parse_user_stats_response_payload(payload: &[u8]) -> Result<UserStatsResponsePayload> {
    let mut reader = PayloadReader::new(payload);
    let response = UserStatsResponsePayload {
        username: reader.read_string()?,
        avg_speed: reader.read_u32()?,
        download_num: reader.read_u32()?,
        files: reader.read_u32()?,
        dirs: reader.read_u32()?,
    };
    // Some server variants append extension counters after the canonical 4 fields.
    // Consume trailing u32 words so runtime decode remains forward-compatible.
    while reader.remaining() >= 4 {
        let _ = reader.read_u32()?;
    }
    ensure_payload_consumed(&reader)?;
    Ok(response)
}

fn parse_connect_to_peer_request_payload(payload: &[u8]) -> Result<ConnectToPeerRequestPayload> {
    let mut reader = PayloadReader::new(payload);
    let request = ConnectToPeerRequestPayload {
        token: reader.read_u32()?,
        username: reader.read_string()?,
        connection_type: reader.read_string()?,
    };
    ensure_payload_consumed(&reader)?;
    Ok(request)
}

fn parse_connect_to_peer_response_payload(payload: &[u8]) -> Result<ConnectToPeerResponsePayload> {
    let mut reader = PayloadReader::new(payload);
    let username = reader.read_string()?;
    let connection_type = reader.read_string()?;
    let ip_address = Ipv4Addr::from(reader.read_u32()?.to_le_bytes()).to_string();
    let port = reader.read_u32()?;
    let token = reader.read_u32()?;
    let mut privileged = false;
    let mut obfuscation_type = 0;
    let mut obfuscated_port = 0;

    let tail_len = reader.remaining();
    if tail_len > 0 {
        if tail_len >= 9 && tail_len % 4 == 1 {
            privileged = reader.read_u8()? != 0;
            if reader.remaining() >= 4 {
                obfuscation_type = reader.read_u32()?;
            }
            if reader.remaining() >= 4 {
                obfuscated_port = reader.read_u32()?;
            }
        } else if tail_len == 8 {
            // Some variants omit privileged and include only obfuscation fields.
            obfuscation_type = reader.read_u32()?;
            obfuscated_port = reader.read_u32()?;
        } else if tail_len >= 4 && tail_len % 4 == 0 {
            privileged = reader.read_bool_u32()?;
            if reader.remaining() >= 4 {
                obfuscation_type = reader.read_u32()?;
            }
            if reader.remaining() >= 4 {
                obfuscated_port = reader.read_u32()?;
            }
        } else if tail_len == 1 {
            privileged = reader.read_u8()? != 0;
        } else {
            bail!("unexpected connect_to_peer tail length: {tail_len}");
        }
    }

    // Consume extension words while preserving canonical fields.
    while reader.remaining() >= 4 {
        let _ = reader.read_u32()?;
    }
    ensure_payload_consumed(&reader)?;
    Ok(ConnectToPeerResponsePayload {
        username,
        connection_type,
        ip_address,
        port,
        token,
        privileged,
        obfuscation_type,
        obfuscated_port,
    })
}

fn parse_connect_to_peer_legacy_payload(payload: &[u8]) -> Result<ConnectToPeerPayload> {
    let mut reader = PayloadReader::new(payload);
    let legacy = ConnectToPeerPayload {
        username: reader.read_string()?,
        token: reader.read_u32()?,
    };
    ensure_payload_consumed(&reader)?;
    Ok(legacy)
}

fn parse_message_user_request_payload(payload: &[u8]) -> Result<MessageUserPayload> {
    let mut reader = PayloadReader::new(payload);
    let request = MessageUserPayload {
        username: reader.read_string()?,
        message: reader.read_string()?,
    };
    ensure_payload_consumed(&reader)?;
    Ok(request)
}

fn parse_message_user_incoming_payload(payload: &[u8]) -> Result<MessageUserIncomingPayload> {
    let mut reader = PayloadReader::new(payload);
    let incoming = MessageUserIncomingPayload {
        message_id: reader.read_u32()?,
        timestamp: reader.read_u32()?,
        username: reader.read_string()?,
        message: reader.read_string()?,
        is_new: if reader.remaining() >= 1 {
            reader.read_u8()? != 0
        } else {
            false
        },
    };
    ensure_payload_consumed(&reader)?;
    Ok(incoming)
}

fn parse_message_users_payload(payload: &[u8]) -> Result<MessageUsersPayload> {
    let mut reader = PayloadReader::new(payload);
    let user_count = reader.read_u32()?;
    if user_count > 50_000 {
        bail!("message_users count exceeds sanity threshold: {user_count}");
    }
    let mut usernames = Vec::with_capacity(user_count as usize);
    for _ in 0..user_count {
        usernames.push(reader.read_string()?);
    }
    let message = reader.read_string()?;
    ensure_payload_consumed(&reader)?;
    Ok(MessageUsersPayload { usernames, message })
}

fn parse_peer_message_request_payload(payload: &[u8]) -> Result<PeerMessagePayload> {
    let mut reader = PayloadReader::new(payload);
    let username = reader.read_string()?;
    let token = reader.read_u32()?;
    let code = reader.read_u32()?;
    let message = reader.read_string()?;
    ensure_payload_consumed(&reader)?;
    Ok(PeerMessagePayload {
        username,
        message,
        token: Some(token),
        code: Some(code),
        ip_address: None,
        port: None,
    })
}

fn parse_peer_message_response_payload(payload: &[u8]) -> Result<PeerMessagePayload> {
    let mut reader = PayloadReader::new(payload);
    let username = reader.read_string()?;
    let code = reader.read_u32()?;
    let token = reader.read_u32()?;
    let ip_address = Ipv4Addr::from(reader.read_u32()?.to_le_bytes()).to_string();
    let port = reader.read_u32()?;
    let message = reader.read_string()?;
    ensure_payload_consumed(&reader)?;
    Ok(PeerMessagePayload {
        username,
        message,
        token: Some(token),
        code: Some(code),
        ip_address: Some(ip_address),
        port: Some(port),
    })
}

fn parse_peer_message_payload(payload: &[u8]) -> Result<PeerMessagePayload> {
    if let Ok(response) = parse_peer_message_response_payload(payload) {
        return Ok(response);
    }
    parse_peer_message_request_payload(payload)
}

fn parse_own_privileges_status_payload(payload: &[u8]) -> Result<OwnPrivilegesStatusPayload> {
    let mut reader = PayloadReader::new(payload);
    let status = OwnPrivilegesStatusPayload {
        time_left_seconds: reader.read_u32()?,
    };
    ensure_payload_consumed(&reader)?;
    Ok(status)
}

fn parse_user_privileges_status_payload(payload: &[u8]) -> Result<UserPrivilegesStatusPayload> {
    let mut reader = PayloadReader::new(payload);
    let status = UserPrivilegesStatusPayload {
        username: reader.read_string()?,
        privileged: if reader.remaining() >= 4 {
            reader.read_bool_u32()?
        } else if reader.remaining() >= 1 {
            reader.read_u8()? != 0
        } else {
            false
        },
    };
    ensure_payload_consumed(&reader)?;
    Ok(status)
}

pub fn parse_user_recommendations_payload(payload: &[u8]) -> Result<UserRecommendationsPayload> {
    let mut reader = PayloadReader::new(payload);
    let username = reader.read_string()?;
    let recommendations = parse_recommendations_payload_from_reader(&mut reader)?;
    Ok(UserRecommendationsPayload {
        username,
        recommendations,
    })
}

fn decode_login_request_payload(payload: &[u8]) -> Result<LoginRequestPayload> {
    let mut reader = PayloadReader::new(payload);
    let request = LoginRequestPayload {
        username: reader.read_string()?,
        password: reader.read_string()?,
        client_version: reader.read_u32()?,
        md5hash: reader.read_string()?,
        minor_version: reader.read_u32()?,
    };
    ensure_payload_consumed(&reader)?;
    Ok(request)
}

pub fn compute_login_md5hash(username: &str, password: &str) -> String {
    let digest = md5::compute(format!("{username}{password}"));
    format!("{digest:x}")
}

pub fn parse_login_response(payload: &[u8]) -> Result<LoginResponsePayload> {
    let mut reader = PayloadReader::new(payload);
    let accepted = reader.read_u8()? != 0;

    if accepted {
        let greeting = reader.read_string()?;
        let ip_raw = reader.read_u32()?;
        let md5hash = reader.read_string()?;
        let is_supporter = if reader.remaining() >= 1 {
            reader.read_u8()? != 0
        } else {
            false
        };
        Ok(LoginResponsePayload::Success(LoginResponseSuccessPayload {
            greeting,
            ip_address: Ipv4Addr::from(ip_raw.to_le_bytes()).to_string(),
            md5hash,
            is_supporter,
        }))
    } else {
        let reason_wire = reader.read_string()?;
        let detail = if reader.remaining() >= 4 {
            Some(reader.read_string()?)
        } else {
            None
        };
        Ok(LoginResponsePayload::Failure(LoginResponseFailurePayload {
            reason: LoginFailureReason::from_wire_str(&reason_wire),
            detail,
        }))
    }
}

pub fn parse_search_response_summary(payload: &[u8]) -> Result<SearchResponseSummary> {
    if let Ok(summary) = parse_search_response_summary_v1(payload) {
        return Ok(summary);
    }

    parse_search_response_summary_room_list(payload)
}

fn parse_search_response_summary_v1(payload: &[u8]) -> Result<SearchResponseSummary> {
    let mut reader = PayloadReader::new(payload);
    let username = reader.read_string()?;
    let token = reader.read_u32()?;
    let files_count = reader.read_u32()?;
    if files_count > 10_000 {
        bail!("files_count exceeds sanity threshold: {files_count}");
    }

    let mut files = Vec::with_capacity(files_count as usize);
    for _ in 0..files_count {
        files.push(SearchFileSummary {
            file_path: reader.read_string()?,
            file_size: reader.read_u64()?,
            extension: reader.read_string()?,
            attr_count: reader.read_u32()?,
        });
    }

    let slots_free = reader.read_u32()?;
    let speed = reader.read_u32()?;
    let in_queue = if reader.remaining() >= 4 {
        reader.read_bool_u32()?
    } else if reader.remaining() >= 1 {
        reader.read_u8()? != 0
    } else {
        false
    };

    Ok(SearchResponseSummary {
        username,
        token,
        files_count,
        slots_free,
        speed,
        in_queue,
        files,
    })
}

fn parse_search_response_summary_room_list(payload: &[u8]) -> Result<SearchResponseSummary> {
    let mut reader = PayloadReader::new(payload);
    let room_count = reader.read_u32()?;
    if room_count > 20_000 {
        bail!("room_count exceeds sanity threshold: {room_count}");
    }

    let mut files = Vec::with_capacity(room_count.min(64) as usize);
    for idx in 0..room_count {
        let room_name = reader.read_string()?;
        if idx < 64 {
            files.push(SearchFileSummary {
                file_path: room_name,
                file_size: 0,
                extension: "room".to_string(),
                attr_count: 0,
            });
        }
    }

    Ok(SearchResponseSummary {
        username: "<server_room_list>".to_string(),
        token: 0,
        files_count: room_count,
        slots_free: 0,
        speed: 0,
        in_queue: false,
        files,
    })
}

fn parse_user_info_reply_payload(payload: &[u8]) -> Result<UserInfoReplyPayload> {
    let mut reader = PayloadReader::new(payload);
    let description = reader.read_string()?;
    let has_picture = if reader.remaining() >= 1 {
        reader.read_u8()? != 0
    } else {
        false
    };
    let picture = if has_picture && reader.remaining() >= 4 {
        reader.read_bytes()?
    } else {
        Vec::new()
    };
    let total_uploads = if reader.remaining() >= 4 {
        reader.read_u32()?
    } else {
        0
    };
    let queue_size = if reader.remaining() >= 4 {
        reader.read_u32()?
    } else {
        0
    };
    let slots_free = if reader.remaining() >= 1 {
        reader.read_u8()? != 0
    } else {
        false
    };
    let upload_permissions = if reader.remaining() >= 4 {
        Some(reader.read_u32()?)
    } else {
        None
    };

    Ok(UserInfoReplyPayload {
        description,
        has_picture,
        picture,
        total_uploads,
        queue_size,
        slots_free,
        upload_permissions,
    })
}

fn parse_peer_search_query_payload(payload: &[u8]) -> Result<PeerSearchQueryPayload> {
    let mut reader = PayloadReader::new(payload);
    if reader.remaining() >= 8 {
        let checkpoint = reader.clone();
        if let Ok(token) = reader.read_u32() {
            if let Ok(query) = reader.read_string() {
                if reader.remaining() == 0 {
                    return Ok(PeerSearchQueryPayload {
                        token: Some(token),
                        query,
                    });
                }
            }
        }
        reader = checkpoint;
    }

    let query = reader.read_string()?;
    ensure_payload_consumed(&reader)?;
    Ok(PeerSearchQueryPayload { token: None, query })
}

fn parse_queued_downloads_payload(payload: &[u8]) -> Result<PeerQueuedDownloadsPayload> {
    if payload.is_empty() {
        return Ok(PeerQueuedDownloadsPayload {
            virtual_paths: Vec::new(),
        });
    }

    let mut reader = PayloadReader::new(payload);
    let checkpoint = reader.clone();

    if let Ok(count) = reader.read_u32() {
        if count <= 100_000 {
            let mut virtual_paths = Vec::with_capacity(count as usize);
            let mut ok = true;
            for _ in 0..count {
                match reader.read_string() {
                    Ok(path) => virtual_paths.push(path),
                    Err(_) => {
                        ok = false;
                        break;
                    }
                }
            }

            if ok && reader.remaining() == 0 {
                return Ok(PeerQueuedDownloadsPayload { virtual_paths });
            }
        }
    }

    reader = checkpoint;
    let single = reader.read_string()?;
    ensure_payload_consumed(&reader)?;
    Ok(PeerQueuedDownloadsPayload {
        virtual_paths: vec![single],
    })
}

pub fn encode_peer_message(message: &PeerMessage) -> Frame {
    let mut writer = PayloadWriter::new();
    let code = match message {
        PeerMessage::Say(payload) => {
            writer.write_raw_bytes(&payload.bytes);
            CODE_PM_SAY
        }
        PeerMessage::GetSharedFileList(payload) => {
            writer.write_string(&payload.username);
            CODE_PM_GET_SHARED_FILE_LIST
        }
        PeerMessage::SharedFileList(payload) => {
            writer.write_u32(payload.entries.len() as u32);
            for entry in &payload.entries {
                writer.write_string(&entry.virtual_path);
                writer.write_u64(entry.size);
            }
            CODE_PM_SHARED_FILE_LIST
        }
        PeerMessage::GetSharedFilesInFolder(payload) => {
            writer.write_string(&payload.directory);
            CODE_PM_GET_SHARED_FILES_IN_FOLDER
        }
        PeerMessage::SharedFilesInFolder(payload) => {
            writer.write_string(&payload.directory);
            writer.write_raw_bytes(&payload.compressed_listing);
            CODE_PM_SHARED_FILES_IN_FOLDER
        }
        PeerMessage::FileSearchRequest(payload) => {
            writer.write_u32(payload.token);
            writer.write_string(&payload.query);
            CODE_PM_FILE_SEARCH_REQUEST
        }
        PeerMessage::FileSearchResult(payload) => {
            writer.write_u32(payload.token);
            writer.write_string(&payload.username);
            writer.write_u32(payload.result_count);
            CODE_PM_FILE_SEARCH_RESULT
        }
        PeerMessage::InviteUserToRoom(payload) => {
            writer.write_string(&payload.room);
            CODE_PM_INVITE_USER_TO_ROOM
        }
        PeerMessage::CancelledQueuedTransfer(payload) => {
            writer.write_string(&payload.virtual_path);
            CODE_PM_CANCELLED_QUEUED_TRANSFER
        }
        PeerMessage::UserInfoRequest(_) => CODE_PM_USER_INFO_REQUEST,
        PeerMessage::UserInfoReply(payload) => {
            writer.write_string(&payload.description);
            writer.write_u8(u8::from(payload.has_picture));
            if payload.has_picture {
                writer.write_bytes(&payload.picture);
            }
            writer.write_u32(payload.total_uploads);
            writer.write_u32(payload.queue_size);
            writer.write_u8(u8::from(payload.slots_free));
            if let Some(value) = payload.upload_permissions {
                writer.write_u32(value);
            }
            CODE_PM_USER_INFO_REPLY
        }
        PeerMessage::SendConnectToken(payload) => {
            writer.write_raw_bytes(&payload.bytes);
            CODE_PM_SEND_CONNECT_TOKEN
        }
        PeerMessage::TransferRequest(payload) => {
            writer.write_u32(payload.direction.as_u32());
            writer.write_u32(payload.token);
            writer.write_string(&payload.virtual_path);
            writer.write_u64(payload.file_size);
            CODE_PM_TRANSFER_REQUEST
        }
        PeerMessage::TransferResponse(payload) => {
            writer.write_u32(payload.token);
            writer.write_bool_u32(payload.allowed);
            writer.write_string(&payload.queue_or_reason);
            CODE_PM_TRANSFER_RESPONSE
        }
        PeerMessage::PlaceholderUpload(payload) => {
            writer.write_raw_bytes(&payload.bytes);
            CODE_PM_PLACEHOLD_UPLOAD
        }
        PeerMessage::QueueUpload(payload) => {
            writer.write_string(&payload.username);
            writer.write_string(&payload.virtual_path);
            CODE_PM_QUEUE_UPLOAD
        }
        PeerMessage::UploadPlaceInLine(payload) => {
            writer.write_string(&payload.username);
            writer.write_string(&payload.virtual_path);
            writer.write_u32(payload.place);
            CODE_PM_UPLOAD_PLACE_IN_LINE
        }
        PeerMessage::MoveDownloadToTop(payload) => {
            writer.write_string(&payload.virtual_path);
            CODE_PM_MOVE_DOWNLOAD_TO_TOP
        }
        PeerMessage::ExactFileSearchRequest(payload) => {
            if let Some(token) = payload.token {
                writer.write_u32(token);
            }
            writer.write_string(&payload.query);
            CODE_PM_EXACT_FILE_SEARCH_REQUEST
        }
        PeerMessage::QueuedDownloads(payload) => {
            writer.write_u32(payload.virtual_paths.len() as u32);
            for virtual_path in &payload.virtual_paths {
                writer.write_string(virtual_path);
            }
            CODE_PM_QUEUED_DOWNLOADS
        }
        PeerMessage::IndirectFileSearchRequest(payload) => {
            if let Some(token) = payload.token {
                writer.write_u32(token);
            }
            writer.write_string(&payload.query);
            CODE_PM_INDIRECT_FILE_SEARCH_REQUEST
        }
        PeerMessage::UploadFailed(payload) => {
            writer.write_string(&payload.username);
            writer.write_string(&payload.virtual_path);
            writer.write_string(&payload.reason);
            CODE_PM_UPLOAD_FAILED
        }
        PeerMessage::UploadDenied(payload) => {
            writer.write_string(&payload.username);
            writer.write_string(&payload.virtual_path);
            writer.write_string(&payload.reason);
            CODE_PM_UPLOAD_DENIED
        }
        PeerMessage::UploadPlaceInLineRequest(payload) => {
            writer.write_string(&payload.virtual_path);
            CODE_PM_UPLOAD_PLACE_IN_LINE_REQUEST
        }
        PeerMessage::Nothing(payload) => {
            writer.write_raw_bytes(&payload.bytes);
            CODE_PM_NOTHING
        }
    };

    Frame::new(code, writer.into_inner())
}

pub fn decode_peer_message(code: u32, payload: &[u8]) -> Result<PeerMessage> {
    let mut reader = PayloadReader::new(payload);
    let mut allow_trailing_bytes = false;

    let message = match code {
        CODE_PM_SAY => {
            allow_trailing_bytes = true;
            PeerMessage::Say(OpaquePayload {
                bytes: payload.to_vec(),
            })
        }
        CODE_PM_GET_SHARED_FILE_LIST => {
            let payload = UserLookupPayload {
                username: reader.read_string()?,
            };
            PeerMessage::GetSharedFileList(payload)
        }
        CODE_PM_SHARED_FILE_LIST => {
            let count = reader.read_u32()? as usize;
            let mut entries = Vec::with_capacity(count);
            for _ in 0..count {
                entries.push(SharedFileEntry {
                    virtual_path: reader.read_string()?,
                    size: reader.read_u64()?,
                });
            }
            PeerMessage::SharedFileList(SharedFileListPayload { entries })
        }
        CODE_PM_GET_SHARED_FILES_IN_FOLDER => {
            let payload = SharedFilesInFolderRequestPayload {
                directory: reader.read_string()?,
            };
            PeerMessage::GetSharedFilesInFolder(payload)
        }
        CODE_PM_SHARED_FILES_IN_FOLDER => {
            let payload = SharedFilesInFolderPayload {
                directory: reader.read_string()?,
                compressed_listing: reader.read_remaining_bytes(),
            };
            PeerMessage::SharedFilesInFolder(payload)
        }
        CODE_PM_FILE_SEARCH_REQUEST => {
            let payload = FileSearchRequestPayload {
                token: reader.read_u32()?,
                query: reader.read_string()?,
            };
            PeerMessage::FileSearchRequest(payload)
        }
        CODE_PM_FILE_SEARCH_RESULT => {
            let payload = FileSearchResultPayload {
                token: reader.read_u32()?,
                username: reader.read_string()?,
                result_count: reader.read_u32()?,
            };
            PeerMessage::FileSearchResult(payload)
        }
        CODE_PM_INVITE_USER_TO_ROOM => {
            let payload = PeerRoomInvitePayload {
                room: reader.read_string()?,
            };
            PeerMessage::InviteUserToRoom(payload)
        }
        CODE_PM_CANCELLED_QUEUED_TRANSFER => {
            let payload = PeerVirtualPathPayload {
                virtual_path: reader.read_string()?,
            };
            PeerMessage::CancelledQueuedTransfer(payload)
        }
        CODE_PM_USER_INFO_REQUEST => PeerMessage::UserInfoRequest(UserInfoRequestPayload),
        CODE_PM_USER_INFO_REPLY => {
            allow_trailing_bytes = true;
            PeerMessage::UserInfoReply(parse_user_info_reply_payload(payload)?)
        }
        CODE_PM_SEND_CONNECT_TOKEN => {
            allow_trailing_bytes = true;
            PeerMessage::SendConnectToken(OpaquePayload {
                bytes: payload.to_vec(),
            })
        }
        CODE_PM_TRANSFER_REQUEST => {
            let direction = TransferDirection::from_u32(reader.read_u32()?)?;
            let payload = TransferRequestPayload {
                direction,
                token: reader.read_u32()?,
                virtual_path: reader.read_string()?,
                file_size: reader.read_u64()?,
            };
            PeerMessage::TransferRequest(payload)
        }
        CODE_PM_TRANSFER_RESPONSE => {
            let payload = TransferResponsePayload {
                token: reader.read_u32()?,
                allowed: reader.read_bool_u32()?,
                queue_or_reason: reader.read_string()?,
            };
            PeerMessage::TransferResponse(payload)
        }
        CODE_PM_PLACEHOLD_UPLOAD => {
            allow_trailing_bytes = true;
            PeerMessage::PlaceholderUpload(OpaquePayload {
                bytes: payload.to_vec(),
            })
        }
        CODE_PM_QUEUE_UPLOAD => {
            let payload = QueueUploadPayload {
                username: reader.read_string()?,
                virtual_path: reader.read_string()?,
            };
            PeerMessage::QueueUpload(payload)
        }
        CODE_PM_UPLOAD_PLACE_IN_LINE => {
            let payload = UploadPlaceInLinePayload {
                username: reader.read_string()?,
                virtual_path: reader.read_string()?,
                place: reader.read_u32()?,
            };
            PeerMessage::UploadPlaceInLine(payload)
        }
        CODE_PM_MOVE_DOWNLOAD_TO_TOP => {
            let payload = PeerVirtualPathPayload {
                virtual_path: reader.read_string()?,
            };
            PeerMessage::MoveDownloadToTop(payload)
        }
        CODE_PM_EXACT_FILE_SEARCH_REQUEST => {
            allow_trailing_bytes = true;
            PeerMessage::ExactFileSearchRequest(parse_peer_search_query_payload(payload)?)
        }
        CODE_PM_QUEUED_DOWNLOADS => {
            allow_trailing_bytes = true;
            PeerMessage::QueuedDownloads(parse_queued_downloads_payload(payload)?)
        }
        CODE_PM_INDIRECT_FILE_SEARCH_REQUEST => {
            allow_trailing_bytes = true;
            PeerMessage::IndirectFileSearchRequest(parse_peer_search_query_payload(payload)?)
        }
        CODE_PM_UPLOAD_FAILED => {
            let payload = UploadStatusPayload {
                username: reader.read_string()?,
                virtual_path: reader.read_string()?,
                reason: reader.read_string()?,
            };
            PeerMessage::UploadFailed(payload)
        }
        CODE_PM_UPLOAD_DENIED => {
            let payload = UploadStatusPayload {
                username: reader.read_string()?,
                virtual_path: reader.read_string()?,
                reason: reader.read_string()?,
            };
            PeerMessage::UploadDenied(payload)
        }
        CODE_PM_UPLOAD_PLACE_IN_LINE_REQUEST => {
            let payload = UploadPlaceInLineRequestPayload {
                virtual_path: reader.read_string()?,
            };
            PeerMessage::UploadPlaceInLineRequest(payload)
        }
        CODE_PM_NOTHING => {
            allow_trailing_bytes = true;
            PeerMessage::Nothing(OpaquePayload {
                bytes: payload.to_vec(),
            })
        }
        other => bail!("unsupported peer message code {other}"),
    };

    if !allow_trailing_bytes {
        ensure_payload_consumed(&reader)?;
    }
    Ok(message)
}

pub fn build_login_request(
    username: &str,
    password: &str,
    client_version: u32,
    minor_version: u32,
) -> Frame {
    let md5hash = compute_login_md5hash(username, password);
    encode_server_message(&ServerMessage::Login(LoginPayload {
        username: username.to_owned(),
        password: password.to_owned(),
        client_version,
        md5hash,
        minor_version,
    }))
}

pub fn build_file_search_request(token: u32, search_text: &str) -> Frame {
    encode_server_message(&ServerMessage::FileSearch(FileSearchPayload {
        search_token: token,
        search_text: search_text.to_owned(),
    }))
}

pub fn build_opaque_server_control_request(code: u32, payload: &[u8]) -> Result<Frame> {
    if !is_opaque_server_control_code(code) {
        bail!("unsupported opaque server control code: {code}");
    }
    Ok(encode_server_message(&ServerMessage::OpaqueControl(
        OpaqueServerControlPayload {
            code,
            bytes: payload.to_vec(),
        },
    )))
}

pub fn build_get_peer_address_request(username: &str) -> Frame {
    encode_server_message(&ServerMessage::GetPeerAddress(UserLookupPayload {
        username: username.to_owned(),
    }))
}

pub fn build_get_user_status_request(username: &str) -> Frame {
    encode_server_message(&ServerMessage::GetUserStatus(UserLookupPayload {
        username: username.to_owned(),
    }))
}

pub fn build_get_user_stats_request(username: &str) -> Frame {
    encode_server_message(&ServerMessage::GetUserStats(UserLookupPayload {
        username: username.to_owned(),
    }))
}

pub fn build_connect_to_peer_request(token: u32, username: &str, connection_type: &str) -> Frame {
    encode_server_message(&ServerMessage::ConnectToPeerRequest(
        ConnectToPeerRequestPayload {
            token,
            username: username.to_owned(),
            connection_type: connection_type.to_owned(),
        },
    ))
}

pub fn build_message_user_request(username: &str, message: &str) -> Frame {
    encode_server_message(&ServerMessage::MessageUser(MessageUserPayload {
        username: username.to_owned(),
        message: message.to_owned(),
    }))
}

pub fn build_message_users_request(usernames: &[String], message: &str) -> Frame {
    encode_server_message(&ServerMessage::MessageUsers(MessageUsersPayload {
        usernames: usernames.to_vec(),
        message: message.to_owned(),
    }))
}

pub fn build_get_recommendations_request() -> Frame {
    encode_server_message(&ServerMessage::GetRecommendations(EmptyPayload))
}

pub fn build_get_my_recommendations_request() -> Frame {
    encode_server_message(&ServerMessage::GetMyRecommendations(EmptyPayload))
}

pub fn build_get_global_recommendations_request() -> Frame {
    encode_server_message(&ServerMessage::GetGlobalRecommendations(EmptyPayload))
}

pub fn build_get_user_recommendations_request(username: &str) -> Frame {
    encode_server_message(&ServerMessage::GetUserRecommendations(UserLookupPayload {
        username: username.to_owned(),
    }))
}

pub fn build_privileged_list_request() -> Frame {
    Frame::new(CODE_SM_PRIVILEGED_LIST, Vec::new())
}

pub fn build_get_recommended_users_request() -> Frame {
    Frame::new(CODE_SM_GET_RECOMMENDED_USERS, Vec::new())
}

pub fn build_get_term_recommendations_request(term: &str) -> Frame {
    encode_server_message(&ServerMessage::GetTermRecommendations(
        SimilarTermsRequestPayload {
            term: term.to_owned(),
        },
    ))
}

pub fn build_get_recommendation_users_request(term: &str) -> Frame {
    encode_server_message(&ServerMessage::GetRecommendationUsers(
        SimilarTermsRequestPayload {
            term: term.to_owned(),
        },
    ))
}

pub fn build_get_similar_terms_request(term: &str) -> Frame {
    encode_server_message(&ServerMessage::GetSimilarTerms(
        SimilarTermsRequestPayload {
            term: term.to_owned(),
        },
    ))
}

pub fn build_ignore_user_request(username: &str) -> Frame {
    encode_server_message(&ServerMessage::IgnoreUser(UserLookupPayload {
        username: username.to_owned(),
    }))
}

pub fn build_unignore_user_request(username: &str) -> Frame {
    encode_server_message(&ServerMessage::UnignoreUser(UserLookupPayload {
        username: username.to_owned(),
    }))
}

pub fn build_ban_user_request(username: &str) -> Frame {
    encode_server_message(&ServerMessage::BanUser(UserLookupPayload {
        username: username.to_owned(),
    }))
}

pub fn build_get_own_privileges_status_request() -> Frame {
    encode_server_message(&ServerMessage::GetOwnPrivilegesStatus(EmptyPayload))
}

pub fn build_get_user_privileges_status_request(username: &str) -> Frame {
    encode_server_message(&ServerMessage::GetUserPrivilegesStatus(UserLookupPayload {
        username: username.to_owned(),
    }))
}

pub fn build_give_privilege_request(username: &str, days: u32) -> Frame {
    encode_server_message(&ServerMessage::GivePrivilege(GivePrivilegePayload {
        username: username.to_owned(),
        days,
    }))
}

pub fn build_inform_user_of_privileges_request(token: u32, username: &str) -> Frame {
    encode_server_message(&ServerMessage::InformUserOfPrivileges(
        InformUserOfPrivilegesPayload {
            token,
            username: username.to_owned(),
        },
    ))
}

pub fn build_inform_user_of_privileges_ack_request(token: u32) -> Frame {
    encode_server_message(&ServerMessage::InformUserOfPrivilegesAck(
        InformUserOfPrivilegesAckPayload { token },
    ))
}

pub fn build_room_list_request() -> Frame {
    Frame::new(CODE_SM_ROOM_LIST, Vec::new())
}

pub fn build_join_room_request(room: &str) -> Frame {
    encode_server_message(&ServerMessage::JoinRoom(JoinRoomPayload {
        room: room.to_owned(),
        users: Vec::new(),
    }))
}

pub fn build_leave_room_request(room: &str) -> Frame {
    encode_server_message(&ServerMessage::LeaveRoom(LeaveRoomPayload {
        room: room.to_owned(),
    }))
}

pub fn build_room_members_request(room: &str) -> Frame {
    encode_server_message(&ServerMessage::RoomMembers(RoomMembersPayload {
        room: room.to_owned(),
        users: Vec::new(),
    }))
}

pub fn build_room_operators_request(room: &str) -> Frame {
    encode_server_message(&ServerMessage::RoomOperators(RoomOperatorsPayload {
        room: room.to_owned(),
        operators: Vec::new(),
    }))
}

pub fn build_add_room_member_request(room: &str, username: &str) -> Frame {
    encode_server_message(&ServerMessage::AddRoomMember(RoomModerationPayload {
        room: room.to_owned(),
        username: username.to_owned(),
    }))
}

pub fn build_remove_room_member_request(room: &str, username: &str) -> Frame {
    encode_server_message(&ServerMessage::RemoveRoomMember(RoomModerationPayload {
        room: room.to_owned(),
        username: username.to_owned(),
    }))
}

pub fn build_add_room_operator_request(room: &str, username: &str) -> Frame {
    encode_server_message(&ServerMessage::AddRoomOperator(RoomModerationPayload {
        room: room.to_owned(),
        username: username.to_owned(),
    }))
}

pub fn build_remove_room_operator_request(room: &str, username: &str) -> Frame {
    encode_server_message(&ServerMessage::RemoveRoomOperator(RoomModerationPayload {
        room: room.to_owned(),
        username: username.to_owned(),
    }))
}

pub fn build_say_chatroom(room: &str, message: &str) -> Frame {
    encode_server_message(&ServerMessage::SayChatRoom(SayChatRoomPayload {
        room: room.to_owned(),
        username: None,
        message: message.to_owned(),
    }))
}

pub fn build_transfer_request(
    direction: TransferDirection,
    token: u32,
    virtual_path: &str,
    file_size: u64,
) -> Frame {
    encode_peer_message(&PeerMessage::TransferRequest(TransferRequestPayload {
        direction,
        token,
        virtual_path: virtual_path.to_owned(),
        file_size,
    }))
}

pub fn build_transfer_response(token: u32, allowed: bool, queue_or_reason: &str) -> Frame {
    encode_peer_message(&PeerMessage::TransferResponse(TransferResponsePayload {
        token,
        allowed,
        queue_or_reason: queue_or_reason.to_owned(),
    }))
}

pub fn build_user_info_request() -> Frame {
    encode_peer_message(&PeerMessage::UserInfoRequest(UserInfoRequestPayload))
}

pub fn build_exact_file_search_request(query: &str, token: Option<u32>) -> Frame {
    encode_peer_message(&PeerMessage::ExactFileSearchRequest(
        PeerSearchQueryPayload {
            token,
            query: query.to_owned(),
        },
    ))
}

pub fn build_peer_invite_user_to_room(room: &str) -> Frame {
    encode_peer_message(&PeerMessage::InviteUserToRoom(PeerRoomInvitePayload {
        room: room.to_owned(),
    }))
}

pub fn build_peer_cancelled_queued_transfer(virtual_path: &str) -> Frame {
    encode_peer_message(&PeerMessage::CancelledQueuedTransfer(
        PeerVirtualPathPayload {
            virtual_path: virtual_path.to_owned(),
        },
    ))
}

pub fn build_peer_move_download_to_top(virtual_path: &str) -> Frame {
    encode_peer_message(&PeerMessage::MoveDownloadToTop(PeerVirtualPathPayload {
        virtual_path: virtual_path.to_owned(),
    }))
}

pub fn build_peer_queued_downloads(virtual_paths: &[String]) -> Frame {
    encode_peer_message(&PeerMessage::QueuedDownloads(PeerQueuedDownloadsPayload {
        virtual_paths: virtual_paths.to_vec(),
    }))
}

pub fn build_indirect_file_search_request(query: &str, token: Option<u32>) -> Frame {
    encode_peer_message(&PeerMessage::IndirectFileSearchRequest(
        PeerSearchQueryPayload {
            token,
            query: query.to_owned(),
        },
    ))
}

pub fn build_upload_place_in_line_request(virtual_path: &str) -> Frame {
    encode_peer_message(&PeerMessage::UploadPlaceInLineRequest(
        UploadPlaceInLineRequestPayload {
            virtual_path: virtual_path.to_owned(),
        },
    ))
}

pub fn build_get_shared_files_in_folder_request(directory: &str) -> Frame {
    encode_peer_message(&PeerMessage::GetSharedFilesInFolder(
        SharedFilesInFolderRequestPayload {
            directory: directory.to_owned(),
        },
    ))
}

pub fn parse_transfer_request(payload: &[u8]) -> Result<TransferRequestPayload> {
    match decode_peer_message(CODE_PM_TRANSFER_REQUEST, payload)? {
        PeerMessage::TransferRequest(msg) => Ok(msg),
        _ => bail!("decoded peer message is not transfer request"),
    }
}

pub fn parse_transfer_response(payload: &[u8]) -> Result<TransferResponsePayload> {
    match decode_peer_message(CODE_PM_TRANSFER_RESPONSE, payload)? {
        PeerMessage::TransferResponse(msg) => Ok(msg),
        _ => bail!("decoded peer message is not transfer response"),
    }
}

pub fn parse_shared_files_in_folder_payload(payload: &[u8]) -> Result<SharedFilesInFolderPayload> {
    match decode_peer_message(CODE_PM_SHARED_FILES_IN_FOLDER, payload)? {
        PeerMessage::SharedFilesInFolder(msg) => Ok(msg),
        _ => bail!("decoded peer message is not shared files in folder response"),
    }
}

pub fn split_first_frame(buffer: &[u8]) -> Result<Option<(Frame, usize)>> {
    if buffer.len() < 4 {
        return Ok(None);
    }

    let declared = u32::from_le_bytes(buffer[0..4].try_into().context("frame length")?) as usize;
    let total = declared + 4;
    if buffer.len() < total {
        return Ok(None);
    }

    let frame = Frame::decode(&buffer[0..total])?;
    Ok(Some((frame, total)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode_hex(input: &str) -> Vec<u8> {
        let clean = input.trim();
        let mut out = Vec::with_capacity(clean.len() / 2);
        for pair in clean.as_bytes().chunks(2) {
            let hi = (pair[0] as char).to_digit(16).expect("hex") as u8;
            let lo = (pair[1] as char).to_digit(16).expect("hex") as u8;
            out.push((hi << 4) | lo);
        }
        out
    }

    fn sample_messages() -> Vec<ProtocolMessage> {
        vec![
            ProtocolMessage::Server(ServerMessage::Login(LoginPayload {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 157,
                md5hash: compute_login_md5hash("alice", "secret-pass"),
                minor_version: 19,
            })),
            ProtocolMessage::Server(ServerMessage::LoginResponse(LoginResponsePayload::Success(
                LoginResponseSuccessPayload {
                    greeting: "welcome".into(),
                    ip_address: "127.0.0.1".into(),
                    md5hash: "0123456789abcdef0123456789abcdef".into(),
                    is_supporter: true,
                },
            ))),
            ProtocolMessage::Server(ServerMessage::SetWaitPort(SetWaitPortPayload {
                listen_port: 2234,
            })),
            ProtocolMessage::Server(ServerMessage::GetPeerAddress(UserLookupPayload {
                username: "bob".into(),
            })),
            ProtocolMessage::Server(ServerMessage::GetPeerAddressResponse(
                PeerAddressResponsePayload {
                    username: "bob".into(),
                    ip_address: "203.0.113.5".into(),
                    port: 2234,
                    obfuscation_type: 1,
                    obfuscated_port: 40123,
                },
            )),
            ProtocolMessage::Server(ServerMessage::IgnoreUser(UserLookupPayload {
                username: "eve".into(),
            })),
            ProtocolMessage::Server(ServerMessage::UnignoreUser(UserLookupPayload {
                username: "eve".into(),
            })),
            ProtocolMessage::Server(ServerMessage::SayChatRoom(SayChatRoomPayload {
                room: "nicotine".into(),
                username: Some("alice".into()),
                message: "hello room".into(),
            })),
            ProtocolMessage::Server(ServerMessage::JoinRoom(JoinRoomPayload {
                room: "nicotine".into(),
                users: vec!["alice".into(), "bob".into()],
            })),
            ProtocolMessage::Server(ServerMessage::LeaveRoom(LeaveRoomPayload {
                room: "nicotine".into(),
            })),
            ProtocolMessage::Server(ServerMessage::UserJoinedRoom(RoomPresenceEventPayload {
                room: "nicotine".into(),
                username: "carol".into(),
            })),
            ProtocolMessage::Server(ServerMessage::UserLeftRoom(RoomPresenceEventPayload {
                room: "nicotine".into(),
                username: "dave".into(),
            })),
            ProtocolMessage::Server(ServerMessage::ConnectToPeerRequest(
                ConnectToPeerRequestPayload {
                    token: 77,
                    username: "bob".into(),
                    connection_type: "P".into(),
                },
            )),
            ProtocolMessage::Server(ServerMessage::ConnectToPeerResponse(
                ConnectToPeerResponsePayload {
                    username: "bob".into(),
                    connection_type: "P".into(),
                    ip_address: "198.51.100.24".into(),
                    port: 5566,
                    token: 77,
                    privileged: false,
                    obfuscation_type: 0,
                    obfuscated_port: 0,
                },
            )),
            ProtocolMessage::Server(ServerMessage::ConnectToPeer(ConnectToPeerPayload {
                username: "bob".into(),
                token: 77,
            })),
            ProtocolMessage::Server(ServerMessage::RoomList(RoomListPayload {
                room_count: 2,
                rooms: vec!["nicotine".into(), "electronic".into()],
            })),
            ProtocolMessage::Server(ServerMessage::FileSearch(FileSearchPayload {
                search_token: 12345,
                search_text: "aphex twin".into(),
            })),
            ProtocolMessage::Server(ServerMessage::FileSearchResponseSummary(
                SearchResponseSummary {
                    username: "peer_user".into(),
                    token: 12345,
                    files_count: 1,
                    slots_free: 2,
                    speed: 4096,
                    in_queue: false,
                    files: vec![SearchFileSummary {
                        file_path: "Music\\Aphex Twin\\Track.flac".into(),
                        file_size: 123_456,
                        extension: "flac".into(),
                        attr_count: 3,
                    }],
                },
            )),
            ProtocolMessage::Server(ServerMessage::SearchRoom(SearchRoomPayload {
                room: "electronic".into(),
                search_text: "selected ambient works".into(),
            })),
            ProtocolMessage::Server(ServerMessage::ExactFileSearch(ExactFileSearchPayload {
                virtual_path: "Music\\Aphex Twin\\Track.flac".into(),
            })),
            ProtocolMessage::Server(ServerMessage::SearchUserFiles(SearchUserFilesPayload {
                username: "bob".into(),
                search_text: "flac".into(),
            })),
            ProtocolMessage::Server(ServerMessage::GetSimilarTerms(SimilarTermsRequestPayload {
                term: "electronic".into(),
            })),
            ProtocolMessage::Server(ServerMessage::GetSimilarTermsResponse(
                SimilarTermsPayload {
                    term: "electronic".into(),
                    entries: vec![RecommendationEntry {
                        term: "idm".into(),
                        score: 7,
                    }],
                },
            )),
            ProtocolMessage::Server(ServerMessage::GetRecommendations(EmptyPayload)),
            ProtocolMessage::Server(ServerMessage::GetRecommendationsResponse(
                RecommendationsPayload {
                    recommendations: vec![RecommendationEntry {
                        term: "flac".into(),
                        score: 3,
                    }],
                    unrecommendations: vec![RecommendationEntry {
                        term: "low-bitrate".into(),
                        score: -2,
                    }],
                },
            )),
            ProtocolMessage::Server(ServerMessage::GetMyRecommendations(EmptyPayload)),
            ProtocolMessage::Server(ServerMessage::GetMyRecommendationsResponse(
                RecommendationsPayload {
                    recommendations: vec![RecommendationEntry {
                        term: "ambient".into(),
                        score: 4,
                    }],
                    unrecommendations: vec![],
                },
            )),
            ProtocolMessage::Server(ServerMessage::GetGlobalRecommendations(EmptyPayload)),
            ProtocolMessage::Server(ServerMessage::GetGlobalRecommendationsResponse(
                RecommendationsPayload {
                    recommendations: vec![RecommendationEntry {
                        term: "lossless".into(),
                        score: 8,
                    }],
                    unrecommendations: vec![RecommendationEntry {
                        term: "ads".into(),
                        score: -4,
                    }],
                },
            )),
            ProtocolMessage::Server(ServerMessage::GetOwnPrivilegesStatus(EmptyPayload)),
            ProtocolMessage::Server(ServerMessage::OwnPrivilegesStatus(
                OwnPrivilegesStatusPayload {
                    time_left_seconds: 86_400,
                },
            )),
            ProtocolMessage::Server(ServerMessage::GetUserPrivilegesStatus(UserLookupPayload {
                username: "bob".into(),
            })),
            ProtocolMessage::Server(ServerMessage::UserPrivilegesStatus(
                UserPrivilegesStatusPayload {
                    username: "bob".into(),
                    privileged: true,
                },
            )),
            ProtocolMessage::Server(ServerMessage::GivePrivilege(GivePrivilegePayload {
                username: "bob".into(),
                days: 7,
            })),
            ProtocolMessage::Server(ServerMessage::InformUserOfPrivileges(
                InformUserOfPrivilegesPayload {
                    token: 1234,
                    username: "bob".into(),
                },
            )),
            ProtocolMessage::Server(ServerMessage::InformUserOfPrivilegesAck(
                InformUserOfPrivilegesAckPayload { token: 1234 },
            )),
            ProtocolMessage::Server(ServerMessage::GetUserRecommendations(UserLookupPayload {
                username: "bob".into(),
            })),
            ProtocolMessage::Server(ServerMessage::GetUserRecommendationsResponse(
                UserRecommendationsPayload {
                    username: "bob".into(),
                    recommendations: RecommendationsPayload {
                        recommendations: vec![RecommendationEntry {
                            term: "aphex".into(),
                            score: 9,
                        }],
                        unrecommendations: vec![],
                    },
                },
            )),
            ProtocolMessage::Server(ServerMessage::PrivilegedList(PrivilegedListPayload {
                users: vec!["alice".into(), "bob".into()],
            })),
            ProtocolMessage::Server(ServerMessage::GetRecommendedUsers(EmptyPayload)),
            ProtocolMessage::Server(ServerMessage::GetRecommendedUsersResponse(
                RecommendedUsersPayload {
                    users: vec![ScoredUserEntry {
                        username: "alice".into(),
                        score: 12,
                    }],
                },
            )),
            ProtocolMessage::Server(ServerMessage::GetTermRecommendations(
                SimilarTermsRequestPayload { term: "idm".into() },
            )),
            ProtocolMessage::Server(ServerMessage::GetTermRecommendationsResponse(
                TermRecommendationsPayload {
                    term: "idm".into(),
                    recommendations: vec![RecommendationEntry {
                        term: "ambient".into(),
                        score: 5,
                    }],
                },
            )),
            ProtocolMessage::Server(ServerMessage::GetRecommendationUsers(
                SimilarTermsRequestPayload { term: "idm".into() },
            )),
            ProtocolMessage::Server(ServerMessage::GetRecommendationUsersResponse(
                RecommendationUsersPayload {
                    term: "idm".into(),
                    users: vec![ScoredUserEntry {
                        username: "charlie".into(),
                        score: 3,
                    }],
                },
            )),
            ProtocolMessage::Server(ServerMessage::BanUser(UserLookupPayload {
                username: "mallory".into(),
            })),
            ProtocolMessage::Server(ServerMessage::RoomMembers(RoomMembersPayload {
                room: "nicotine".into(),
                users: vec!["alice".into(), "bob".into(), "carol".into()],
            })),
            ProtocolMessage::Server(ServerMessage::RoomOperators(RoomOperatorsPayload {
                room: "nicotine".into(),
                operators: vec!["alice".into()],
            })),
            ProtocolMessage::Server(ServerMessage::AddRoomMember(RoomModerationPayload {
                room: "private-room".into(),
                username: "bob".into(),
            })),
            ProtocolMessage::Server(ServerMessage::RemoveRoomMember(RoomModerationPayload {
                room: "private-room".into(),
                username: "bob".into(),
            })),
            ProtocolMessage::Server(ServerMessage::AddRoomOperator(RoomModerationPayload {
                room: "private-room".into(),
                username: "alice".into(),
            })),
            ProtocolMessage::Server(ServerMessage::RemoveRoomOperator(RoomModerationPayload {
                room: "private-room".into(),
                username: "alice".into(),
            })),
            ProtocolMessage::Server(ServerMessage::MessageUserIncoming(
                MessageUserIncomingPayload {
                    message_id: 91,
                    timestamp: 1_705_000_000,
                    username: "bob".into(),
                    message: "hello inbound".into(),
                    is_new: true,
                },
            )),
            ProtocolMessage::Server(ServerMessage::MessageUser(MessageUserPayload {
                username: "bob".into(),
                message: "hello".into(),
            })),
            ProtocolMessage::Server(ServerMessage::MessageAcked(MessageAckedPayload {
                message_id: 55,
            })),
            ProtocolMessage::Server(ServerMessage::MessageUsers(MessageUsersPayload {
                usernames: vec!["alice".into(), "bob".into()],
                message: "broadcast test".into(),
            })),
            ProtocolMessage::Server(ServerMessage::PeerMessage(PeerMessagePayload {
                username: "bob".into(),
                message: "legacy request".into(),
                token: Some(17),
                code: Some(40),
                ip_address: None,
                port: None,
            })),
            ProtocolMessage::Server(ServerMessage::PeerMessage(PeerMessagePayload {
                username: "bob".into(),
                message: "legacy response".into(),
                token: Some(17),
                code: Some(40),
                ip_address: Some("198.51.100.15".into()),
                port: Some(2242),
            })),
            ProtocolMessage::Server(ServerMessage::GetUserStats(UserLookupPayload {
                username: "bob".into(),
            })),
            ProtocolMessage::Server(ServerMessage::GetUserStatsResponse(
                UserStatsResponsePayload {
                    username: "bob".into(),
                    avg_speed: 2048,
                    download_num: 12,
                    files: 500,
                    dirs: 42,
                },
            )),
            ProtocolMessage::Server(ServerMessage::GetUserStatus(UserLookupPayload {
                username: "bob".into(),
            })),
            ProtocolMessage::Server(ServerMessage::GetUserStatusResponse(
                UserStatusResponsePayload {
                    username: "bob".into(),
                    status: 2,
                    privileged: true,
                },
            )),
            ProtocolMessage::Server(ServerMessage::SharedFoldersFiles(
                SharedFoldersFilesPayload {
                    folder_count: 12,
                    file_count: 200,
                },
            )),
            ProtocolMessage::Server(ServerMessage::DownloadSpeed(SpeedPayload {
                bytes_per_sec: 2048,
            })),
            ProtocolMessage::Server(ServerMessage::UploadSpeed(SpeedPayload {
                bytes_per_sec: 1024,
            })),
            ProtocolMessage::Server(ServerMessage::AddUser(UserLookupPayload {
                username: "new-user".into(),
            })),
            ProtocolMessage::Server(ServerMessage::RemoveUser(UserLookupPayload {
                username: "old-user".into(),
            })),
            ProtocolMessage::Server(ServerMessage::LowPriorityFileSearch(FileSearchPayload {
                search_token: 99,
                search_text: "wishlist ambient".into(),
            })),
            ProtocolMessage::Server(ServerMessage::SendConnectToken(OpaquePayload {
                bytes: vec![0x10, 0x20, 0x30, 0x40],
            })),
            ProtocolMessage::Server(ServerMessage::PlaceInLine(OpaquePayload {
                bytes: vec![0xaa, 0xbb, 0xcc],
            })),
            ProtocolMessage::Server(ServerMessage::PlaceInLineResponse(OpaquePayload {
                bytes: vec![0xde, 0xad, 0xbe, 0xef],
            })),
            ProtocolMessage::Server(ServerMessage::AddPrivilegedUser(UserLookupPayload {
                username: "vip-user".into(),
            })),
            ProtocolMessage::Server(ServerMessage::WishlistWait(OpaquePayload {
                bytes: vec![0x3c, 0x00, 0x00, 0x00],
            })),
            ProtocolMessage::Server(ServerMessage::DnetLevel(OpaquePayload {
                bytes: vec![0x01, 0x00, 0x00, 0x00],
            })),
            ProtocolMessage::Server(ServerMessage::DnetGroupLeader(OpaquePayload {
                bytes: vec![0x02, 0x00, 0x00, 0x00],
            })),
            ProtocolMessage::Server(ServerMessage::DnetDeliveryReport(OpaquePayload {
                bytes: vec![0x03, 0x00, 0x00, 0x00],
            })),
            ProtocolMessage::Server(ServerMessage::DnetChildDepth(OpaquePayload {
                bytes: vec![0x04, 0x00, 0x00, 0x00],
            })),
            ProtocolMessage::Server(ServerMessage::Flood(OpaquePayload {
                bytes: vec![0x05, 0x00, 0x00, 0x00],
            })),
            ProtocolMessage::Server(ServerMessage::RemoveRoomOperatorship(OpaquePayload {
                bytes: vec![0x06, 0x00, 0x00, 0x00],
            })),
            ProtocolMessage::Server(ServerMessage::RemoveOwnRoomOperatorship(OpaquePayload {
                bytes: vec![0x07, 0x00, 0x00, 0x00],
            })),
            ProtocolMessage::Server(ServerMessage::JoinGlobalRoom(OpaquePayload {
                bytes: vec![0x08, 0x00, 0x00, 0x00],
            })),
            ProtocolMessage::Server(ServerMessage::LeaveGlobalRoom(OpaquePayload {
                bytes: vec![0x09, 0x00, 0x00, 0x00],
            })),
            ProtocolMessage::Server(ServerMessage::SayGlobalRoom(OpaquePayload {
                bytes: b"global hello".to_vec(),
            })),
            ProtocolMessage::Server(ServerMessage::SearchCorrelations(OpaquePayload {
                bytes: b"ambient".to_vec(),
            })),
            ProtocolMessage::Peer(PeerMessage::Say(OpaquePayload {
                bytes: b"peer say".to_vec(),
            })),
            ProtocolMessage::Peer(PeerMessage::GetSharedFileList(UserLookupPayload {
                username: "alice".into(),
            })),
            ProtocolMessage::Peer(PeerMessage::SharedFileList(SharedFileListPayload {
                entries: vec![
                    SharedFileEntry {
                        virtual_path: "Music\\A.flac".into(),
                        size: 100,
                    },
                    SharedFileEntry {
                        virtual_path: "Music\\B.flac".into(),
                        size: 200,
                    },
                ],
            })),
            ProtocolMessage::Peer(PeerMessage::GetSharedFilesInFolder(
                SharedFilesInFolderRequestPayload {
                    directory: "Music\\Albums".into(),
                },
            )),
            ProtocolMessage::Peer(PeerMessage::SharedFilesInFolder(
                SharedFilesInFolderPayload {
                    directory: "Music\\Albums".into(),
                    compressed_listing: vec![0x78, 0x9c, 0x03, 0x00],
                },
            )),
            ProtocolMessage::Peer(PeerMessage::FileSearchRequest(FileSearchRequestPayload {
                token: 9,
                query: "ambient".into(),
            })),
            ProtocolMessage::Peer(PeerMessage::FileSearchResult(FileSearchResultPayload {
                token: 9,
                username: "bob".into(),
                result_count: 2,
            })),
            ProtocolMessage::Peer(PeerMessage::InviteUserToRoom(PeerRoomInvitePayload {
                room: "nicotine".into(),
            })),
            ProtocolMessage::Peer(PeerMessage::CancelledQueuedTransfer(
                PeerVirtualPathPayload {
                    virtual_path: "Music\\A.flac".into(),
                },
            )),
            ProtocolMessage::Peer(PeerMessage::UserInfoRequest(UserInfoRequestPayload)),
            ProtocolMessage::Peer(PeerMessage::UserInfoReply(UserInfoReplyPayload {
                description: "hello".into(),
                has_picture: false,
                picture: Vec::new(),
                total_uploads: 12,
                queue_size: 2,
                slots_free: true,
                upload_permissions: Some(1),
            })),
            ProtocolMessage::Peer(PeerMessage::SendConnectToken(OpaquePayload {
                bytes: vec![0x33, 0x44, 0x55],
            })),
            ProtocolMessage::Peer(PeerMessage::TransferRequest(TransferRequestPayload {
                direction: TransferDirection::Download,
                token: 555,
                virtual_path: "Music\\Aphex Twin\\Track.flac".into(),
                file_size: 123_456_789,
            })),
            ProtocolMessage::Peer(PeerMessage::TransferResponse(TransferResponsePayload {
                token: 555,
                allowed: true,
                queue_or_reason: String::new(),
            })),
            ProtocolMessage::Peer(PeerMessage::PlaceholderUpload(OpaquePayload {
                bytes: vec![0x42, 0x42, 0x42],
            })),
            ProtocolMessage::Peer(PeerMessage::QueueUpload(QueueUploadPayload {
                username: "alice".into(),
                virtual_path: "Music\\queued.flac".into(),
            })),
            ProtocolMessage::Peer(PeerMessage::UploadPlaceInLine(UploadPlaceInLinePayload {
                username: "alice".into(),
                virtual_path: "Music\\queued.flac".into(),
                place: 3,
            })),
            ProtocolMessage::Peer(PeerMessage::MoveDownloadToTop(PeerVirtualPathPayload {
                virtual_path: "Music\\queued.flac".into(),
            })),
            ProtocolMessage::Peer(PeerMessage::ExactFileSearchRequest(
                PeerSearchQueryPayload {
                    token: Some(123),
                    query: "Music\\A.flac".into(),
                },
            )),
            ProtocolMessage::Peer(PeerMessage::QueuedDownloads(PeerQueuedDownloadsPayload {
                virtual_paths: vec!["Music\\A.flac".into(), "Music\\B.flac".into()],
            })),
            ProtocolMessage::Peer(PeerMessage::IndirectFileSearchRequest(
                PeerSearchQueryPayload {
                    token: None,
                    query: "A.flac".into(),
                },
            )),
            ProtocolMessage::Peer(PeerMessage::UploadFailed(UploadStatusPayload {
                username: "alice".into(),
                virtual_path: "Music\\queued.flac".into(),
                reason: "network".into(),
            })),
            ProtocolMessage::Peer(PeerMessage::UploadDenied(UploadStatusPayload {
                username: "alice".into(),
                virtual_path: "Music\\queued.flac".into(),
                reason: "blocked".into(),
            })),
            ProtocolMessage::Peer(PeerMessage::UploadPlaceInLineRequest(
                UploadPlaceInLineRequestPayload {
                    virtual_path: "Music\\queued.flac".into(),
                },
            )),
            ProtocolMessage::Peer(PeerMessage::Nothing(OpaquePayload {
                bytes: vec![0x00, 0x01],
            })),
        ]
    }

    #[test]
    fn roundtrip_all_core_messages() {
        for message in sample_messages() {
            let frame = encode_message(&message);
            match &message {
                ProtocolMessage::Server(_) => {
                    let decoded = decode_server_message(frame.code, &frame.payload)
                        .expect("decode server message");
                    assert_eq!(ProtocolMessage::Server(decoded), message);
                }
                ProtocolMessage::Peer(_) => {
                    let decoded = decode_peer_message(frame.code, &frame.payload)
                        .expect("decode peer message");
                    assert_eq!(ProtocolMessage::Peer(decoded), message);
                }
            }
        }
    }

    #[test]
    fn room_request_builders_emit_expected_codes() {
        assert_eq!(build_room_list_request().code, CODE_SM_ROOM_LIST);
        assert_eq!(build_join_room_request("nicotine").code, CODE_SM_JOIN_ROOM);
        assert_eq!(
            build_leave_room_request("nicotine").code,
            CODE_SM_LEAVE_ROOM
        );
        assert_eq!(
            build_room_members_request("nicotine").code,
            CODE_SM_ROOM_MEMBERS
        );
        assert_eq!(
            build_room_operators_request("nicotine").code,
            CODE_SM_ROOM_OPERATORS
        );
        assert_eq!(
            build_say_chatroom("nicotine", "hello").code,
            CODE_SM_SAY_CHATROOM
        );
        assert_eq!(
            build_add_room_member_request("private", "bob").code,
            CODE_SM_ADD_ROOM_MEMBER
        );
        assert_eq!(
            build_remove_room_member_request("private", "bob").code,
            CODE_SM_REMOVE_ROOM_MEMBER
        );
        assert_eq!(
            build_add_room_operator_request("private", "bob").code,
            CODE_SM_ADD_ROOM_OPERATOR
        );
        assert_eq!(
            build_remove_room_operator_request("private", "bob").code,
            CODE_SM_REMOVE_ROOM_OPERATOR
        );
    }

    #[test]
    fn discovery_request_builders_emit_expected_codes() {
        assert_eq!(
            build_get_recommendations_request().code,
            CODE_SM_GET_RECOMMENDATIONS
        );
        assert_eq!(
            build_get_my_recommendations_request().code,
            CODE_SM_GET_MY_RECOMMENDATIONS
        );
        assert_eq!(
            build_get_global_recommendations_request().code,
            CODE_SM_GET_GLOBAL_RECOMMENDATIONS
        );
        assert_eq!(
            build_get_user_recommendations_request("alice").code,
            CODE_SM_GET_USER_RECOMMENDATIONS
        );
        assert_eq!(
            build_get_similar_terms_request("electronic").code,
            CODE_SM_GET_SIMILAR_TERMS
        );
    }

    #[test]
    fn privileges_request_builders_emit_expected_codes() {
        assert_eq!(build_ignore_user_request("alice").code, CODE_SM_IGNORE_USER);
        assert_eq!(
            build_unignore_user_request("alice").code,
            CODE_SM_UNIGNORE_USER
        );
        assert_eq!(
            build_get_own_privileges_status_request().code,
            CODE_SM_GET_OWN_PRIVILEGES_STATUS
        );
        assert_eq!(
            build_get_user_privileges_status_request("alice").code,
            CODE_SM_GET_USER_PRIVILEGES_STATUS
        );
        assert_eq!(
            build_give_privilege_request("alice", 7).code,
            CODE_SM_GIVE_PRIVILEGE
        );
        assert_eq!(
            build_inform_user_of_privileges_request(55, "alice").code,
            CODE_SM_INFORM_USER_OF_PRIVILEGES
        );
        assert_eq!(
            build_inform_user_of_privileges_ack_request(55).code,
            CODE_SM_INFORM_USER_OF_PRIVILEGES_ACK
        );
    }

    #[test]
    fn stage4d_server_request_builders_emit_expected_codes() {
        assert_eq!(build_ban_user_request("alice").code, CODE_SM_BAN_USER);
        assert_eq!(
            build_privileged_list_request().code,
            CODE_SM_PRIVILEGED_LIST
        );
        assert_eq!(
            build_get_recommended_users_request().code,
            CODE_SM_GET_RECOMMENDED_USERS
        );
        assert_eq!(
            build_get_term_recommendations_request("idm").code,
            CODE_SM_GET_TERM_RECOMMENDATIONS
        );
        assert_eq!(
            build_get_recommendation_users_request("idm").code,
            CODE_SM_GET_RECOMMENDATION_USERS
        );
    }

    #[test]
    fn stage4e_server_request_builders_emit_expected_codes() {
        assert_eq!(
            build_get_peer_address_request("alice").code,
            CODE_SM_GET_PEER_ADDRESS
        );
        assert_eq!(
            build_get_user_status_request("alice").code,
            CODE_SM_GET_USER_STATUS
        );
        assert_eq!(
            build_get_user_stats_request("alice").code,
            CODE_SM_GET_USER_STATS
        );
        assert_eq!(
            build_connect_to_peer_request(77, "alice", "P").code,
            CODE_SM_CONNECT_TO_PEER
        );
        assert_eq!(
            build_message_user_request("alice", "hello").code,
            CODE_SM_MESSAGE_USER
        );
        assert_eq!(
            build_message_users_request(&["alice".to_string(), "bob".to_string()], "hello").code,
            CODE_SM_MESSAGE_USERS
        );
    }

    #[test]
    fn s4l_opaque_server_control_codes_roundtrip() {
        for code in OPAQUE_SERVER_CONTROL_CODES {
            let payload = vec![code as u8, 0x55, 0xaa];
            let decoded = decode_server_message(code, &payload).expect("decode opaque control");
            let ServerMessage::OpaqueControl(control) = decoded else {
                panic!("expected opaque control variant for code {code}");
            };
            assert_eq!(control.code, code);
            assert_eq!(control.bytes, payload);

            let rebuilt = encode_server_message(&ServerMessage::OpaqueControl(control.clone()));
            assert_eq!(rebuilt.code, code);
            assert_eq!(rebuilt.payload, payload);

            let built = build_opaque_server_control_request(code, &payload)
                .expect("builder should accept closure code");
            assert_eq!(built.code, code);
            assert_eq!(built.payload, payload);
        }
    }

    #[test]
    fn opaque_server_control_builder_rejects_unknown_code() {
        let err = build_opaque_server_control_request(4242, &[1, 2, 3]).expect_err("must fail");
        assert!(
            err.to_string()
                .contains("unsupported opaque server control code")
        );
    }

    #[test]
    fn peer_folder_request_builder_emits_expected_code() {
        assert_eq!(
            build_get_shared_files_in_folder_request("Music").code,
            CODE_PM_GET_SHARED_FILES_IN_FOLDER
        );
    }

    #[test]
    fn frame_rejects_truncated_payload() {
        let bad = decode_hex("04000000010000");
        let err = Frame::decode(&bad).expect_err("must fail");
        assert!(
            err.to_string().contains("frame too short") || err.to_string().contains("mismatch")
        );
    }

    #[test]
    fn decode_rejects_unknown_code() {
        let frame = Frame::new(9999, vec![0, 1, 2]);
        let err = decode_message(&frame).expect_err("unknown code must fail");
        assert!(err.to_string().contains("unsupported message code"));
    }

    #[test]
    fn decode_rejects_ambiguous_code_without_scope() {
        let mut payload = PayloadWriter::new();
        payload.write_string("nicotine");
        let frame = Frame::new(CODE_SM_JOIN_ROOM, payload.into_inner());
        let err = decode_message(&frame).expect_err("ambiguous decode must fail");
        assert!(err.to_string().contains("ambiguous message code"));
    }

    #[test]
    fn login_request_includes_md5hash() {
        let frame = build_login_request("alice", "secret-pass", 157, 19);
        assert_eq!(frame.code, CODE_SM_LOGIN);

        let decoded =
            decode_server_message(frame.code, &frame.payload).expect("decode login payload");
        let ServerMessage::Login(payload) = decoded else {
            panic!("expected login payload");
        };

        assert_eq!(payload.username, "alice");
        assert_eq!(payload.password, "secret-pass");
        assert_eq!(payload.client_version, 157);
        assert_eq!(payload.minor_version, 19);
        assert_eq!(payload.md5hash, "a709495ec9adc487831c96a72a7728cf");
    }

    #[test]
    fn parse_login_response_success_fixture() {
        let payload = decode_hex(
            "0100000000922d5e0320000000376261383835313432656633366135353765376531646430633030623263373600",
        );
        let parsed = parse_login_response(&payload).expect("parse login success");
        let LoginResponsePayload::Success(success) = parsed else {
            panic!("expected success payload");
        };
        assert_eq!(success.greeting, "");
        assert_eq!(success.ip_address, "146.45.94.3");
        assert_eq!(success.md5hash, "7ba885142ef36a557e7e1dd0c00b2c76");
        assert!(!success.is_supporter);
    }

    #[test]
    fn parse_login_response_failure_fixture() {
        let payload = decode_hex("000e000000494e56414c494456455253494f4e");
        let parsed = parse_login_response(&payload).expect("parse login failure");
        let LoginResponsePayload::Failure(failure) = parsed else {
            panic!("expected failure payload");
        };
        assert_eq!(failure.reason, LoginFailureReason::InvalidVersion);
        assert_eq!(failure.detail, None);
    }

    #[test]
    fn parse_search_summary_roundtrip() {
        let original = SearchResponseSummary {
            username: "peer-user".into(),
            token: 321,
            files_count: 1,
            slots_free: 3,
            speed: 2048,
            in_queue: true,
            files: vec![SearchFileSummary {
                file_path: "Music\\Track.flac".into(),
                file_size: 9999,
                extension: "flac".into(),
                attr_count: 2,
            }],
        };
        let frame =
            encode_server_message(&ServerMessage::FileSearchResponseSummary(original.clone()));
        let parsed = parse_search_response_summary(&frame.payload).expect("parse summary");
        assert_eq!(parsed, original);
    }

    #[test]
    fn search_fixture_matches() {
        let frame = build_file_search_request(12345, "aphex twin");
        let expected = decode_hex("160000001a000000393000000a0000006170686578207477696e");
        assert_eq!(frame.encode(), expected);
    }

    #[test]
    fn transfer_response_parse_roundtrip() {
        let frame = build_transfer_response(555, true, "");
        let parsed = parse_transfer_response(&frame.payload).expect("parse response");
        assert_eq!(parsed.token, 555);
        assert!(parsed.allowed);
        assert_eq!(parsed.queue_or_reason, "");
    }

    #[test]
    fn peer_advanced_request_builders_emit_expected_codes() {
        assert_eq!(build_user_info_request().code, CODE_PM_USER_INFO_REQUEST);
        assert_eq!(
            build_exact_file_search_request("A.flac", Some(10)).code,
            CODE_PM_EXACT_FILE_SEARCH_REQUEST
        );
        assert_eq!(
            build_indirect_file_search_request("A.flac", None).code,
            CODE_PM_INDIRECT_FILE_SEARCH_REQUEST
        );
        assert_eq!(
            build_upload_place_in_line_request("Music\\A.flac").code,
            CODE_PM_UPLOAD_PLACE_IN_LINE_REQUEST
        );
    }

    #[test]
    fn stage4d_peer_legacy_builders_emit_expected_codes() {
        assert_eq!(
            build_peer_invite_user_to_room("nicotine").code,
            CODE_PM_INVITE_USER_TO_ROOM
        );
        assert_eq!(
            build_peer_cancelled_queued_transfer("Music\\A.flac").code,
            CODE_PM_CANCELLED_QUEUED_TRANSFER
        );
        assert_eq!(
            build_peer_move_download_to_top("Music\\A.flac").code,
            CODE_PM_MOVE_DOWNLOAD_TO_TOP
        );
        assert_eq!(
            build_peer_queued_downloads(&["Music\\A.flac".to_string()]).code,
            CODE_PM_QUEUED_DOWNLOADS
        );
    }

    #[test]
    fn decode_server_directional_responses_for_s4e_payloads() {
        let peer_addr_frame = encode_server_message(&ServerMessage::GetPeerAddressResponse(
            PeerAddressResponsePayload {
                username: "alice".into(),
                ip_address: "203.0.113.8".into(),
                port: 2242,
                obfuscation_type: 0,
                obfuscated_port: 0,
            },
        ));
        let peer_addr =
            decode_server_message(peer_addr_frame.code, &peer_addr_frame.payload).expect("decode");
        assert!(matches!(
            peer_addr,
            ServerMessage::GetPeerAddressResponse(_)
        ));

        let status_frame = encode_server_message(&ServerMessage::GetUserStatusResponse(
            UserStatusResponsePayload {
                username: "alice".into(),
                status: 2,
                privileged: true,
            },
        ));
        let status = decode_server_message(status_frame.code, &status_frame.payload)
            .expect("decode user status response");
        assert!(matches!(status, ServerMessage::GetUserStatusResponse(_)));

        let stats_frame = encode_server_message(&ServerMessage::GetUserStatsResponse(
            UserStatsResponsePayload {
                username: "alice".into(),
                avg_speed: 2048,
                download_num: 3,
                files: 120,
                dirs: 12,
            },
        ));
        let stats =
            decode_server_message(stats_frame.code, &stats_frame.payload).expect("decode stats");
        assert!(matches!(stats, ServerMessage::GetUserStatsResponse(_)));
    }

    #[test]
    fn decode_user_stats_response_accepts_extension_counter() {
        let mut writer = PayloadWriter::new();
        writer.write_string("alice");
        writer.write_u32(2048);
        writer.write_u32(3);
        writer.write_u32(120);
        writer.write_u32(12);
        writer.write_u32(777); // runtime extension counter

        let decoded =
            decode_server_message(CODE_SM_GET_USER_STATS, &writer.into_inner()).expect("decode");
        match decoded {
            ServerMessage::GetUserStatsResponse(payload) => {
                assert_eq!(payload.username, "alice");
                assert_eq!(payload.avg_speed, 2048);
                assert_eq!(payload.download_num, 3);
                assert_eq!(payload.files, 120);
                assert_eq!(payload.dirs, 12);
            }
            other => panic!("expected user stats response, got {other:?}"),
        }
    }

    #[test]
    fn decode_connect_to_peer_response_accepts_u8_privileged_variant() {
        let mut writer = PayloadWriter::new();
        writer.write_string("alice");
        writer.write_string("P");
        writer.write_u32(u32::from_le_bytes([203, 0, 113, 8]));
        writer.write_u32(2242);
        writer.write_u32(77);
        writer.write_u8(1); // privileged as bool_u8
        writer.write_u32(2);
        writer.write_u32(40123);

        let decoded =
            decode_server_message(CODE_SM_CONNECT_TO_PEER, &writer.into_inner()).expect("decode");
        match decoded {
            ServerMessage::ConnectToPeerResponse(payload) => {
                assert_eq!(payload.username, "alice");
                assert_eq!(payload.connection_type, "P");
                assert_eq!(payload.ip_address, "203.0.113.8");
                assert_eq!(payload.port, 2242);
                assert_eq!(payload.token, 77);
                assert!(payload.privileged);
                assert_eq!(payload.obfuscation_type, 2);
                assert_eq!(payload.obfuscated_port, 40123);
            }
            other => panic!("expected connect_to_peer response, got {other:?}"),
        }
    }

    #[test]
    fn decode_message_user_incoming_and_peer_message_alt() {
        let incoming = encode_server_message(&ServerMessage::MessageUserIncoming(
            MessageUserIncomingPayload {
                message_id: 11,
                timestamp: 1_705_000_000,
                username: "bob".into(),
                message: "hello".into(),
                is_new: true,
            },
        ));
        let decoded =
            decode_server_message(incoming.code, &incoming.payload).expect("decode incoming");
        assert!(matches!(decoded, ServerMessage::MessageUserIncoming(_)));

        let peer_message = encode_server_message(&ServerMessage::PeerMessage(PeerMessagePayload {
            username: "bob".into(),
            message: "legacy".into(),
            token: Some(7),
            code: Some(40),
            ip_address: Some("198.51.100.11".into()),
            port: Some(2242),
        }));
        let alt_frame = Frame::new(CODE_SM_PEER_MESSAGE_ALT, peer_message.payload);
        let alt_decoded =
            decode_server_message(alt_frame.code, &alt_frame.payload).expect("decode alt");
        assert!(matches!(alt_decoded, ServerMessage::PeerMessage(_)));
    }
}
