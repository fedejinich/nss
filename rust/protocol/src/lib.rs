use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use thiserror::Error;

pub const CODE_SM_LOGIN: u32 = 1;
pub const CODE_SM_SET_WAIT_PORT: u32 = 2;
pub const CODE_SM_GET_PEER_ADDRESS: u32 = 3;
pub const CODE_SM_GET_USER_STATUS: u32 = 7;
pub const CODE_SM_CONNECT_TO_PEER: u32 = 18;
pub const CODE_SM_MESSAGE_USER: u32 = 22;
pub const CODE_SM_MESSAGE_ACKED: u32 = 23;
pub const CODE_SM_FILE_SEARCH: u32 = 26;
pub const CODE_SM_FILE_SEARCH_RESPONSE: u32 = 64;
pub const CODE_SM_DOWNLOAD_SPEED: u32 = 34;
pub const CODE_SM_SHARED_FOLDERS_FILES: u32 = 35;
pub const CODE_SM_GET_USER_STATS: u32 = 36;
pub const CODE_SM_SEARCH_USER_FILES: u32 = 42;
pub const CODE_SM_EXACT_FILE_SEARCH: u32 = 65;
pub const CODE_SM_SEARCH_ROOM: u32 = 120;
pub const CODE_SM_UPLOAD_SPEED: u32 = 121;

pub const CODE_PM_GET_SHARED_FILE_LIST: u32 = 4;
pub const CODE_PM_SHARED_FILE_LIST: u32 = 5;
pub const CODE_PM_FILE_SEARCH_REQUEST: u32 = 8;
pub const CODE_PM_FILE_SEARCH_RESULT: u32 = 9;
pub const CODE_PM_TRANSFER_REQUEST: u32 = 40;
pub const CODE_PM_TRANSFER_RESPONSE: u32 = 41;
pub const CODE_PM_QUEUE_UPLOAD: u32 = 43;
pub const CODE_PM_UPLOAD_PLACE_IN_LINE: u32 = 44;
pub const CODE_PM_UPLOAD_FAILED: u32 = 46;
pub const CODE_PM_UPLOAD_DENIED: u32 = 50;

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

    pub fn read_string(&mut self) -> Result<String, DecoderError> {
        let len = self.read_u32()? as usize;
        let bytes = self.take(len)?;
        Ok(String::from_utf8_lossy(bytes).into_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmptyPayload;

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
pub struct ConnectToPeerPayload {
    pub username: String,
    pub token: u32,
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
pub struct MessageUserPayload {
    pub username: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageAckedPayload {
    pub message_id: u32,
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
pub enum ServerMessage {
    Login(LoginPayload),
    LoginResponse(LoginResponsePayload),
    SetWaitPort(SetWaitPortPayload),
    GetPeerAddress(UserLookupPayload),
    ConnectToPeer(ConnectToPeerPayload),
    FileSearch(FileSearchPayload),
    FileSearchResponseSummary(SearchResponseSummary),
    SearchRoom(SearchRoomPayload),
    ExactFileSearch(ExactFileSearchPayload),
    SearchUserFiles(SearchUserFilesPayload),
    MessageUser(MessageUserPayload),
    MessageAcked(MessageAckedPayload),
    GetUserStats(UserLookupPayload),
    GetUserStatus(UserLookupPayload),
    SharedFoldersFiles(SharedFoldersFilesPayload),
    DownloadSpeed(SpeedPayload),
    UploadSpeed(SpeedPayload),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeerMessage {
    GetSharedFileList(UserLookupPayload),
    SharedFileList(SharedFileListPayload),
    FileSearchRequest(FileSearchRequestPayload),
    FileSearchResult(FileSearchResultPayload),
    TransferRequest(TransferRequestPayload),
    TransferResponse(TransferResponsePayload),
    QueueUpload(QueueUploadPayload),
    UploadPlaceInLine(UploadPlaceInLinePayload),
    UploadFailed(UploadStatusPayload),
    UploadDenied(UploadStatusPayload),
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

pub fn encode_message(message: &ProtocolMessage) -> Frame {
    match message {
        ProtocolMessage::Server(server) => encode_server_message(server),
        ProtocolMessage::Peer(peer) => encode_peer_message(peer),
    }
}

pub fn decode_message(frame: &Frame) -> Result<ProtocolMessage> {
    if let Ok(server) = decode_server_message(frame.code, &frame.payload) {
        return Ok(ProtocolMessage::Server(server));
    }

    if let Ok(peer) = decode_peer_message(frame.code, &frame.payload) {
        return Ok(ProtocolMessage::Peer(peer));
    }

    bail!(
        "unsupported message code {} (payload_len={})",
        frame.code,
        frame.payload.len()
    )
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
        ServerMessage::MessageUser(payload) => {
            writer.write_string(&payload.username);
            writer.write_string(&payload.message);
            CODE_SM_MESSAGE_USER
        }
        ServerMessage::MessageAcked(payload) => {
            writer.write_u32(payload.message_id);
            CODE_SM_MESSAGE_ACKED
        }
        ServerMessage::GetUserStats(payload) => {
            writer.write_string(&payload.username);
            CODE_SM_GET_USER_STATS
        }
        ServerMessage::GetUserStatus(payload) => {
            writer.write_string(&payload.username);
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

    if code == CODE_SM_FILE_SEARCH_RESPONSE {
        return Ok(ServerMessage::FileSearchResponseSummary(
            parse_search_response_summary(payload)?,
        ));
    }

    let mut reader = PayloadReader::new(payload);

    let message = match code {
        CODE_SM_SET_WAIT_PORT => {
            let payload = SetWaitPortPayload {
                listen_port: reader.read_u32()?,
            };
            ServerMessage::SetWaitPort(payload)
        }
        CODE_SM_GET_PEER_ADDRESS => {
            let payload = UserLookupPayload {
                username: reader.read_string()?,
            };
            ServerMessage::GetPeerAddress(payload)
        }
        CODE_SM_CONNECT_TO_PEER => {
            let payload = ConnectToPeerPayload {
                username: reader.read_string()?,
                token: reader.read_u32()?,
            };
            ServerMessage::ConnectToPeer(payload)
        }
        CODE_SM_FILE_SEARCH => {
            let payload = FileSearchPayload {
                search_token: reader.read_u32()?,
                search_text: reader.read_string()?,
            };
            ServerMessage::FileSearch(payload)
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
        CODE_SM_MESSAGE_USER => {
            let payload = MessageUserPayload {
                username: reader.read_string()?,
                message: reader.read_string()?,
            };
            ServerMessage::MessageUser(payload)
        }
        CODE_SM_MESSAGE_ACKED => {
            let payload = MessageAckedPayload {
                message_id: reader.read_u32()?,
            };
            ServerMessage::MessageAcked(payload)
        }
        CODE_SM_GET_USER_STATS => {
            let payload = UserLookupPayload {
                username: reader.read_string()?,
            };
            ServerMessage::GetUserStats(payload)
        }
        CODE_SM_GET_USER_STATUS => {
            let payload = UserLookupPayload {
                username: reader.read_string()?,
            };
            ServerMessage::GetUserStatus(payload)
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
        other => bail!("unsupported server message code {other}"),
    };

    ensure_payload_consumed(&reader)?;
    Ok(message)
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

pub fn encode_peer_message(message: &PeerMessage) -> Frame {
    let mut writer = PayloadWriter::new();
    let code = match message {
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
    };

    Frame::new(code, writer.into_inner())
}

pub fn decode_peer_message(code: u32, payload: &[u8]) -> Result<PeerMessage> {
    let mut reader = PayloadReader::new(payload);

    let message = match code {
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
        other => bail!("unsupported peer message code {other}"),
    };

    ensure_payload_consumed(&reader)?;
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
            ProtocolMessage::Server(ServerMessage::ConnectToPeer(ConnectToPeerPayload {
                username: "bob".into(),
                token: 77,
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
            ProtocolMessage::Server(ServerMessage::MessageUser(MessageUserPayload {
                username: "bob".into(),
                message: "hola".into(),
            })),
            ProtocolMessage::Server(ServerMessage::MessageAcked(MessageAckedPayload {
                message_id: 55,
            })),
            ProtocolMessage::Server(ServerMessage::GetUserStats(UserLookupPayload {
                username: "bob".into(),
            })),
            ProtocolMessage::Server(ServerMessage::GetUserStatus(UserLookupPayload {
                username: "bob".into(),
            })),
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
            ProtocolMessage::Peer(PeerMessage::FileSearchRequest(FileSearchRequestPayload {
                token: 9,
                query: "ambient".into(),
            })),
            ProtocolMessage::Peer(PeerMessage::FileSearchResult(FileSearchResultPayload {
                token: 9,
                username: "bob".into(),
                result_count: 2,
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
            ProtocolMessage::Peer(PeerMessage::QueueUpload(QueueUploadPayload {
                username: "alice".into(),
                virtual_path: "Music\\queued.flac".into(),
            })),
            ProtocolMessage::Peer(PeerMessage::UploadPlaceInLine(UploadPlaceInLinePayload {
                username: "alice".into(),
                virtual_path: "Music\\queued.flac".into(),
                place: 3,
            })),
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
        ]
    }

    #[test]
    fn roundtrip_all_core_messages() {
        for message in sample_messages() {
            let frame = encode_message(&message);
            let decoded = decode_message(&frame).expect("decode message");
            assert_eq!(decoded, message);
        }
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
}
