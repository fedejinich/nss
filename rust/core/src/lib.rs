use anyhow::{Context, Result, anyhow, bail};
use protocol::{
    CODE_PM_FILE_SEARCH_RESULT, CODE_PM_QUEUE_UPLOAD, CODE_PM_TRANSFER_REQUEST,
    CODE_PM_UPLOAD_DENIED, CODE_PM_UPLOAD_FAILED, CODE_PM_UPLOAD_PLACE_IN_LINE,
    CODE_SM_GET_OWN_PRIVILEGES_STATUS, CODE_SM_GET_PEER_ADDRESS, CODE_SM_GET_RECOMMENDATION_USERS,
    CODE_SM_GET_RECOMMENDED_USERS, CODE_SM_GET_ROOM_TICKER, CODE_SM_GET_TERM_RECOMMENDATIONS,
    CODE_SM_GET_USER_PRIVILEGES_STATUS, CODE_SM_GET_USER_STATS, CODE_SM_GET_USER_STATUS,
    CODE_SM_LOGIN, CODE_SM_MESSAGE_ACKED, CODE_SM_PRIVILEGED_LIST, CODE_SM_ROOM_LIST,
    ConnectToPeerResponsePayload, FileSearchRequestPayload, Frame, LoginFailureReason,
    LoginResponsePayload, MessageAckedPayload, MessageUserIncomingPayload,
    OwnPrivilegesStatusPayload, PayloadReader, PayloadWriter, PeerAddressResponsePayload,
    PeerMessage, PrivilegedListPayload, ProtocolMessage, RecommendationUsersPayload,
    QueueUploadPayload, RecommendationsPayload, RecommendedUsersPayload, RoomListPayload, RoomMembersPayload,
    RoomOperatorsPayload, RoomTickerPayload, SearchResponseSummary, ServerMessage,
    SetWaitPortPayload, SimilarTermsPayload, TermRecommendationsPayload, TransferDirection,
    TransferRequestPayload, TransferResponsePayload, UserPrivilegesStatusPayload,
    UserRecommendationsPayload, UserStatsResponsePayload, UserStatusResponsePayload,
    build_add_chatroom_request, build_add_like_term_request, build_add_room_member_request,
    build_add_room_operator_request, build_ban_user_request, build_connect_to_peer_request,
    build_file_search_request, build_get_global_recommendations_request,
    build_get_my_recommendations_request, build_get_own_privileges_status_request,
    build_get_peer_address_request, build_get_recommendation_users_request,
    build_get_recommendations_request, build_get_recommended_users_request,
    build_get_room_ticker_request, build_get_similar_terms_request,
    build_get_term_recommendations_request, build_get_user_privileges_status_request,
    build_get_user_recommendations_request, build_get_user_stats_request,
    build_get_user_status_request, build_give_privilege_request, build_ignore_user_request,
    build_inform_user_of_privileges_ack_request, build_inform_user_of_privileges_request,
    build_join_room_request, build_leave_room_request, build_login_request,
    build_message_user_request, build_message_users_request, build_privileged_list_request,
    build_remove_like_term_request, build_remove_room_member_request,
    build_remove_room_operator_request, build_room_list_request, build_room_members_request,
    build_room_operators_request, build_say_chatroom, build_send_connect_token,
    build_transfer_request, build_unignore_user_request, build_upload_speed_request,
    decode_peer_message, decode_server_message, encode_peer_message, encode_server_message,
    split_first_frame,
};
use std::collections::HashSet;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub client_version: u32,
    pub minor_version: u32,
}

#[derive(Debug, Clone)]
pub struct DownloadPlan {
    pub peer_addr: String,
    pub token: u32,
    pub virtual_path: String,
    pub file_size: u64,
    pub output_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct DownloadResult {
    pub output_path: PathBuf,
    pub bytes_written: u64,
}

#[derive(Debug, Clone)]
pub struct SearchSelectDownloadRequest {
    pub search_token: u32,
    pub query: String,
    pub search_timeout: Duration,
    pub max_messages: usize,
    pub result_index: usize,
    pub file_index: usize,
    pub transfer_token: u32,
    pub output_path: PathBuf,
    pub peer_addr_override: Option<String>,
    pub peer_lookup_timeout: Duration,
    pub connection_type: String,
    pub wait_port: Option<u16>,
    pub skip_connect_probe: bool,
    pub search_mode: SearchMode,
    pub strict_track: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SearchSelectDownloadResult {
    pub selected_username: String,
    pub selected_virtual_path: String,
    pub selected_file_size: u64,
    pub peer_addr: String,
    pub transfer_token: u32,
    pub output_path: PathBuf,
    pub bytes_written: u64,
    pub search_source: SearchResultSource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchCandidate {
    pub username: String,
    pub file_path: String,
    pub file_size: u64,
    pub peer_addr: Option<String>,
    pub connect_token: Option<u32>,
    pub source: SearchResultSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchMode {
    Auto,
    Summary,
    Distributed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchResultSource {
    ServerSummary,
    DistributedPeer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Disconnected,
    Connected,
    LoggedIn,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoomEvent {
    UserJoined {
        room: String,
        username: String,
    },
    UserLeft {
        room: String,
        username: String,
    },
    RoomMessage {
        room: String,
        username: Option<String>,
        message: String,
    },
    MembersSnapshot(RoomMembersPayload),
    OperatorsSnapshot(RoomOperatorsPayload),
    TickerSnapshot(RoomTickerPayload),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrivateEvent {
    Message(MessageUserIncomingPayload),
    Ack(MessageAckedPayload),
}

#[derive(Debug)]
pub struct SessionClient {
    stream: Option<TcpStream>,
    state: SessionState,
    login_response_timeout: Duration,
    logged_username: Option<String>,
}

pub type SoulClient = SessionClient;

impl SessionClient {
    const DEFAULT_LOGIN_RESPONSE_TIMEOUT: Duration = Duration::from_secs(5);

    pub fn new_disconnected() -> Self {
        Self {
            stream: None,
            state: SessionState::Disconnected,
            login_response_timeout: Self::DEFAULT_LOGIN_RESPONSE_TIMEOUT,
            logged_username: None,
        }
    }

    pub async fn connect(server_addr: &str) -> Result<Self> {
        let stream = TcpStream::connect(server_addr)
            .await
            .with_context(|| format!("connect failed: {server_addr}"))?;

        Ok(Self {
            stream: Some(stream),
            state: SessionState::Connected,
            login_response_timeout: Self::DEFAULT_LOGIN_RESPONSE_TIMEOUT,
            logged_username: None,
        })
    }

    pub fn state(&self) -> SessionState {
        self.state
    }

    pub fn set_login_response_timeout(&mut self, timeout: Duration) {
        self.login_response_timeout = timeout;
    }

    pub async fn login(&mut self, credentials: &Credentials) -> std::result::Result<(), AuthError> {
        self.ensure_connected()
            .map_err(|err| AuthError::ProtocolDecode(err.to_string()))?;
        let frame = build_login_request(
            &credentials.username,
            &credentials.password,
            credentials.client_version,
            credentials.minor_version,
        );
        let stream = self.stream.as_mut().ok_or_else(|| {
            AuthError::ProtocolDecode("session stream is unavailable".to_string())
        })?;
        write_frame(stream, &frame)
            .await
            .map_err(|err| AuthError::ProtocolDecode(format!("write login frame: {err}")))?;

        let response_frame = tokio::time::timeout(self.login_response_timeout, read_frame(stream))
            .await
            .map_err(|_| AuthError::Timeout)?
            .map_err(|err| {
                if is_connection_eof(&err) {
                    AuthError::ServerClosedBeforeLoginResponse
                } else {
                    AuthError::ProtocolDecode(format!("read login response: {err}"))
                }
            })?;

        if response_frame.code != CODE_SM_LOGIN {
            return Err(AuthError::ProtocolDecode(format!(
                "expected login response code {} got {}",
                CODE_SM_LOGIN, response_frame.code
            )));
        }

        let payload = match decode_server_message(response_frame.code, &response_frame.payload)
            .map_err(|err| AuthError::ProtocolDecode(format!("decode login response: {err}")))?
        {
            ServerMessage::LoginResponse(payload) => payload,
            other => {
                return Err(AuthError::ProtocolDecode(format!(
                    "expected LoginResponse payload, got {other:?}"
                )));
            }
        };

        match payload {
            LoginResponsePayload::Success(_) => {
                self.state = SessionState::LoggedIn;
                self.logged_username = Some(credentials.username.clone());
                Ok(())
            }
            LoginResponsePayload::Failure(failure) => {
                self.logged_username = None;
                Err(AuthError::from_failure_reason(
                    failure.reason,
                    failure.detail,
                ))
            }
        }
    }

    pub async fn search(&mut self, token: u32, search_text: &str) -> Result<()> {
        self.ensure_logged_in()?;
        let frame = build_file_search_request(token, search_text);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn list_rooms(&mut self, timeout: Duration) -> Result<RoomListPayload> {
        self.ensure_logged_in()?;
        let frame = build_room_list_request();
        write_frame(self.stream_mut()?, &frame).await?;

        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            let remaining = deadline.saturating_duration_since(Instant::now());
            match tokio::time::timeout(remaining, self.read_next_frame()).await {
                Ok(Ok(response)) => {
                    if response.code != CODE_SM_ROOM_LIST {
                        continue;
                    }
                    if let Ok(ServerMessage::RoomList(payload)) =
                        decode_server_message(response.code, &response.payload)
                    {
                        return Ok(payload);
                    }
                }
                Ok(Err(err)) => {
                    if is_connection_eof(&err) {
                        break;
                    }
                    return Err(err);
                }
                Err(_) => break,
            }
        }

        bail!("timed out waiting for room list")
    }

    pub async fn join_room(&mut self, room: &str) -> Result<()> {
        self.ensure_logged_in()?;
        let frame = build_join_room_request(room);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn add_chatroom(&mut self, room: &str) -> Result<()> {
        self.ensure_logged_in()?;
        let frame = build_add_chatroom_request(room);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn leave_room(&mut self, room: &str) -> Result<()> {
        self.ensure_logged_in()?;
        let frame = build_leave_room_request(room);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn request_room_members(&mut self, room: &str) -> Result<()> {
        self.ensure_logged_in()?;
        let frame = build_room_members_request(room);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn request_room_operators(&mut self, room: &str) -> Result<()> {
        self.ensure_logged_in()?;
        let frame = build_room_operators_request(room);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn request_room_ticker(
        &mut self,
        room: &str,
        timeout: Duration,
    ) -> Result<RoomTickerPayload> {
        self.ensure_logged_in()?;
        let frame = build_get_room_ticker_request(room);
        write_frame(self.stream_mut()?, &frame).await?;

        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            let remaining = deadline.saturating_duration_since(Instant::now());
            match tokio::time::timeout(remaining, self.read_next_frame()).await {
                Ok(Ok(response)) => {
                    if response.code != CODE_SM_GET_ROOM_TICKER {
                        continue;
                    }
                    let Ok(message) = decode_server_message(response.code, &response.payload)
                    else {
                        continue;
                    };
                    if let ServerMessage::RoomTicker(payload) = message {
                        return Ok(payload);
                    }
                }
                Ok(Err(err)) => {
                    if is_connection_eof(&err) {
                        break;
                    }
                    return Err(err);
                }
                Err(_) => break,
            }
        }
        bail!("timed out waiting for room ticker response")
    }

    pub async fn add_room_member(&mut self, room: &str, username: &str) -> Result<()> {
        self.ensure_logged_in()?;
        let frame = build_add_room_member_request(room, username);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn remove_room_member(&mut self, room: &str, username: &str) -> Result<()> {
        self.ensure_logged_in()?;
        let frame = build_remove_room_member_request(room, username);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn add_room_operator(&mut self, room: &str, username: &str) -> Result<()> {
        self.ensure_logged_in()?;
        let frame = build_add_room_operator_request(room, username);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn remove_room_operator(&mut self, room: &str, username: &str) -> Result<()> {
        self.ensure_logged_in()?;
        let frame = build_remove_room_operator_request(room, username);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn ignore_user(&mut self, username: &str) -> Result<()> {
        self.ensure_logged_in()?;
        let frame = build_ignore_user_request(username);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn unignore_user(&mut self, username: &str) -> Result<()> {
        self.ensure_logged_in()?;
        let frame = build_unignore_user_request(username);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn ban_user(&mut self, username: &str) -> Result<()> {
        self.ensure_logged_in()?;
        let frame = build_ban_user_request(username);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn set_upload_speed(&mut self, bytes_per_sec: u32) -> Result<()> {
        self.ensure_logged_in()?;
        let frame = build_upload_speed_request(bytes_per_sec);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn get_own_privileges_status(
        &mut self,
        timeout: Duration,
    ) -> Result<OwnPrivilegesStatusPayload> {
        self.ensure_logged_in()?;
        let frame = build_get_own_privileges_status_request();
        write_frame(self.stream_mut()?, &frame).await?;

        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            let remaining = deadline.saturating_duration_since(Instant::now());
            match tokio::time::timeout(remaining, self.read_next_frame()).await {
                Ok(Ok(response)) => {
                    if response.code != CODE_SM_GET_OWN_PRIVILEGES_STATUS {
                        continue;
                    }
                    let Ok(message) = decode_server_message(response.code, &response.payload)
                    else {
                        continue;
                    };
                    if let ServerMessage::OwnPrivilegesStatus(payload) = message {
                        return Ok(payload);
                    }
                }
                Ok(Err(err)) => {
                    if is_connection_eof(&err) {
                        break;
                    }
                    return Err(err);
                }
                Err(_) => break,
            }
        }

        bail!("timed out waiting for own privileges status response")
    }

    pub async fn get_user_privileges_status(
        &mut self,
        username: &str,
        timeout: Duration,
    ) -> Result<UserPrivilegesStatusPayload> {
        self.ensure_logged_in()?;
        let frame = build_get_user_privileges_status_request(username);
        write_frame(self.stream_mut()?, &frame).await?;

        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            let remaining = deadline.saturating_duration_since(Instant::now());
            match tokio::time::timeout(remaining, self.read_next_frame()).await {
                Ok(Ok(response)) => {
                    if response.code != CODE_SM_GET_USER_PRIVILEGES_STATUS {
                        continue;
                    }
                    let Ok(message) = decode_server_message(response.code, &response.payload)
                    else {
                        continue;
                    };
                    if let ServerMessage::UserPrivilegesStatus(payload) = message {
                        return Ok(payload);
                    }
                }
                Ok(Err(err)) => {
                    if is_connection_eof(&err) {
                        break;
                    }
                    return Err(err);
                }
                Err(_) => break,
            }
        }

        bail!("timed out waiting for user privileges status response")
    }

    pub async fn give_privilege(&mut self, username: &str, days: u32) -> Result<()> {
        self.ensure_logged_in()?;
        let frame = build_give_privilege_request(username, days);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn inform_user_of_privileges(&mut self, token: u32, username: &str) -> Result<()> {
        self.ensure_logged_in()?;
        let frame = build_inform_user_of_privileges_request(token, username);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn inform_user_of_privileges_ack(&mut self, token: u32) -> Result<()> {
        self.ensure_logged_in()?;
        let frame = build_inform_user_of_privileges_ack_request(token);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn say_chatroom(&mut self, room: &str, message: &str) -> Result<()> {
        self.ensure_logged_in()?;
        let frame = build_say_chatroom(room, message);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn send_private_message(&mut self, target_user: &str, message: &str) -> Result<()> {
        self.ensure_logged_in()?;
        let frame = build_message_user_request(target_user, message);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn send_message_users(&mut self, targets: &[String], message: &str) -> Result<()> {
        self.ensure_logged_in()?;
        let frame = build_message_users_request(targets, message);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn wait_message_ack(&mut self, timeout: Duration) -> Result<MessageAckedPayload> {
        self.ensure_logged_in()?;
        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            let remaining = deadline.saturating_duration_since(Instant::now());
            match tokio::time::timeout(remaining, self.read_next_frame()).await {
                Ok(Ok(response)) => {
                    if response.code != CODE_SM_MESSAGE_ACKED {
                        continue;
                    }
                    let Ok(message) = decode_server_message(response.code, &response.payload)
                    else {
                        continue;
                    };
                    if let ServerMessage::MessageAcked(payload) = message {
                        return Ok(payload);
                    }
                }
                Ok(Err(err)) => {
                    if is_connection_eof(&err) {
                        break;
                    }
                    return Err(err);
                }
                Err(_) => break,
            }
        }
        bail!("timed out waiting for message ack")
    }

    pub async fn get_user_status(
        &mut self,
        username: &str,
        timeout: Duration,
    ) -> Result<UserStatusResponsePayload> {
        self.ensure_logged_in()?;
        let frame = build_get_user_status_request(username);
        write_frame(self.stream_mut()?, &frame).await?;

        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            let remaining = deadline.saturating_duration_since(Instant::now());
            match tokio::time::timeout(remaining, self.read_next_frame()).await {
                Ok(Ok(response)) => {
                    if response.code != CODE_SM_GET_USER_STATUS {
                        continue;
                    }
                    let Ok(message) = decode_server_message(response.code, &response.payload)
                    else {
                        continue;
                    };
                    if let ServerMessage::GetUserStatusResponse(payload) = message {
                        return Ok(payload);
                    }
                }
                Ok(Err(err)) => {
                    if is_connection_eof(&err) {
                        break;
                    }
                    return Err(err);
                }
                Err(_) => break,
            }
        }
        bail!("timed out waiting for user status response")
    }

    pub async fn get_user_stats(
        &mut self,
        username: &str,
        timeout: Duration,
    ) -> Result<UserStatsResponsePayload> {
        self.ensure_logged_in()?;
        let frame = build_get_user_stats_request(username);
        write_frame(self.stream_mut()?, &frame).await?;

        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            let remaining = deadline.saturating_duration_since(Instant::now());
            match tokio::time::timeout(remaining, self.read_next_frame()).await {
                Ok(Ok(response)) => {
                    if response.code != CODE_SM_GET_USER_STATS {
                        continue;
                    }
                    let Ok(message) = decode_server_message(response.code, &response.payload)
                    else {
                        continue;
                    };
                    if let ServerMessage::GetUserStatsResponse(payload) = message {
                        return Ok(payload);
                    }
                }
                Ok(Err(err)) => {
                    if is_connection_eof(&err) {
                        break;
                    }
                    return Err(err);
                }
                Err(_) => break,
            }
        }
        bail!("timed out waiting for user stats response")
    }

    pub async fn get_peer_address(
        &mut self,
        username: &str,
        timeout: Duration,
    ) -> Result<PeerAddressResponsePayload> {
        self.ensure_logged_in()?;
        let frame = build_get_peer_address_request(username);
        write_frame(self.stream_mut()?, &frame).await?;

        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            let remaining = deadline.saturating_duration_since(Instant::now());
            match tokio::time::timeout(remaining, self.read_next_frame()).await {
                Ok(Ok(response)) => {
                    if response.code != CODE_SM_GET_PEER_ADDRESS {
                        continue;
                    }
                    let Ok(message) = decode_server_message(response.code, &response.payload)
                    else {
                        continue;
                    };
                    if let ServerMessage::GetPeerAddressResponse(payload) = message {
                        return Ok(payload);
                    }
                }
                Ok(Err(err)) => {
                    if is_connection_eof(&err) {
                        break;
                    }
                    return Err(err);
                }
                Err(_) => break,
            }
        }
        bail!("timed out waiting for peer address response")
    }

    pub async fn connect_to_peer(
        &mut self,
        username: &str,
        token: u32,
        connection_type: &str,
    ) -> Result<()> {
        self.ensure_logged_in()?;
        let frame = build_connect_to_peer_request(token, username, connection_type);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn set_wait_port(&mut self, listen_port: u16) -> Result<()> {
        self.ensure_logged_in()?;
        let frame = encode_server_message(&ServerMessage::SetWaitPort(SetWaitPortPayload {
            listen_port: listen_port as u32,
        }));
        write_frame(self.stream_mut()?, &frame).await
    }

    async fn wait_connect_to_peer_response(
        &mut self,
        username: &str,
        timeout: Duration,
    ) -> Result<ConnectToPeerResponsePayload> {
        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            let remaining = deadline.saturating_duration_since(Instant::now());
            match tokio::time::timeout(remaining, self.read_next_frame()).await {
                Ok(Ok(response)) => {
                    if response.code != protocol::CODE_SM_CONNECT_TO_PEER {
                        continue;
                    }
                    let Ok(message) = decode_server_message(response.code, &response.payload)
                    else {
                        continue;
                    };
                    if let ServerMessage::ConnectToPeerResponse(payload) = message
                        && payload.username.eq_ignore_ascii_case(username)
                    {
                        return Ok(payload);
                    }
                }
                Ok(Err(err)) => {
                    if is_connection_eof(&err) {
                        break;
                    }
                    return Err(err);
                }
                Err(_) => break,
            }
        }
        bail!("timed out waiting for connect-to-peer response");
    }

    async fn download_single_file_via_inbound_wait_port(
        &mut self,
        plan: &DownloadPlan,
        login_username: &str,
        peer_username: &str,
        connect_token: u32,
        wait_port: u16,
        connection_type: &str,
    ) -> Result<DownloadResult> {
        let bind_addr = format!("0.0.0.0:{wait_port}");
        transfer_debug(format!(
            "inbound wait-port flow start wait_port={} peer_user={} connect_token={}",
            wait_port, peer_username, connect_token
        ));
        let listener = TcpListener::bind(&bind_addr)
            .await
            .with_context(|| format!("bind inbound wait-port listener failed: {bind_addr}"))?;
        self.set_wait_port(wait_port).await?;
        self.connect_to_peer(peer_username, connect_token, connection_type)
            .await?;

        let (mut p_stream, p_init, _) = accept_peer_connection_with_init(
            &listener,
            "P",
            None,
            Duration::from_secs(10),
        )
        .await
        .context("accept inbound P connection")?;
        transfer_debug(format!(
            "accepted inbound P connection token={} user={}",
            p_init.token, p_init.username
        ));
        if p_init.token != 0 && p_init.token != connect_token {
            bail!(
                "unexpected inbound P token: expected={} got={}",
                connect_token,
                p_init.token
            );
        }

        let connect_token_frame = build_send_connect_token(login_username, connect_token);
        write_frame(&mut p_stream, &connect_token_frame).await?;

        let transfer_request = send_queue_upload_and_wait_transfer_request(
            &mut p_stream,
            login_username,
            &plan.virtual_path,
            resolve_transfer_flow_timeout(),
        )
        .await?;
        transfer_debug(format!(
            "inbound wait-port received transfer request token={} direction={:?} size={}",
            transfer_request.token, transfer_request.direction, transfer_request.file_size
        ));
        write_transfer_allow(&mut p_stream, transfer_request.token).await?;

        let (mut f_stream, maybe_f_init, addr) =
            accept_peer_file_socket(&listener, Duration::from_secs(45))
                .await
                .context("accept inbound F connection")?;
        if let Some(f_init) = maybe_f_init {
            transfer_debug(format!(
                "accepted inbound F connection with init token={} user={} from={}",
                f_init.token, f_init.username, addr
            ));
        } else {
            transfer_debug(format!(
                "accepted inbound F connection without init from={addr}"
            ));
        }
        ensure_parent_dir(&plan.output_path).await?;
        let expected_size = if transfer_request.file_size == 0 {
            plan.file_size
        } else {
            transfer_request.file_size
        };
        let content = read_file_transfer_content(&mut f_stream, expected_size, transfer_request.token)
            .await?;
        validate_transfer_content_size(content.len(), expected_size)?;
        fs::write(&plan.output_path, &content)
            .await
            .with_context(|| format!("write output file: {}", plan.output_path.display()))?;

        Ok(DownloadResult {
            output_path: plan.output_path.clone(),
            bytes_written: content.len() as u64,
        })
    }

    pub async fn collect_private_events(
        &mut self,
        timeout: Duration,
        max_events: usize,
    ) -> Result<Vec<PrivateEvent>> {
        self.ensure_logged_in()?;
        let mut events = Vec::new();
        let deadline = Instant::now() + timeout;

        while events.len() < max_events {
            let now = Instant::now();
            if now >= deadline {
                break;
            }

            let remaining = deadline.saturating_duration_since(now);
            match tokio::time::timeout(remaining, self.read_next_frame()).await {
                Ok(Ok(frame)) => {
                    let Ok(msg) = decode_server_message(frame.code, &frame.payload) else {
                        continue;
                    };
                    match msg {
                        ServerMessage::MessageUserIncoming(payload) => {
                            events.push(PrivateEvent::Message(payload));
                        }
                        ServerMessage::MessageAcked(payload) => {
                            events.push(PrivateEvent::Ack(payload));
                        }
                        _ => {}
                    }
                }
                Ok(Err(err)) => {
                    if is_connection_eof(&err) {
                        break;
                    }
                    return Err(err);
                }
                Err(_) => break,
            }
        }

        Ok(events)
    }

    pub async fn get_recommendations(
        &mut self,
        timeout: Duration,
    ) -> Result<RecommendationsPayload> {
        self.ensure_logged_in()?;
        let frame = build_get_recommendations_request();
        write_frame(self.stream_mut()?, &frame).await?;
        self.wait_for_recommendations_response(timeout, RecommendationKind::General)
            .await
    }

    pub async fn get_my_recommendations(
        &mut self,
        timeout: Duration,
    ) -> Result<RecommendationsPayload> {
        self.ensure_logged_in()?;
        let frame = build_get_my_recommendations_request();
        write_frame(self.stream_mut()?, &frame).await?;
        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            let remaining = deadline.saturating_duration_since(Instant::now());
            match tokio::time::timeout(remaining, self.read_next_frame()).await {
                Ok(Ok(response)) => {
                    let Ok(message) = decode_server_message(response.code, &response.payload)
                    else {
                        continue;
                    };
                    if let ServerMessage::GetMyRecommendationsResponse(payload) = message {
                        return Ok(payload);
                    }
                }
                Ok(Err(err)) => {
                    if is_connection_eof(&err) {
                        break;
                    }
                    return Err(err);
                }
                Err(_) => break,
            }
        }
        Ok(RecommendationsPayload {
            recommendations: Vec::new(),
            unrecommendations: Vec::new(),
        })
    }

    pub async fn get_global_recommendations(
        &mut self,
        timeout: Duration,
    ) -> Result<RecommendationsPayload> {
        self.ensure_logged_in()?;
        let frame = build_get_global_recommendations_request();
        write_frame(self.stream_mut()?, &frame).await?;
        self.wait_for_recommendations_response(timeout, RecommendationKind::Global)
            .await
    }

    pub async fn get_user_recommendations(
        &mut self,
        username: &str,
        timeout: Duration,
    ) -> Result<UserRecommendationsPayload> {
        self.ensure_logged_in()?;
        let frame = build_get_user_recommendations_request(username);
        write_frame(self.stream_mut()?, &frame).await?;

        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            let remaining = deadline.saturating_duration_since(Instant::now());
            match tokio::time::timeout(remaining, self.read_next_frame()).await {
                Ok(Ok(response)) => {
                    let Ok(message) = decode_server_message(response.code, &response.payload)
                    else {
                        continue;
                    };
                    if let ServerMessage::GetUserRecommendationsResponse(payload) = message {
                        return Ok(payload);
                    }
                }
                Ok(Err(err)) => {
                    if is_connection_eof(&err) {
                        break;
                    }
                    return Err(err);
                }
                Err(_) => break,
            }
        }

        bail!("timed out waiting for user recommendations response")
    }

    pub async fn get_similar_terms(
        &mut self,
        term: &str,
        timeout: Duration,
    ) -> Result<SimilarTermsPayload> {
        self.ensure_logged_in()?;
        let frame = build_get_similar_terms_request(term);
        write_frame(self.stream_mut()?, &frame).await?;

        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            let remaining = deadline.saturating_duration_since(Instant::now());
            match tokio::time::timeout(remaining, self.read_next_frame()).await {
                Ok(Ok(response)) => {
                    let Ok(message) = decode_server_message(response.code, &response.payload)
                    else {
                        continue;
                    };
                    if let ServerMessage::GetSimilarTermsResponse(payload) = message {
                        return Ok(payload);
                    }
                }
                Ok(Err(err)) => {
                    if is_connection_eof(&err) {
                        break;
                    }
                    return Err(err);
                }
                Err(_) => break,
            }
        }

        bail!("timed out waiting for similar terms response")
    }

    pub async fn add_like_term(&mut self, term: &str) -> Result<()> {
        self.ensure_logged_in()?;
        let frame = build_add_like_term_request(term);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn remove_like_term(&mut self, term: &str) -> Result<()> {
        self.ensure_logged_in()?;
        let frame = build_remove_like_term_request(term);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn get_privileged_list(
        &mut self,
        timeout: Duration,
    ) -> Result<PrivilegedListPayload> {
        self.ensure_logged_in()?;
        let frame = build_privileged_list_request();
        write_frame(self.stream_mut()?, &frame).await?;

        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            let remaining = deadline.saturating_duration_since(Instant::now());
            match tokio::time::timeout(remaining, self.read_next_frame()).await {
                Ok(Ok(response)) => {
                    if response.code != CODE_SM_PRIVILEGED_LIST {
                        continue;
                    }
                    let Ok(message) = decode_server_message(response.code, &response.payload)
                    else {
                        continue;
                    };
                    if let ServerMessage::PrivilegedList(payload) = message {
                        return Ok(payload);
                    }
                }
                Ok(Err(err)) => {
                    if is_connection_eof(&err) {
                        break;
                    }
                    return Err(err);
                }
                Err(_) => break,
            }
        }

        bail!("timed out waiting for privileged list response")
    }

    pub async fn get_recommended_users(
        &mut self,
        timeout: Duration,
    ) -> Result<RecommendedUsersPayload> {
        self.ensure_logged_in()?;
        let frame = build_get_recommended_users_request();
        write_frame(self.stream_mut()?, &frame).await?;

        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            let remaining = deadline.saturating_duration_since(Instant::now());
            match tokio::time::timeout(remaining, self.read_next_frame()).await {
                Ok(Ok(response)) => {
                    if response.code != CODE_SM_GET_RECOMMENDED_USERS {
                        continue;
                    }
                    let Ok(message) = decode_server_message(response.code, &response.payload)
                    else {
                        continue;
                    };
                    if let ServerMessage::GetRecommendedUsersResponse(payload) = message {
                        return Ok(payload);
                    }
                }
                Ok(Err(err)) => {
                    if is_connection_eof(&err) {
                        break;
                    }
                    return Err(err);
                }
                Err(_) => break,
            }
        }

        bail!("timed out waiting for recommended users response")
    }

    pub async fn get_term_recommendations(
        &mut self,
        term: &str,
        timeout: Duration,
    ) -> Result<TermRecommendationsPayload> {
        self.ensure_logged_in()?;
        let frame = build_get_term_recommendations_request(term);
        write_frame(self.stream_mut()?, &frame).await?;

        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            let remaining = deadline.saturating_duration_since(Instant::now());
            match tokio::time::timeout(remaining, self.read_next_frame()).await {
                Ok(Ok(response)) => {
                    if response.code != CODE_SM_GET_TERM_RECOMMENDATIONS {
                        continue;
                    }
                    let Ok(message) = decode_server_message(response.code, &response.payload)
                    else {
                        continue;
                    };
                    match message {
                        ServerMessage::GetTermRecommendationsResponse(payload) => {
                            return Ok(payload);
                        }
                        // Code 111 request/response payloads are wire-ambiguous when the response
                        // carries zero entries (term-only payload). Treat request-classified
                        // decodes as empty responses to avoid false timeouts.
                        ServerMessage::GetTermRecommendations(request) => {
                            return Ok(TermRecommendationsPayload {
                                term: request.term,
                                recommendations: Vec::new(),
                            });
                        }
                        _ => {}
                    }
                }
                Ok(Err(err)) => {
                    if is_connection_eof(&err) {
                        break;
                    }
                    return Err(err);
                }
                Err(_) => break,
            }
        }

        bail!("timed out waiting for term recommendations response")
    }

    pub async fn get_recommendation_users(
        &mut self,
        term: &str,
        timeout: Duration,
    ) -> Result<RecommendationUsersPayload> {
        self.ensure_logged_in()?;
        let frame = build_get_recommendation_users_request(term);
        write_frame(self.stream_mut()?, &frame).await?;

        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            let remaining = deadline.saturating_duration_since(Instant::now());
            match tokio::time::timeout(remaining, self.read_next_frame()).await {
                Ok(Ok(response)) => {
                    if response.code != CODE_SM_GET_RECOMMENDATION_USERS {
                        continue;
                    }
                    let Ok(message) = decode_server_message(response.code, &response.payload)
                    else {
                        continue;
                    };
                    match message {
                        ServerMessage::GetRecommendationUsersResponse(payload) => {
                            return Ok(payload);
                        }
                        // Code 112 has the same term-only ambiguity as code 111 for zero-entry
                        // responses; normalize to an empty users response.
                        ServerMessage::GetRecommendationUsers(request) => {
                            return Ok(RecommendationUsersPayload {
                                term: request.term,
                                users: Vec::new(),
                            });
                        }
                        _ => {}
                    }
                }
                Ok(Err(err)) => {
                    if is_connection_eof(&err) {
                        break;
                    }
                    return Err(err);
                }
                Err(_) => break,
            }
        }

        bail!("timed out waiting for recommendation users response")
    }

    pub async fn send_server_message(&mut self, message: &ServerMessage) -> Result<()> {
        self.ensure_connected()?;
        let frame = encode_server_message(message);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn send_peer_message(&mut self, message: &PeerMessage) -> Result<()> {
        self.ensure_connected()?;
        let frame = encode_peer_message(message);
        write_frame(self.stream_mut()?, &frame).await
    }

    pub async fn read_next_frame(&mut self) -> Result<Frame> {
        self.ensure_connected()?;
        read_frame(self.stream_mut()?).await
    }

    pub async fn read_next_message(&mut self) -> Result<ProtocolMessage> {
        let frame = self.read_next_frame().await?;
        let server = decode_server_message(frame.code, &frame.payload)?;
        Ok(ProtocolMessage::Server(server))
    }

    pub async fn search_and_collect(
        &mut self,
        token: u32,
        query: &str,
        timeout: Duration,
        max_messages: usize,
    ) -> Result<Vec<ServerMessage>> {
        self.search(token, query).await?;

        let mut collected = Vec::new();
        let deadline = Instant::now() + timeout;

        while collected.len() < max_messages {
            let now = Instant::now();
            if now >= deadline {
                break;
            }

            let remaining = deadline.saturating_duration_since(now);
            match tokio::time::timeout(remaining, self.read_next_frame()).await {
                Ok(Ok(frame)) => {
                    if let Ok(msg) = decode_server_message(frame.code, &frame.payload) {
                        collected.push(msg);
                    }
                }
                Ok(Err(err)) => {
                    if is_connection_eof(&err) {
                        break;
                    }
                    return Err(err);
                }
                Err(_) => break,
            }
        }

        Ok(collected)
    }

    pub async fn search_select_and_download(
        &mut self,
        request: &SearchSelectDownloadRequest,
    ) -> std::result::Result<SearchSelectDownloadResult, SearchSelectDownloadError> {
        self.ensure_logged_in()
            .map_err(|err| SearchSelectDownloadError::Session(err.to_string()))?;
        let candidates = self
            .search_collect_candidates(
                request.search_token,
                &request.query,
                request.search_timeout,
                request.max_messages,
                request.search_mode,
                request.strict_track.as_deref(),
                &request.connection_type,
            )
            .await
            .map_err(|err| {
                let rendered = err.to_string();
                if rendered.contains("no reachable peer candidates") {
                    SearchSelectDownloadError::NoReachablePeerCandidates
                } else if rendered.contains("distributed search handshake failed") {
                    SearchSelectDownloadError::DistributedSearchHandshakeFailed
                } else if rendered.contains("distributed search found no matching track") {
                    SearchSelectDownloadError::DistributedSearchNoMatchingTrack {
                        query: request.query.clone(),
                    }
                } else {
                    SearchSelectDownloadError::Search(rendered)
                }
            })?;

        if candidates.is_empty() {
            return if request.search_mode == SearchMode::Distributed {
                Err(
                    SearchSelectDownloadError::DistributedSearchNoMatchingTrack {
                        query: request.query.clone(),
                    },
                )
            } else {
                Err(SearchSelectDownloadError::NoSearchResults {
                    query: request.query.clone(),
                })
            };
        }

        let selected = candidates.get(request.result_index).ok_or_else(|| {
            SearchSelectDownloadError::InvalidSearchResultIndex {
                index: request.result_index,
                available: candidates.len(),
            }
        })?;

        if selected.source == SearchResultSource::DistributedPeer
            && request.peer_addr_override.is_none()
        {
            transfer_debug(format!(
                "distributed download selection: query='{}' candidates={} result_index={}",
                request.query,
                candidates.len(),
                request.result_index
            ));
            let transfer_connection_type = if request.connection_type.eq_ignore_ascii_case("P") {
                "F".to_string()
            } else {
                request.connection_type.clone()
            };
            let wait_port = resolve_wait_port(request.wait_port);
            let login_username = self
                .logged_username
                .clone()
                .unwrap_or_else(|| "neosoulseek".to_string());
            if let Some(configured_wait_port) = wait_port
                && let Err(err) = self.set_wait_port(configured_wait_port).await
            {
                transfer_debug(format!(
                    "set-wait-port preflight failed on {}: {}",
                    configured_wait_port,
                    format_error_chain(&err)
                ));
            }
            let mut last_error = None::<String>;
            for (attempt_index, candidate) in candidates
                .iter()
                .skip(request.result_index)
                .chain(candidates.iter().take(request.result_index))
                .take(resolve_max_candidate_attempts())
                .enumerate()
            {
                if candidate.source != SearchResultSource::DistributedPeer {
                    continue;
                }
                let Some(candidate_peer_addr) = candidate.peer_addr.clone() else {
                    continue;
                };
                transfer_debug(format!(
                    "candidate user={} peer={} path={} token={}",
                    candidate.username,
                    candidate_peer_addr,
                    candidate.file_path,
                    candidate.connect_token.unwrap_or(request.transfer_token)
                ));
                let mut peer_addr = candidate_peer_addr;
                let mut wire_token = candidate.connect_token.unwrap_or(request.transfer_token);
                let mut attempt_errors = Vec::new();

                if !request.skip_connect_probe
                    && let Some(inbound_wait_port) = wait_port
                {
                    let inbound_plan = DownloadPlan {
                        peer_addr: peer_addr.clone(),
                        token: request.transfer_token,
                        virtual_path: candidate.file_path.clone(),
                        file_size: candidate.file_size,
                        output_path: request.output_path.clone(),
                    };
                    match tokio::time::timeout(
                        Duration::from_secs(25),
                        self.download_single_file_via_inbound_wait_port(
                            &inbound_plan,
                            &login_username,
                            &candidate.username,
                            wire_token,
                            inbound_wait_port,
                            &request.connection_type,
                        ),
                    )
                    .await
                    {
                        Ok(Ok(download_result)) => {
                            transfer_debug("inbound wait-port flow succeeded");
                            return Ok(SearchSelectDownloadResult {
                                selected_username: candidate.username.clone(),
                                selected_virtual_path: candidate.file_path.clone(),
                                selected_file_size: candidate.file_size,
                                peer_addr,
                                transfer_token: request.transfer_token,
                                output_path: download_result.output_path,
                                bytes_written: download_result.bytes_written,
                                search_source: candidate.source,
                            });
                        }
                        Ok(Err(err)) => {
                            let rendered = format_error_chain(&err);
                            transfer_debug(format!("inbound wait-port flow failed: {rendered}"));
                            attempt_errors.push(format!(
                                "inbound wait-port flow failed (port {inbound_wait_port}): {rendered}"
                            ));
                        }
                        Err(_) => {
                            transfer_debug("inbound wait-port flow timed out");
                            attempt_errors.push(format!(
                                "inbound wait-port flow timed out (port {inbound_wait_port})"
                            ));
                        }
                    }
                }

                if !request.skip_connect_probe {
                    let probe_token_p = request
                        .transfer_token
                        .wrapping_add((attempt_index as u32).wrapping_mul(17))
                        .wrapping_add(7);
                    if let Err(err) = self
                        .connect_to_peer(&candidate.username, probe_token_p, &request.connection_type)
                        .await
                    {
                        let rendered = format_error_chain(&err);
                        transfer_debug(format!(
                            "connect-to-peer probe failed, keeping candidate address: {rendered}"
                        ));
                        attempt_errors.push(format!(
                            "connect-to-peer probe failed: {rendered}; using candidate address"
                        ));
                    } else {
                        match self
                            .wait_connect_to_peer_response(
                                &candidate.username,
                                Duration::from_secs(12),
                            )
                            .await
                        {
                            Ok(response) => {
                                peer_addr = format!("{}:{}", response.ip_address, response.port);
                                wire_token = response.token;
                                transfer_debug(format!(
                                    "connect-to-peer response accepted: peer={} token={}",
                                    peer_addr, wire_token
                                ));
                            }
                            Err(err) => {
                                let rendered = format_error_chain(&err);
                                transfer_debug(format!(
                                    "connect-to-peer response wait failed, keeping candidate address: {rendered}"
                                ));
                                attempt_errors.push(format!(
                                    "connect-to-peer response wait failed: {rendered}; using candidate address"
                                ));
                            }
                        }
                    }
                    tokio::time::sleep(Duration::from_millis(120)).await;
                }
                let mut file_peer_addr = peer_addr.clone();
                let mut file_connect_token = wire_token;
                if !request.skip_connect_probe {
                    let probe_token_f = request
                        .transfer_token
                        .wrapping_add((attempt_index as u32).wrapping_mul(17))
                        .wrapping_add(13);
                    if let Err(err) = self
                        .connect_to_peer(&candidate.username, probe_token_f, "F")
                        .await
                    {
                        transfer_debug(format!(
                            "connect-to-peer F probe failed, keeping control peer address: {}",
                            format_error_chain(&err)
                        ));
                    } else {
                        match self
                            .wait_connect_to_peer_response(
                                &candidate.username,
                                Duration::from_secs(12),
                            )
                            .await
                        {
                            Ok(response) => {
                                file_peer_addr =
                                    format!("{}:{}", response.ip_address, response.port);
                                file_connect_token = response.token;
                                transfer_debug(format!(
                                    "connect-to-peer F response accepted: peer={} token={}",
                                    file_peer_addr, file_connect_token
                                ));
                            }
                            Err(err) => {
                                transfer_debug(format!(
                                    "connect-to-peer F response wait failed, using control peer address: {}",
                                    format_error_chain(&err)
                                ));
                            }
                        }
                    }
                }
                let plan = DownloadPlan {
                    peer_addr: peer_addr.clone(),
                    token: request.transfer_token,
                    virtual_path: candidate.file_path.clone(),
                    file_size: candidate.file_size,
                    output_path: request.output_path.clone(),
                };
                if !resolve_skip_transfer_request_flow() {
                    let transfer_flow_timeout = resolve_transfer_flow_timeout();
                    let transfer_request_flow = tokio::time::timeout(
                        transfer_flow_timeout,
                        download_single_file_via_transfer_request(
                            &plan,
                            &login_username,
                            wire_token,
                            wait_port,
                            Some(&file_peer_addr),
                            Some(file_connect_token),
                        ),
                    )
                    .await;
                    match transfer_request_flow {
                        Ok(Ok(download_result)) => {
                            transfer_debug("transfer-request flow succeeded");
                            return Ok(SearchSelectDownloadResult {
                                selected_username: candidate.username.clone(),
                                selected_virtual_path: candidate.file_path.clone(),
                                selected_file_size: candidate.file_size,
                                peer_addr,
                                transfer_token: request.transfer_token,
                                output_path: download_result.output_path,
                                bytes_written: download_result.bytes_written,
                                search_source: candidate.source,
                            });
                        }
                        Ok(Err(err)) => {
                            let rendered = err.to_string();
                            transfer_debug(format!("transfer-request flow failed: {rendered}"));
                            attempt_errors.push(format!("transfer-request flow failed: {rendered}"));
                        }
                        Err(_) => {
                            transfer_debug("transfer-request flow timed out");
                            attempt_errors.push("transfer-request flow timed out".to_string());
                        }
                    }
                } else {
                    transfer_debug("transfer-request flow skipped by NSS_SKIP_TRANSFER_REQUEST_FLOW");
                }
                let queue_flow_timeout = resolve_transfer_flow_timeout();
                let modern_queue = tokio::time::timeout(
                    queue_flow_timeout,
                    download_single_file_via_queue_upload(
                        &plan,
                        &login_username,
                        &candidate.username,
                        wire_token,
                        wait_port,
                        Some(&file_peer_addr),
                        Some(file_connect_token),
                    ),
                )
                .await;
                match modern_queue {
                    Ok(Ok(download_result)) => {
                        transfer_debug("queue-upload flow succeeded");
                        return Ok(SearchSelectDownloadResult {
                            selected_username: candidate.username.clone(),
                            selected_virtual_path: candidate.file_path.clone(),
                            selected_file_size: candidate.file_size,
                            peer_addr,
                            transfer_token: request.transfer_token,
                            output_path: download_result.output_path,
                            bytes_written: download_result.bytes_written,
                            search_source: candidate.source,
                        });
                    }
                    Ok(Err(err)) => {
                        let rendered = format_error_chain(&err);
                        transfer_debug(format!("queue-upload flow failed: {rendered}"));
                        attempt_errors.push(format!("queue-upload flow failed: {rendered}"));
                    }
                    Err(_) => {
                        transfer_debug("queue-upload flow timed out");
                        attempt_errors.push("queue-upload flow timed out".to_string());
                    }
                }

                match tokio::time::timeout(
                    Duration::from_secs(90),
                    download_single_file_with_peer_init(
                        &DownloadPlan {
                            peer_addr: file_peer_addr.clone(),
                            token: plan.token,
                            virtual_path: plan.virtual_path.clone(),
                            file_size: plan.file_size,
                            output_path: plan.output_path.clone(),
                        },
                        &login_username,
                        &transfer_connection_type,
                        file_connect_token,
                    ),
                )
                .await
                {
                    Ok(Ok(download_result)) => {
                        transfer_debug("direct transfer flow succeeded");
                        return Ok(SearchSelectDownloadResult {
                            selected_username: candidate.username.clone(),
                            selected_virtual_path: candidate.file_path.clone(),
                            selected_file_size: candidate.file_size,
                            peer_addr,
                            transfer_token: request.transfer_token,
                            output_path: download_result.output_path,
                            bytes_written: download_result.bytes_written,
                            search_source: candidate.source,
                        });
                    }
                    Ok(Err(err)) => {
                        let rendered = format_error_chain(&err);
                        transfer_debug(format!("direct flow failed: {rendered}"));
                        attempt_errors.push(format!("direct flow failed: {rendered}"));
                    }
                    Err(_) => {
                        transfer_debug("direct flow timed out");
                        attempt_errors.push("direct flow timed out".to_string());
                    }
                }
                if !attempt_errors.is_empty() {
                    last_error = Some(attempt_errors.join(" | "));
                }
            }

            return Err(SearchSelectDownloadError::Download(format!(
                "all distributed candidates failed: {}",
                last_error.unwrap_or_else(|| "no candidate could be downloaded".to_string())
            )));
        }

        let peer_addr = if let Some(override_addr) = request.peer_addr_override.clone() {
            override_addr
        } else if let Some(candidate_addr) = selected.peer_addr.clone() {
            candidate_addr
        } else {
            let payload = self
                .get_peer_address(&selected.username, request.peer_lookup_timeout)
                .await
                .map_err(|err| SearchSelectDownloadError::PeerLookup(err.to_string()))?;
            format!("{}:{}", payload.ip_address, payload.port)
        };
        let wire_token = selected.connect_token.unwrap_or(request.transfer_token);
        let transfer_connection_type = if request.connection_type.eq_ignore_ascii_case("P") {
            "F".to_string()
        } else {
            request.connection_type.clone()
        };

        if selected.source == SearchResultSource::ServerSummary && !request.skip_connect_probe {
            self.connect_to_peer(
                &selected.username,
                request.transfer_token,
                &request.connection_type,
            )
            .await
            .map_err(|err| SearchSelectDownloadError::ConnectToPeer(err.to_string()))?;
        } else if selected.source == SearchResultSource::DistributedPeer
            && !request.skip_connect_probe
        {
            self.connect_to_peer(&selected.username, wire_token, &request.connection_type)
                .await
                .map_err(|err| SearchSelectDownloadError::ConnectToPeer(err.to_string()))?;
            tokio::time::sleep(Duration::from_millis(120)).await;
        }

        let plan = DownloadPlan {
            peer_addr: peer_addr.clone(),
            token: request.transfer_token,
            virtual_path: selected.file_path.clone(),
            file_size: selected.file_size,
            output_path: request.output_path.clone(),
        };
        let download_result = if selected.source == SearchResultSource::DistributedPeer {
            let login_username = self
                .logged_username
                .clone()
                .unwrap_or_else(|| "neosoulseek".to_string());
            let mut inbound_error = None::<String>;
            if !request.skip_connect_probe
                && let Some(wait_port) = resolve_wait_port(request.wait_port)
            {
                match tokio::time::timeout(
                    Duration::from_secs(25),
                    self.download_single_file_via_inbound_wait_port(
                        &plan,
                        &login_username,
                        &selected.username,
                        wire_token,
                        wait_port,
                        &request.connection_type,
                    ),
                )
                .await
                {
                    Ok(Ok(download_result)) => download_result,
                    Ok(Err(err)) => {
                        inbound_error = Some(format!(
                            "inbound wait-port flow failed (port {wait_port}): {err}"
                        ));
                        match tokio::time::timeout(
                            Duration::from_secs(45),
                            download_single_file_via_queue_upload(
                                &plan,
                                &login_username,
                                &selected.username,
                                wire_token,
                                resolve_wait_port(request.wait_port),
                                None,
                                None,
                            ),
                        )
                        .await
                        {
                            Ok(Ok(download_result)) => download_result,
                            Ok(Err(queue_err)) => {
                                download_single_file_with_peer_init(
                                    &plan,
                                    &login_username,
                                    &transfer_connection_type,
                                    wire_token,
                                )
                                .await
                                .map_err(|err| {
                                    SearchSelectDownloadError::Download(format!(
                                        "{}; queue-upload flow failed: {queue_err}; direct flow failed: {err}",
                                        inbound_error
                                            .clone()
                                            .unwrap_or_else(|| "inbound flow failed".to_string())
                                    ))
                                })?
                            }
                            Err(_) => {
                                download_single_file_with_peer_init(
                                    &plan,
                                    &login_username,
                                    &transfer_connection_type,
                                    wire_token,
                                )
                                .await
                                .map_err(|err| {
                                    SearchSelectDownloadError::Download(format!(
                                        "{}; queue-upload flow timed out; direct flow failed: {err}",
                                        inbound_error
                                            .clone()
                                            .unwrap_or_else(|| "inbound flow failed".to_string())
                                    ))
                                })?
                            }
                        }
                    }
                    Err(_) => {
                        inbound_error = Some(format!(
                            "inbound wait-port flow timed out (port {wait_port})"
                        ));
                        match tokio::time::timeout(
                            Duration::from_secs(45),
                            download_single_file_via_queue_upload(
                                &plan,
                                &login_username,
                                &selected.username,
                                wire_token,
                                resolve_wait_port(request.wait_port),
                                None,
                                None,
                            ),
                        )
                        .await
                        {
                            Ok(Ok(download_result)) => download_result,
                            Ok(Err(queue_err)) => {
                                download_single_file_with_peer_init(
                                    &plan,
                                    &login_username,
                                    &transfer_connection_type,
                                    wire_token,
                                )
                                .await
                                .map_err(|err| {
                                    SearchSelectDownloadError::Download(format!(
                                        "{}; queue-upload flow failed: {queue_err}; direct flow failed: {err}",
                                        inbound_error
                                            .clone()
                                            .unwrap_or_else(|| "inbound flow timed out".to_string())
                                    ))
                                })?
                            }
                            Err(_) => {
                                download_single_file_with_peer_init(
                                    &plan,
                                    &login_username,
                                    &transfer_connection_type,
                                    wire_token,
                                )
                                .await
                                .map_err(|err| {
                                    SearchSelectDownloadError::Download(format!(
                                        "{}; queue-upload flow timed out; direct flow failed: {err}",
                                        inbound_error
                                            .clone()
                                            .unwrap_or_else(|| "inbound flow timed out".to_string())
                                    ))
                                })?
                            }
                        }
                    }
                }
            } else {
                match tokio::time::timeout(
                    Duration::from_secs(45),
                    download_single_file_via_queue_upload(
                        &plan,
                        &login_username,
                        &selected.username,
                        wire_token,
                        resolve_wait_port(request.wait_port),
                        None,
                        None,
                    ),
                )
                .await
                {
                    Ok(Ok(download_result)) => download_result,
                    Ok(Err(queue_err)) => download_single_file_with_peer_init(
                        &plan,
                        &login_username,
                        &transfer_connection_type,
                        wire_token,
                    )
                    .await
                    .map_err(|err| {
                        SearchSelectDownloadError::Download(format!(
                            "queue-upload flow failed: {queue_err}; direct flow failed: {err}"
                        ))
                    })?,
                    Err(_) => download_single_file_with_peer_init(
                        &plan,
                        &login_username,
                        &transfer_connection_type,
                        wire_token,
                    )
                    .await
                    .map_err(|err| {
                        SearchSelectDownloadError::Download(format!(
                            "queue-upload flow timed out; direct flow failed: {err}"
                        ))
                    })?,
                }
            }
        } else {
            download_single_file(&plan)
                .await
                .map_err(|err| SearchSelectDownloadError::Download(err.to_string()))?
        };

        Ok(SearchSelectDownloadResult {
            selected_username: selected.username.clone(),
            selected_virtual_path: selected.file_path.clone(),
            selected_file_size: selected.file_size,
            peer_addr,
            transfer_token: request.transfer_token,
            output_path: download_result.output_path,
            bytes_written: download_result.bytes_written,
            search_source: selected.source,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn search_collect_candidates(
        &mut self,
        token: u32,
        query: &str,
        timeout: Duration,
        max_messages: usize,
        mode: SearchMode,
        strict_track: Option<&str>,
        connection_type: &str,
    ) -> Result<Vec<SearchCandidate>> {
        self.ensure_logged_in()?;
        let collected = self
            .search_and_collect(token, query, timeout, max_messages)
            .await?;
        let summary_candidates = flatten_summary_candidates(&collect_search_summaries(&collected));

        if mode != SearchMode::Distributed && !summary_candidates.is_empty() {
            return Ok(summary_candidates);
        }
        if mode == SearchMode::Summary {
            return Ok(summary_candidates);
        }

        let connect_candidates = collect_connect_candidates(&collected);
        if connect_candidates.is_empty() {
            return if mode == SearchMode::Distributed {
                bail!(SearchSelectDownloadError::NoReachablePeerCandidates)
            } else {
                Ok(summary_candidates)
            };
        }

        let logged_username = self
            .logged_username
            .clone()
            .unwrap_or_else(|| "neosoulseek".to_string());
        let distributed = collect_distributed_candidates(
            &logged_username,
            &connect_candidates,
            token,
            query,
            strict_track,
            connection_type,
        )
        .await;

        if distributed.reachable == 0 {
            bail!(SearchSelectDownloadError::NoReachablePeerCandidates);
        }
        if distributed.handshake_ready == 0 {
            bail!(SearchSelectDownloadError::DistributedSearchHandshakeFailed);
        }
        if distributed.hits.is_empty() {
            return if mode == SearchMode::Distributed {
                bail!(
                    SearchSelectDownloadError::DistributedSearchNoMatchingTrack {
                        query: query.to_owned(),
                    }
                )
            } else {
                Ok(summary_candidates)
            };
        }

        let mut candidates = distributed
            .hits
            .into_iter()
            .map(|hit| SearchCandidate {
                username: hit.username,
                file_path: hit.file_path,
                file_size: hit.file_size,
                peer_addr: Some(hit.peer_addr),
                connect_token: Some(hit.connect_token),
                source: SearchResultSource::DistributedPeer,
            })
            .collect::<Vec<_>>();

        if mode == SearchMode::Auto && !summary_candidates.is_empty() {
            return Ok(summary_candidates);
        }
        if mode != SearchMode::Distributed && candidates.is_empty() {
            return Ok(summary_candidates);
        }

        candidates.sort_by(|a, b| {
            b.file_size
                .cmp(&a.file_size)
                .then_with(|| a.file_path.cmp(&b.file_path))
        });
        Ok(candidates)
    }

    pub async fn collect_room_events(
        &mut self,
        timeout: Duration,
        max_events: usize,
    ) -> Result<Vec<RoomEvent>> {
        self.ensure_logged_in()?;
        let mut events = Vec::new();
        let deadline = Instant::now() + timeout;

        while events.len() < max_events {
            let now = Instant::now();
            if now >= deadline {
                break;
            }

            let remaining = deadline.saturating_duration_since(now);
            match tokio::time::timeout(remaining, self.read_next_frame()).await {
                Ok(Ok(frame)) => {
                    let Ok(msg) = decode_server_message(frame.code, &frame.payload) else {
                        continue;
                    };
                    match msg {
                        ServerMessage::UserJoinedRoom(payload) => {
                            events.push(RoomEvent::UserJoined {
                                room: payload.room,
                                username: payload.username,
                            });
                        }
                        ServerMessage::UserLeftRoom(payload) => {
                            events.push(RoomEvent::UserLeft {
                                room: payload.room,
                                username: payload.username,
                            });
                        }
                        ServerMessage::SayChatRoom(payload) => {
                            events.push(RoomEvent::RoomMessage {
                                room: payload.room,
                                username: payload.username,
                                message: payload.message,
                            });
                        }
                        ServerMessage::RoomMembers(payload) => {
                            events.push(RoomEvent::MembersSnapshot(payload));
                        }
                        ServerMessage::RoomOperators(payload) => {
                            events.push(RoomEvent::OperatorsSnapshot(payload));
                        }
                        ServerMessage::RoomTicker(payload) => {
                            events.push(RoomEvent::TickerSnapshot(payload));
                        }
                        ServerMessage::JoinRoom(payload) => {
                            if !payload.users.is_empty() {
                                events.push(RoomEvent::MembersSnapshot(RoomMembersPayload {
                                    room: payload.room,
                                    users: payload.users,
                                }));
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Err(err)) => {
                    if is_connection_eof(&err) {
                        break;
                    }
                    return Err(err);
                }
                Err(_) => break,
            }
        }

        Ok(events)
    }

    fn ensure_connected(&self) -> Result<()> {
        if self.state == SessionState::Disconnected || self.stream.is_none() {
            bail!("session is not connected");
        }
        Ok(())
    }

    fn ensure_logged_in(&self) -> Result<()> {
        if self.state != SessionState::LoggedIn {
            bail!("session is not logged in");
        }
        Ok(())
    }

    fn stream_mut(&mut self) -> Result<&mut TcpStream> {
        self.stream
            .as_mut()
            .ok_or_else(|| anyhow!("session stream is unavailable"))
    }

    async fn wait_for_recommendations_response(
        &mut self,
        timeout: Duration,
        kind: RecommendationKind,
    ) -> Result<RecommendationsPayload> {
        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            let remaining = deadline.saturating_duration_since(Instant::now());
            match tokio::time::timeout(remaining, self.read_next_frame()).await {
                Ok(Ok(response)) => {
                    let Ok(message) = decode_server_message(response.code, &response.payload)
                    else {
                        continue;
                    };
                    match (kind, message) {
                        (
                            RecommendationKind::General,
                            ServerMessage::GetRecommendationsResponse(payload),
                        ) => return Ok(payload),
                        (
                            RecommendationKind::Global,
                            ServerMessage::GetGlobalRecommendationsResponse(payload),
                        ) => return Ok(payload),
                        _ => {}
                    }
                }
                Ok(Err(err)) => {
                    if is_connection_eof(&err) {
                        break;
                    }
                    return Err(err);
                }
                Err(_) => break,
            }
        }

        match kind {
            RecommendationKind::General => bail!("timed out waiting for recommendations response"),
            RecommendationKind::Global => {
                bail!("timed out waiting for global recommendations response")
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RecommendationKind {
    General,
    Global,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AuthError {
    #[error("login rejected: INVALIDVERSION")]
    InvalidVersion,
    #[error("login rejected: INVALIDPASS")]
    InvalidPass,
    #[error("login rejected: INVALIDUSERNAME")]
    InvalidUsername,
    #[error("server closed before login response (possible invalid account/registration/ban)")]
    ServerClosedBeforeLoginResponse,
    #[error("login response decode failure: {0}")]
    ProtocolDecode(String),
    #[error("login response timed out")]
    Timeout,
}

impl AuthError {
    fn from_failure_reason(reason: LoginFailureReason, detail: Option<String>) -> Self {
        match reason {
            LoginFailureReason::InvalidVersion => Self::InvalidVersion,
            LoginFailureReason::InvalidPass => Self::InvalidPass,
            LoginFailureReason::InvalidUsername => Self::InvalidUsername,
            LoginFailureReason::Unknown(value) => {
                let mut message = format!("unknown login failure reason: {value}");
                if let Some(extra) = detail {
                    message.push_str(" detail=");
                    message.push_str(&extra);
                }
                Self::ProtocolDecode(message)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SearchSelectDownloadError {
    #[error("session precondition failed: {0}")]
    Session(String),
    #[error("search request failed: {0}")]
    Search(String),
    #[error("no search results were decoded for query: {query}")]
    NoSearchResults { query: String },
    #[error("search result index out of range: index={index} available={available}")]
    InvalidSearchResultIndex { index: usize, available: usize },
    #[error(
        "search file index out of range: result_index={result_index} file_index={file_index} available={available}"
    )]
    InvalidSearchFileIndex {
        result_index: usize,
        file_index: usize,
        available: usize,
    },
    #[error("peer address lookup failed: {0}")]
    PeerLookup(String),
    #[error("connect-to-peer probe failed: {0}")]
    ConnectToPeer(String),
    #[error("no reachable peer candidates were available for distributed search")]
    NoReachablePeerCandidates,
    #[error("distributed search handshake failed for all peer candidates")]
    DistributedSearchHandshakeFailed,
    #[error("distributed search found no matching track for query: {query}")]
    DistributedSearchNoMatchingTrack { query: String },
    #[error("download execution failed: {0}")]
    Download(String),
}

fn collect_search_summaries(messages: &[ServerMessage]) -> Vec<SearchResponseSummary> {
    messages
        .iter()
        .filter_map(|message| {
            if let ServerMessage::FileSearchResponseSummary(summary) = message {
                Some(summary.clone())
            } else {
                None
            }
        })
        .collect()
}

fn flatten_summary_candidates(summaries: &[SearchResponseSummary]) -> Vec<SearchCandidate> {
    let mut rows = Vec::new();
    for summary in summaries {
        for file in &summary.files {
            rows.push(SearchCandidate {
                username: summary.username.clone(),
                file_path: file.file_path.clone(),
                file_size: file.file_size,
                peer_addr: None,
                connect_token: None,
                source: SearchResultSource::ServerSummary,
            });
        }
    }
    rows
}

fn collect_connect_candidates(messages: &[ServerMessage]) -> Vec<ConnectToPeerResponsePayload> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for message in messages {
        let ServerMessage::ConnectToPeerResponse(payload) = message else {
            continue;
        };
        if payload.ip_address == "0.0.0.0" || payload.port == 0 {
            continue;
        }
        let peer_addr = format!("{}:{}", payload.ip_address, payload.port);
        if seen.insert((payload.username.clone(), payload.token, peer_addr.clone())) {
            out.push(payload.clone());
        }
    }
    out
}

#[derive(Debug, Clone)]
struct DistributedSearchHit {
    username: String,
    peer_addr: String,
    file_path: String,
    file_size: u64,
    connect_token: u32,
    score: i64,
}

#[derive(Debug, Default)]
struct DistributedSearchCollection {
    hits: Vec<DistributedSearchHit>,
    reachable: usize,
    handshake_ready: usize,
}

#[allow(clippy::too_many_arguments)]
async fn collect_distributed_candidates(
    logged_username: &str,
    candidates: &[ConnectToPeerResponsePayload],
    token: u32,
    query: &str,
    strict_track: Option<&str>,
    connection_type: &str,
) -> DistributedSearchCollection {
    let mut collection = DistributedSearchCollection::default();
    let mut seen_hits = HashSet::new();
    let per_peer_connect_timeout = Duration::from_millis(1000);
    let per_peer_read_window = Duration::from_millis(1500);

    for candidate in candidates.iter().take(64) {
        let peer_addr = format!("{}:{}", candidate.ip_address, candidate.port);
        let connect =
            tokio::time::timeout(per_peer_connect_timeout, TcpStream::connect(&peer_addr)).await;
        let Ok(Ok(mut stream)) = connect else {
            continue;
        };
        collection.reachable += 1;

        if write_peer_init_frame(
            &mut stream,
            logged_username,
            connection_type,
            candidate.token,
        )
        .await
        .is_err()
        {
            continue;
        }

        let search_frame =
            encode_peer_message(&PeerMessage::FileSearchRequest(FileSearchRequestPayload {
                token,
                query: query.to_owned(),
            }));
        if write_frame(&mut stream, &search_frame).await.is_err() {
            continue;
        }
        collection.handshake_ready += 1;

        let deadline = Instant::now() + per_peer_read_window;
        let mut frame_budget = 6usize;
        while frame_budget > 0 && Instant::now() < deadline {
            frame_budget = frame_budget.saturating_sub(1);
            let remaining = deadline.saturating_duration_since(Instant::now());
            let frame = match tokio::time::timeout(remaining, read_frame(&mut stream)).await {
                Ok(Ok(frame)) => frame,
                Ok(Err(_)) | Err(_) => break,
            };
            if frame.code != CODE_PM_FILE_SEARCH_RESULT {
                continue;
            }
            let Ok(PeerMessage::FileSearchResult(payload)) =
                decode_peer_message(frame.code, &frame.payload)
            else {
                continue;
            };
            for file in payload.files {
                let raw_path = sanitize_peer_virtual_path(&file.file_path);
                if raw_path.is_empty() || !matches_track_filter(&raw_path, strict_track) {
                    continue;
                }
                let score = score_track_candidate(&raw_path, query, strict_track);
                let resolved_username = if payload.username.is_empty() {
                    candidate.username.clone()
                } else {
                    payload.username.clone()
                };
                let key = (
                    resolved_username.clone(),
                    peer_addr.clone(),
                    raw_path.clone(),
                );
                if !seen_hits.insert(key) {
                    continue;
                }
                collection.hits.push(DistributedSearchHit {
                    username: resolved_username,
                    peer_addr: peer_addr.clone(),
                    file_path: raw_path,
                    file_size: file.file_size,
                    connect_token: candidate.token,
                    score,
                });
            }
        }

        if collection.hits.len() >= 40 {
            break;
        }
    }

    collection.hits.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| b.file_size.cmp(&a.file_size))
    });
    collection
}

fn matches_track_filter(file_path: &str, strict_track: Option<&str>) -> bool {
    let lower = file_path.to_ascii_lowercase();
    match strict_track {
        Some(track) => lower.contains(&track.to_ascii_lowercase()),
        None => true,
    }
}

fn sanitize_peer_virtual_path(file_path: &str) -> String {
    file_path.trim().trim_matches(char::from(0)).to_owned()
}

fn normalize_peer_virtual_path(file_path: &str) -> String {
    let mut normalized = sanitize_peer_virtual_path(file_path).replace('/', "\\");
    while normalized.contains("\\\\") {
        normalized = normalized.replace("\\\\", "\\");
    }
    if let Some(stripped) = normalized.strip_prefix("@@")
        && let Some(split) = stripped.find('\\')
    {
        normalized = stripped[split + 1..].to_owned();
    }
    normalized.trim_start_matches('\\').to_owned()
}

fn score_track_candidate(file_path: &str, query: &str, strict_track: Option<&str>) -> i64 {
    let lower = file_path.to_ascii_lowercase();
    let mut score = 0_i64;
    if let Some(track) = strict_track {
        if lower.contains(&track.to_ascii_lowercase()) {
            score += 1_000;
        } else {
            score -= 1_000;
        }
    }
    for keyword in ["aphex", "twin", "flim"] {
        if lower.contains(keyword) {
            score += 120;
        }
    }
    for token in query
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|part| part.len() >= 3)
    {
        if lower.contains(&token.to_ascii_lowercase()) {
            score += 30;
        }
    }
    if lower.ends_with(".flac") {
        score += 40;
    } else if lower.ends_with(".mp3") {
        score += 25;
    } else if lower.ends_with(".m4a") {
        score += 20;
    }
    score
}

async fn write_peer_init_frame(
    stream: &mut TcpStream,
    username: &str,
    connection_type: &str,
    token: u32,
) -> Result<()> {
    let mut payload = Vec::new();
    payload.push(1_u8);
    let mut writer = PayloadWriter::new();
    writer.write_string(username);
    writer.write_string(connection_type);
    writer.write_u32(token);
    payload.extend_from_slice(&writer.into_inner());

    let mut frame = Vec::with_capacity(4 + payload.len());
    frame.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    frame.extend_from_slice(&payload);
    stream
        .write_all(&frame)
        .await
        .context("write peer init frame")?;
    stream.flush().await.context("flush peer init frame")?;
    Ok(())
}

#[derive(Debug, Clone)]
struct PeerInitPayload {
    username: String,
    connection_type: String,
    token: u32,
}

fn parse_peer_init_payload(payload: &[u8]) -> Result<PeerInitPayload> {
    if payload.is_empty() {
        bail!("peer init payload is empty");
    }
    let message_type = payload[0];
    if message_type != 1 {
        bail!("unexpected peer init message type: {message_type}");
    }
    let mut reader = PayloadReader::new(&payload[1..]);
    let username = reader
        .read_string()
        .map_err(|err| anyhow!("decode peer init username: {err}"))?;
    let connection_type = reader
        .read_string()
        .map_err(|err| anyhow!("decode peer init connection type: {err}"))?;
    let token = reader
        .read_u32()
        .map_err(|err| anyhow!("decode peer init token: {err}"))?;
    Ok(PeerInitPayload {
        username,
        connection_type,
        token,
    })
}

async fn read_peer_init_payload(stream: &mut TcpStream) -> Result<PeerInitPayload> {
    let mut len_buf = [0_u8; 4];
    stream
        .read_exact(&mut len_buf)
        .await
        .context("read peer init frame len")?;
    let body_len = u32::from_le_bytes(len_buf) as usize;
    if body_len == 0 || body_len > 4 * 1024 {
        bail!("peer init body length out of bounds: {body_len}");
    }
    let mut body = vec![0_u8; body_len];
    stream
        .read_exact(&mut body)
        .await
        .context("read peer init frame body")?;
    parse_peer_init_payload(&body)
}

async fn accept_peer_connection_with_init(
    listener: &TcpListener,
    expected_connection_type: &str,
    expected_username: Option<&str>,
    timeout: Duration,
) -> Result<(TcpStream, PeerInitPayload, SocketAddr)> {
    let deadline = Instant::now() + timeout;
    let mut last_error = None::<String>;
    while Instant::now() < deadline {
        let remaining = deadline.saturating_duration_since(Instant::now());
        let accepted = tokio::time::timeout(remaining, listener.accept())
            .await
            .context("timed out waiting for inbound peer socket")?
            .context("accept inbound peer socket")?;
        let (mut stream, addr) = accepted;
        let init =
            match tokio::time::timeout(Duration::from_secs(6), read_peer_init_payload(&mut stream))
                .await
            {
                Ok(Ok(payload)) => payload,
                Ok(Err(err)) => {
                    last_error = Some(format!("decode peer init failed: {err}"));
                    continue;
                }
                Err(_) => {
                    last_error = Some("timed out waiting for peer init payload".to_string());
                    continue;
                }
            };
        if !init
            .connection_type
            .eq_ignore_ascii_case(expected_connection_type)
        {
            last_error = Some(format!(
                "unexpected peer init connection type {} from {}",
                init.connection_type, addr
            ));
            continue;
        }
        if let Some(username) = expected_username
            && !init.username.eq_ignore_ascii_case(username)
        {
            last_error = Some(format!(
                "unexpected peer init username {} (expected {})",
                init.username, username
            ));
            continue;
        }
        return Ok((stream, init, addr));
    }
    bail!(
        "timed out waiting for inbound {} connection: {}",
        expected_connection_type,
        last_error.unwrap_or_else(|| "no matching connection observed".to_string())
    )
}

async fn maybe_read_peer_init_payload(stream: &mut TcpStream) -> Option<PeerInitPayload> {
    let mut probe = [0_u8; 5];
    let Ok(Ok(peeked)) = tokio::time::timeout(Duration::from_secs(2), stream.peek(&mut probe)).await
    else {
        return None;
    };
    if peeked < 5 {
        return None;
    }
    let body_len = u32::from_le_bytes([probe[0], probe[1], probe[2], probe[3]]) as usize;
    if !(13..=4 * 1024).contains(&body_len) || probe[4] != 1 {
        return None;
    }

    match tokio::time::timeout(Duration::from_secs(6), read_peer_init_payload(stream)).await {
        Ok(Ok(init)) => Some(init),
        Ok(Err(err)) => {
            transfer_debug(format!("inbound file socket peer init decode failed: {err}"));
            None
        }
        Err(_) => {
            transfer_debug("inbound file socket peer init read timed out");
            None
        }
    }
}

async fn accept_peer_file_socket(
    listener: &TcpListener,
    timeout: Duration,
) -> Result<(TcpStream, Option<PeerInitPayload>, SocketAddr)> {
    let deadline = Instant::now() + timeout;
    let mut last_error = None::<String>;
    while Instant::now() < deadline {
        let remaining = deadline.saturating_duration_since(Instant::now());
        let accepted = tokio::time::timeout(remaining, listener.accept())
            .await
            .context("timed out waiting for inbound file socket")?
            .context("accept inbound file socket")?;
        let (mut stream, addr) = accepted;
        let maybe_init = maybe_read_peer_init_payload(&mut stream).await;
        if let Some(init) = &maybe_init
            && !init.connection_type.eq_ignore_ascii_case("F")
            && !init.connection_type.eq_ignore_ascii_case("P")
        {
            last_error = Some(format!(
                "unexpected inbound file socket connection type {} from {}",
                init.connection_type, addr
            ));
            continue;
        }
        return Ok((stream, maybe_init, addr));
    }
    bail!(
        "timed out waiting for inbound file socket: {}",
        last_error.unwrap_or_else(|| "no candidate socket accepted".to_string())
    )
}

fn resolve_wait_port(explicit: Option<u16>) -> Option<u16> {
    if explicit.is_some() {
        return explicit;
    }
    if let Ok(raw) = std::env::var("NSS_WAIT_PORT")
        && let Ok(value) = raw.parse::<u16>()
    {
        return Some(value);
    }
    Some(50036)
}

fn resolve_max_candidate_attempts() -> usize {
    if let Ok(raw) = std::env::var("NSS_MAX_CANDIDATE_ATTEMPTS")
        && let Ok(value) = raw.parse::<usize>()
    {
        return value.clamp(1, 128);
    }
    32
}

fn resolve_queue_wait_secs() -> u64 {
    if let Ok(raw) = std::env::var("NSS_QUEUE_WAIT_SECS")
        && let Ok(value) = raw.parse::<u64>()
    {
        return value.clamp(0, 900);
    }
    0
}

fn resolve_queue_upload_include_username() -> bool {
    matches!(
        std::env::var("NSS_QUEUE_UPLOAD_INCLUDE_USERNAME"),
        Ok(value)
            if value == "1"
                || value.eq_ignore_ascii_case("true")
                || value.eq_ignore_ascii_case("yes")
    )
}

fn resolve_transfer_flow_timeout() -> Duration {
    if let Ok(raw) = std::env::var("NSS_TRANSFER_FLOW_TIMEOUT_SECS")
        && let Ok(value) = raw.parse::<u64>()
    {
        return Duration::from_secs(value.clamp(20, 900));
    }
    let queue_wait = resolve_queue_wait_secs();
    let base = queue_wait.saturating_add(90).clamp(90, 900);
    Duration::from_secs(base)
}

fn resolve_transfer_download_include_size() -> bool {
    matches!(
        std::env::var("NSS_TRANSFER_REQ_INCLUDE_SIZE_DOWNLOAD"),
        Ok(value)
            if value == "1"
                || value.eq_ignore_ascii_case("true")
                || value.eq_ignore_ascii_case("yes")
    )
}

fn resolve_transfer_download_direction() -> TransferDirection {
    match std::env::var("NSS_TRANSFER_REQ_DIRECTION") {
        Ok(value)
            if value == "1"
                || value.eq_ignore_ascii_case("upload")
                || value.eq_ignore_ascii_case("up") =>
        {
            TransferDirection::Upload
        }
        _ => TransferDirection::Download,
    }
}

fn resolve_skip_transfer_request_flow() -> bool {
    matches!(
        std::env::var("NSS_SKIP_TRANSFER_REQUEST_FLOW"),
        Ok(value)
            if value == "1"
                || value.eq_ignore_ascii_case("true")
                || value.eq_ignore_ascii_case("yes")
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransferAllowMode {
    Legacy,
    Modern,
    Dual,
}

fn resolve_transfer_allow_mode() -> TransferAllowMode {
    match std::env::var("NSS_TRANSFER_ALLOW_MODE") {
        Ok(value) if value.eq_ignore_ascii_case("modern") => TransferAllowMode::Modern,
        Ok(value) if value.eq_ignore_ascii_case("dual") => TransferAllowMode::Dual,
        _ => TransferAllowMode::Legacy,
    }
}

fn resolve_transfer_init_read_timeout() -> Duration {
    if let Ok(raw) = std::env::var("NSS_TRANSFER_INIT_READ_TIMEOUT_SECS")
        && let Ok(value) = raw.parse::<u64>()
    {
        return Duration::from_secs(value.clamp(1, 30));
    }
    Duration::from_secs(3)
}

fn resolve_inbound_file_wait_timeout() -> Duration {
    if let Ok(raw) = std::env::var("NSS_INBOUND_FILE_WAIT_SECS")
        && let Ok(value) = raw.parse::<u64>()
    {
        return Duration::from_secs(value.clamp(0, 120));
    }
    Duration::from_secs(8)
}

fn resolve_transfer_body_chunk_timeout() -> Duration {
    if let Ok(raw) = std::env::var("NSS_TRANSFER_BODY_CHUNK_TIMEOUT_SECS")
        && let Ok(value) = raw.parse::<u64>()
    {
        return Duration::from_secs(value.clamp(2, 90));
    }
    Duration::from_secs(12)
}

fn transfer_debug_enabled() -> bool {
    matches!(
        std::env::var("NSS_DEBUG_TRANSFER"),
        Ok(value)
            if value == "1"
                || value.eq_ignore_ascii_case("true")
                || value.eq_ignore_ascii_case("yes")
    )
}

fn transfer_debug(message: impl AsRef<str>) {
    if transfer_debug_enabled() {
        eprintln!("[transfer-debug] {}", message.as_ref());
    }
}

fn format_error_chain(err: &anyhow::Error) -> String {
    err.chain()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>()
        .join(" | caused by: ")
}

fn build_download_transfer_request_runtime(token: u32, virtual_path: &str, file_size: u64) -> Frame {
    let direction = resolve_transfer_download_direction();
    if direction == TransferDirection::Upload {
        return build_transfer_request(direction, token, virtual_path, file_size);
    }
    if !resolve_transfer_download_include_size() {
        return build_transfer_request(TransferDirection::Download, token, virtual_path, file_size);
    }
    let mut writer = PayloadWriter::new();
    writer.write_u32(TransferDirection::Download.as_u32());
    writer.write_u32(token);
    writer.write_string(virtual_path);
    writer.write_u64(file_size);
    Frame::new(CODE_PM_TRANSFER_REQUEST, writer.into_inner())
}

fn build_transfer_response_runtime(token: u32, allowed: bool, queue_or_reason: &str) -> Frame {
    encode_peer_message(&PeerMessage::TransferResponse(TransferResponsePayload {
        token,
        allowed,
        queue_or_reason: queue_or_reason.to_owned(),
    }))
}

async fn write_transfer_allow(stream: &mut TcpStream, token: u32) -> Result<()> {
    match resolve_transfer_allow_mode() {
        TransferAllowMode::Legacy => {
            let legacy_allow = protocol::build_transfer_response(token, true, "");
            write_frame(stream, &legacy_allow).await?;
        }
        TransferAllowMode::Modern => {
            let modern_allow = build_transfer_response_runtime(token, true, "");
            write_frame(stream, &modern_allow).await?;
        }
        TransferAllowMode::Dual => {
            let legacy_allow = protocol::build_transfer_response(token, true, "");
            write_frame(stream, &legacy_allow).await?;
            let modern_allow = build_transfer_response_runtime(token, true, "");
            if let Err(err) = write_frame(stream, &modern_allow).await {
                transfer_debug(format!(
                    "failed to write modern transfer allow fallback for token={token}: {err}"
                ));
            }
        }
    }
    Ok(())
}

fn validate_transfer_content_size(actual_len: usize, expected_size: u64) -> Result<()> {
    if actual_len == 0 {
        bail!("peer returned zero bytes for transfer");
    }
    if expected_size > 0 && actual_len as u64 != expected_size {
        bail!(
            "peer returned partial bytes for transfer: got={} expected={expected_size}",
            actual_len
        );
    }
    Ok(())
}

fn hex_prefix(bytes: &[u8], max_len: usize) -> String {
    let mut out = String::new();
    for (idx, byte) in bytes.iter().take(max_len).enumerate() {
        if idx > 0 {
            out.push(' ');
        }
        out.push_str(&format!("{byte:02x}"));
    }
    if bytes.len() > max_len {
        out.push_str(" ...");
    }
    out
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginProbeAttempt {
    pub client_version: u32,
    pub minor_version: u32,
    pub result: String,
}

pub async fn probe_login_versions(
    server_addr: &str,
    username: &str,
    password: &str,
) -> Result<Vec<LoginProbeAttempt>> {
    const CANDIDATES: &[(u32, u32)] = &[(160, 1), (157, 19), (157, 17), (157, 100)];
    let mut attempts = Vec::with_capacity(CANDIDATES.len());

    for (client_version, minor_version) in CANDIDATES {
        let mut session = SessionClient::connect(server_addr).await?;
        let credentials = Credentials {
            username: username.to_owned(),
            password: password.to_owned(),
            client_version: *client_version,
            minor_version: *minor_version,
        };

        let result = match session.login(&credentials).await {
            Ok(()) => "success".to_string(),
            Err(err) => format!("{err}"),
        };

        attempts.push(LoginProbeAttempt {
            client_version: *client_version,
            minor_version: *minor_version,
            result: result.clone(),
        });

        if result == "success" || !result.contains("INVALIDVERSION") {
            break;
        }
    }

    Ok(attempts)
}

fn is_connection_eof(err: &anyhow::Error) -> bool {
    err.chain().any(|cause| {
        let rendered = cause.to_string().to_lowercase();
        rendered.contains("early eof")
            || rendered.contains("connection reset")
            || rendered.contains("failed to fill whole buffer")
    })
}

pub async fn write_frame(stream: &mut TcpStream, frame: &Frame) -> Result<()> {
    let bytes = frame.encode();
    stream.write_all(&bytes).await.context("write frame")?;
    stream.flush().await.context("flush frame")?;
    Ok(())
}

pub async fn read_frame(stream: &mut TcpStream) -> Result<Frame> {
    let mut len_buf = [0_u8; 4];
    stream
        .read_exact(&mut len_buf)
        .await
        .context("read frame len")?;
    let body_len = u32::from_le_bytes(len_buf) as usize;

    let mut body = vec![0_u8; body_len];
    stream
        .read_exact(&mut body)
        .await
        .context("read frame body")?;

    let mut frame_bytes = Vec::with_capacity(body_len + 4);
    frame_bytes.extend_from_slice(&len_buf);
    frame_bytes.extend_from_slice(&body);
    Frame::decode(&frame_bytes)
}

pub async fn download_single_file(plan: &DownloadPlan) -> Result<DownloadResult> {
    let mut stream = TcpStream::connect(&plan.peer_addr)
        .await
        .with_context(|| format!("connect peer failed: {}", plan.peer_addr))?;

    let request = build_download_transfer_request_runtime(plan.token, &plan.virtual_path, plan.file_size);
    write_frame(&mut stream, &request).await?;
    let response = read_transfer_response(&mut stream).await?;

    validate_transfer_response(plan.token, &response)?;
    ensure_parent_dir(&plan.output_path).await?;

    let content = read_transfer_body(&mut stream, plan.file_size).await?;
    validate_transfer_content_size(content.len(), plan.file_size)?;
    fs::write(&plan.output_path, &content)
        .await
        .with_context(|| format!("write output file: {}", plan.output_path.display()))?;

    Ok(DownloadResult {
        output_path: plan.output_path.clone(),
        bytes_written: content.len() as u64,
    })
}

pub async fn download_single_file_with_peer_init(
    plan: &DownloadPlan,
    login_username: &str,
    connection_type: &str,
    connect_token: u32,
) -> Result<DownloadResult> {
    let mut stream = TcpStream::connect(&plan.peer_addr)
        .await
        .with_context(|| format!("connect peer failed: {}", plan.peer_addr))?;

    write_peer_init_frame(&mut stream, login_username, connection_type, connect_token).await?;
    if connection_type.eq_ignore_ascii_case("P") {
        let connect_token_frame = build_send_connect_token(login_username, connect_token);
        write_frame(&mut stream, &connect_token_frame).await?;
    }
    let request = build_download_transfer_request_runtime(plan.token, &plan.virtual_path, plan.file_size);
    write_frame(&mut stream, &request).await?;
    let response = read_transfer_response(&mut stream).await?;

    validate_transfer_response(plan.token, &response)?;
    ensure_parent_dir(&plan.output_path).await?;

    let content = read_transfer_body(&mut stream, plan.file_size).await?;
    validate_transfer_content_size(content.len(), plan.file_size)?;
    fs::write(&plan.output_path, &content)
        .await
        .with_context(|| format!("write output file: {}", plan.output_path.display()))?;

    Ok(DownloadResult {
        output_path: plan.output_path.clone(),
        bytes_written: content.len() as u64,
    })
}

async fn download_single_file_via_transfer_request(
    plan: &DownloadPlan,
    login_username: &str,
    connect_token: u32,
    wait_port: Option<u16>,
    outbound_peer_addr: Option<&str>,
    outbound_connect_token: Option<u32>,
) -> Result<DownloadResult> {
    transfer_debug(format!(
        "transfer-request flow start peer={} path={} token={} connect_token={}",
        plan.peer_addr, plan.virtual_path, plan.token, connect_token
    ));
    let mut p_stream = TcpStream::connect(&plan.peer_addr)
        .await
        .with_context(|| format!("connect peer failed: {}", plan.peer_addr))?;
    write_peer_init_frame(&mut p_stream, login_username, "P", connect_token).await?;
    let connect_token_frame = build_send_connect_token(login_username, connect_token);
    write_frame(&mut p_stream, &connect_token_frame).await?;

    let mut inbound_listener = None::<TcpListener>;
    let mut inbound_bind_error = None::<String>;
    if let Some(port) = wait_port {
        let bind_addr = format!("0.0.0.0:{port}");
        match TcpListener::bind(&bind_addr).await {
            Ok(listener) => inbound_listener = Some(listener),
            Err(err) => {
                inbound_bind_error = Some(format!(
                    "bind inbound transfer listener failed on {bind_addr}: {err}"
                ));
            }
        }
    }

    let transfer_request =
        build_download_transfer_request_runtime(plan.token, &plan.virtual_path, plan.file_size);
    write_frame(&mut p_stream, &transfer_request).await?;

    let response = read_transfer_response(&mut p_stream).await?;
    let mut expected_size = plan.file_size;
    let mut file_transfer_token = plan.token;
    if response.allowed {
        validate_transfer_response(plan.token, &response)?;
    } else if response
        .queue_or_reason
        .to_ascii_lowercase()
        .contains("queued")
    {
        if response.token != plan.token {
            bail!(
                "token mismatch: expected {} got {}",
                plan.token,
                response.token
            );
        }
        let wait_secs = resolve_queue_wait_secs();
        if wait_secs == 0 {
            validate_transfer_response(plan.token, &response)?;
        } else {
            transfer_debug(format!(
                "transfer-request queued, waiting up to {}s for peer grant",
                wait_secs
            ));
            let queued_request =
                read_peer_transfer_request(&mut p_stream, Duration::from_secs(wait_secs)).await?;
            transfer_debug(format!(
                "transfer-request queue granted token={} size={}",
                queued_request.token, queued_request.file_size
            ));
            file_transfer_token = queued_request.token;
            write_transfer_allow(&mut p_stream, queued_request.token).await?;
            if queued_request.file_size != 0 {
                expected_size = queued_request.file_size;
            }
        }
    } else {
        validate_transfer_response(plan.token, &response)?;
    }
    ensure_parent_dir(&plan.output_path).await?;

    if let Some(content) =
        try_read_transfer_body_on_control_channel(&mut p_stream, expected_size).await?
    {
        if validate_transfer_content_size(content.len(), expected_size).is_ok() {
            fs::write(&plan.output_path, &content)
                .await
                .with_context(|| format!("write output file: {}", plan.output_path.display()))?;
            return Ok(DownloadResult {
                output_path: plan.output_path.clone(),
                bytes_written: content.len() as u64,
            });
        }
    }

    let mut inbound_f_error = inbound_bind_error;
    if let Some(listener) = inbound_listener.as_ref() {
        match accept_peer_file_socket(listener, resolve_inbound_file_wait_timeout()).await {
            Ok((mut f_stream, maybe_init, addr)) => {
                if let Some(init) = maybe_init {
                    transfer_debug(format!(
                        "transfer-request: accepted inbound F socket with init user={} token={} from={}",
                        init.username, init.token, addr
                    ));
                } else {
                    transfer_debug(format!(
                        "transfer-request: accepted inbound F socket without init from={addr}"
                    ));
                }
                let content =
                    read_file_transfer_content(&mut f_stream, expected_size, file_transfer_token)
                        .await?;
                if validate_transfer_content_size(content.len(), expected_size).is_ok() {
                    fs::write(&plan.output_path, &content)
                        .await
                        .with_context(|| format!("write output file: {}", plan.output_path.display()))?;
                    return Ok(DownloadResult {
                        output_path: plan.output_path.clone(),
                        bytes_written: content.len() as u64,
                    });
                }
                inbound_f_error = Some(format!(
                    "peer returned partial/empty bytes after inbound F flow: got={} expected={expected_size}",
                    content.len()
                ));
            }
            Err(err) => {
                let rendered = format_error_chain(&err);
                transfer_debug(format!("transfer-request inbound F failed: {rendered}"));
                inbound_f_error = Some(format!("inbound F flow failed: {rendered}"));
            }
        }
    }

    let outbound_addr = outbound_peer_addr.unwrap_or(&plan.peer_addr);
    let outbound_token = outbound_connect_token.unwrap_or(connect_token);
    let content = match read_file_transfer_content_outbound_with_variants(
        outbound_addr,
        login_username,
        outbound_token,
        expected_size,
        file_transfer_token,
        plan.token,
    )
    .await
    {
        Ok(content) => content,
        Err(err) => {
            let rendered = format_error_chain(&err);
            if let Some(inbound_error) = inbound_f_error {
                bail!("outbound F flow failed: {rendered}; inbound F flow: {inbound_error}");
            }
            return Err(err);
        }
    };
    validate_transfer_content_size(content.len(), expected_size)?;
    fs::write(&plan.output_path, &content)
        .await
        .with_context(|| format!("write output file: {}", plan.output_path.display()))?;

    Ok(DownloadResult {
        output_path: plan.output_path.clone(),
        bytes_written: content.len() as u64,
    })
}

async fn read_transfer_response(stream: &mut TcpStream) -> Result<TransferResponsePayload> {
    let mut budget = 10usize;
    while budget > 0 {
        budget = budget.saturating_sub(1);
        let frame = tokio::time::timeout(Duration::from_secs(8), read_frame(stream))
            .await
            .context("timed out waiting for transfer response frame")??;
        let Ok(message) = decode_peer_message(frame.code, &frame.payload) else {
            continue;
        };
        if let PeerMessage::TransferResponse(payload) = message {
            return Ok(payload);
        }
    }
    bail!("did not receive transfer response from peer");
}

async fn read_file_transfer_content_outbound_with_variants(
    peer_addr: &str,
    login_username: &str,
    connect_token: u32,
    expected_size: u64,
    file_transfer_token: u32,
    request_token: u32,
) -> Result<Vec<u8>> {
    async fn run_variant(
        peer_addr: &str,
        login_username: &str,
        connect_token: u32,
        expected_size: u64,
        file_transfer_token: u32,
        request_token: u32,
        init_connection_type: Option<&str>,
        variant_name: &str,
    ) -> Result<Vec<u8>> {
        async fn connect_variant_socket(
            peer_addr: &str,
            login_username: &str,
            connect_token: u32,
            init_connection_type: Option<&str>,
            variant_name: &str,
        ) -> Result<TcpStream> {
            let mut stream = TcpStream::connect(peer_addr)
                .await
                .with_context(|| format!("connect peer file socket failed: {peer_addr}"))?;
            if let Some(connection_type) = init_connection_type {
                write_peer_init_frame(&mut stream, login_username, connection_type, connect_token)
                    .await
                    .with_context(|| format!("write F init frame ({variant_name})"))?;
            }
            Ok(stream)
        }

        let mut token_candidates = Vec::with_capacity(2);
        token_candidates.push(file_transfer_token);
        if connect_token != file_transfer_token {
            token_candidates.push(connect_token);
        }
        if request_token != file_transfer_token && request_token != connect_token {
            token_candidates.push(request_token);
        }

        let mut last_error = None::<String>;
        for candidate_token in token_candidates {
            let mut token_offset_stream = connect_variant_socket(
                peer_addr,
                login_username,
                connect_token,
                init_connection_type,
                variant_name,
            )
            .await?;
            if let Ok(content) = read_file_transfer_content_with_token_init(
                &mut token_offset_stream,
                expected_size,
                candidate_token,
            )
            .await
            {
                return Ok(content);
            }

            let mut token_only_stream = connect_variant_socket(
                peer_addr,
                login_username,
                connect_token,
                init_connection_type,
                variant_name,
            )
            .await?;
            match read_file_transfer_content_with_token_only_init(
                &mut token_only_stream,
                expected_size,
                candidate_token,
            )
            .await
            {
                Ok(content) => return Ok(content),
                Err(err) => {
                    last_error = Some(format!(
                        "token={candidate_token} failed: {}",
                        format_error_chain(&err)
                    ));
                }
            }
        }

        let mut offset_only_stream = connect_variant_socket(
            peer_addr,
            login_username,
            connect_token,
            init_connection_type,
            variant_name,
        )
        .await?;
        if let Ok(content) =
            read_file_transfer_content_with_offset_only_init(&mut offset_only_stream, expected_size).await
        {
            return Ok(content);
        }

        let mut wait_remote_token_stream = connect_variant_socket(
            peer_addr,
            login_username,
            connect_token,
            init_connection_type,
            variant_name,
        )
        .await?;
        if let Ok(content) =
            read_file_transfer_content(&mut wait_remote_token_stream, expected_size, file_transfer_token).await
        {
            return Ok(content);
        }

        bail!(
            "token-init file transfer variants failed ({variant_name}): {}",
            last_error.unwrap_or_else(|| "unknown token-init failure".to_string())
        )
    }

    match run_variant(
        peer_addr,
        login_username,
        connect_token,
        expected_size,
        file_transfer_token,
        request_token,
        None,
        "no-init",
    )
    .await
    {
        Ok(content) => Ok(content),
        Err(no_init_err) => {
            transfer_debug(format!(
                "outbound F no-init variants failed: {}; retrying with F init",
                format_error_chain(&no_init_err)
            ));
            let f_init_result = run_variant(
                peer_addr,
                login_username,
                connect_token,
                expected_size,
                file_transfer_token,
                request_token,
                Some("F"),
                "with-init-f",
            )
            .await;
            match f_init_result {
                Ok(content) => Ok(content),
                Err(f_init_err) => {
                    transfer_debug(format!(
                        "outbound F with-init variants failed: {}; retrying with P init",
                        format_error_chain(&f_init_err)
                    ));
                    run_variant(
                        peer_addr,
                        login_username,
                        connect_token,
                        expected_size,
                        file_transfer_token,
                        request_token,
                        Some("P"),
                        "with-init-p",
                    )
                    .await
                    .with_context(|| {
                        format!(
                            "with-init-p outbound fallback failed after no-init={} and with-init-f={}",
                            format_error_chain(&no_init_err),
                            format_error_chain(&f_init_err)
                        )
                    })
                }
            }
        }
    }
}

async fn download_single_file_via_queue_upload(
    plan: &DownloadPlan,
    login_username: &str,
    _peer_username: &str,
    connect_token: u32,
    wait_port: Option<u16>,
    outbound_peer_addr: Option<&str>,
    outbound_connect_token: Option<u32>,
) -> Result<DownloadResult> {
    let mut inbound_listener = None::<TcpListener>;
    let mut inbound_bind_error = None::<String>;
    if let Some(port) = wait_port {
        let bind_addr = format!("0.0.0.0:{port}");
        match TcpListener::bind(&bind_addr).await {
            Ok(listener) => {
                inbound_listener = Some(listener);
            }
            Err(err) => {
                inbound_bind_error = Some(format!(
                    "bind inbound F listener failed on {bind_addr}: {err}"
                ));
            }
        }
    }

    let mut p_stream = TcpStream::connect(&plan.peer_addr)
        .await
        .with_context(|| format!("connect peer failed: {}", plan.peer_addr))?;
    transfer_debug(format!(
        "queue-upload flow start peer={} path={} token={}",
        plan.peer_addr, plan.virtual_path, connect_token
    ));
    write_peer_init_frame(&mut p_stream, login_username, "P", connect_token).await?;
    let connect_token_frame = build_send_connect_token(login_username, connect_token);
    write_frame(&mut p_stream, &connect_token_frame).await?;

    let transfer_request = send_queue_upload_and_wait_transfer_request(
        &mut p_stream,
        login_username,
        &plan.virtual_path,
        resolve_transfer_flow_timeout(),
    )
    .await?;
    transfer_debug(format!(
        "received transfer request token={} direction={:?} size={}",
        transfer_request.token, transfer_request.direction, transfer_request.file_size
    ));
    write_transfer_allow(&mut p_stream, transfer_request.token).await?;

    ensure_parent_dir(&plan.output_path).await?;
    let expected_size = if transfer_request.file_size == 0 {
        plan.file_size
    } else {
        transfer_request.file_size
    };

    if let Some(content) =
        try_read_transfer_body_on_control_channel(&mut p_stream, expected_size).await?
    {
        if validate_transfer_content_size(content.len(), expected_size).is_ok() {
            fs::write(&plan.output_path, &content)
                .await
                .with_context(|| format!("write output file: {}", plan.output_path.display()))?;
            return Ok(DownloadResult {
                output_path: plan.output_path.clone(),
                bytes_written: content.len() as u64,
            });
        }
    }

    let mut inbound_f_error = inbound_bind_error;
    if let Some(listener) = inbound_listener.as_ref() {
        match accept_peer_file_socket(listener, resolve_inbound_file_wait_timeout()).await {
            Ok((mut f_stream, maybe_init, addr)) => {
                if let Some(init) = maybe_init {
                    transfer_debug(format!(
                        "queue-upload: accepted inbound F socket with init user={} token={} from={}",
                        init.username, init.token, addr
                    ));
                } else {
                    transfer_debug(format!(
                        "queue-upload: accepted inbound F socket without init from={addr}"
                    ));
                }
                let content =
                    read_file_transfer_content(&mut f_stream, expected_size, transfer_request.token)
                        .await?;
                if validate_transfer_content_size(content.len(), expected_size).is_ok() {
                    fs::write(&plan.output_path, &content)
                        .await
                        .with_context(|| format!("write output file: {}", plan.output_path.display()))?;
                    return Ok(DownloadResult {
                        output_path: plan.output_path.clone(),
                        bytes_written: content.len() as u64,
                    });
                }
                inbound_f_error = Some(format!(
                    "peer returned partial/empty bytes after inbound F flow: got={} expected={expected_size}",
                    content.len()
                ));
            }
            Err(err) => {
                let rendered = format_error_chain(&err);
                transfer_debug(format!("queue-upload inbound F failed: {rendered}"));
                inbound_f_error = Some(format!("inbound F flow failed: {rendered}"));
            }
        }
    }

    let outbound_addr = outbound_peer_addr.unwrap_or(&plan.peer_addr);
    let outbound_token = outbound_connect_token.unwrap_or(connect_token);
    let content = match read_file_transfer_content_outbound_with_variants(
        outbound_addr,
        login_username,
        outbound_token,
        expected_size,
        transfer_request.token,
        plan.token,
    )
    .await
    {
        Ok(content) => content,
        Err(err) => {
            let rendered = format_error_chain(&err);
            bail!(
                "queue-upload transfer flow failed; outbound={rendered} inbound={}",
                inbound_f_error.unwrap_or_else(|| "no inbound F attempt".to_string())
            );
        }
    };
    validate_transfer_content_size(content.len(), expected_size)?;
    fs::write(&plan.output_path, &content)
        .await
        .with_context(|| format!("write output file: {}", plan.output_path.display()))?;
    Ok(DownloadResult {
        output_path: plan.output_path.clone(),
        bytes_written: content.len() as u64,
    })
}

async fn read_file_transfer_content(
    stream: &mut TcpStream,
    expected_size: u64,
    token_hint: u32,
) -> Result<Vec<u8>> {
    let mut transfer_init_token = [0_u8; 4];
    match tokio::time::timeout(
        resolve_transfer_init_read_timeout(),
        stream.read_exact(&mut transfer_init_token),
    )
    .await
    {
        Ok(Ok(_)) => {
            let _remote_token = u32::from_le_bytes(transfer_init_token);
            transfer_debug("file-transfer init: received remote token, sending offset");
            stream
                .write_all(&0_u64.to_le_bytes())
                .await
                .context("write file-transfer offset")?;
            stream.flush().await.context("flush file-transfer init")?;
        }
        Ok(Err(_read_err)) => {
            transfer_debug(
                "file-transfer init: no token frame, sending token+offset fallback if token hint exists",
            );
            if token_hint != 0 {
                stream
                    .write_all(&token_hint.to_le_bytes())
                    .await
                    .context("write file-transfer token fallback")?;
            }
            stream
                .write_all(&0_u64.to_le_bytes())
                .await
                .context("write file-transfer offset fallback")?;
            stream
                .flush()
                .await
                .context("flush file-transfer fallback init")?;
        }
        Err(_) => {
            transfer_debug(
                "file-transfer init: timeout, sending token+offset fallback if token hint exists",
            );
            if token_hint != 0 {
                stream
                    .write_all(&token_hint.to_le_bytes())
                    .await
                    .context("write file-transfer token after timeout")?;
            }
            stream
                .write_all(&0_u64.to_le_bytes())
                .await
                .context("write file-transfer offset after timeout")?;
            stream
                .flush()
                .await
                .context("flush file-transfer timeout fallback init")?;
        }
    }
    read_transfer_body(stream, expected_size).await
}

async fn read_file_transfer_content_with_token_init(
    stream: &mut TcpStream,
    expected_size: u64,
    token: u32,
) -> Result<Vec<u8>> {
    stream
        .write_all(&token.to_le_bytes())
        .await
        .context("write file-transfer token init")?;
    stream
        .write_all(&0_u64.to_le_bytes())
        .await
        .context("write file-transfer offset init")?;
    stream
        .flush()
        .await
        .context("flush file-transfer token+offset init")?;
    read_transfer_body(stream, expected_size).await
}

async fn read_file_transfer_content_with_token_only_init(
    stream: &mut TcpStream,
    expected_size: u64,
    token: u32,
) -> Result<Vec<u8>> {
    stream
        .write_all(&token.to_le_bytes())
        .await
        .context("write file-transfer token-only init")?;
    stream
        .flush()
        .await
        .context("flush file-transfer token-only init")?;
    read_transfer_body(stream, expected_size).await
}

async fn read_file_transfer_content_with_offset_only_init(
    stream: &mut TcpStream,
    expected_size: u64,
) -> Result<Vec<u8>> {
    stream
        .write_all(&0_u64.to_le_bytes())
        .await
        .context("write file-transfer offset-only init")?;
    stream
        .flush()
        .await
        .context("flush file-transfer offset-only init")?;
    read_transfer_body(stream, expected_size).await
}

async fn try_read_transfer_body_on_control_channel(
    stream: &mut TcpStream,
    expected_size: u64,
) -> Result<Option<Vec<u8>>> {
    let mut probe = [0_u8; 16 * 1024];
    let peeked = match tokio::time::timeout(resolve_transfer_init_read_timeout(), stream.peek(&mut probe))
        .await
    {
        Ok(Ok(n)) => n,
        Ok(Err(_)) => return Ok(None),
        Err(_) => return Ok(None),
    };
    if peeked == 0 {
        return Ok(Some(Vec::new()));
    }

    if peeked >= 8 {
        let body_len = u32::from_le_bytes([probe[0], probe[1], probe[2], probe[3]]);
        let code = u32::from_le_bytes([probe[4], probe[5], probe[6], probe[7]]);
        if (4..=65_536).contains(&body_len) && code <= 1_024 {
            let frame = tokio::time::timeout(resolve_transfer_body_chunk_timeout(), read_frame(stream))
                .await
                .context("timed out reading framed control payload during transfer")?
                .context("read framed control payload during transfer")?;
            transfer_debug(format!(
                "control channel frame detected while waiting for bytes (code={} len={})",
                frame.code,
                frame.payload.len()
            ));
            let decoded = decode_peer_message(frame.code, &frame.payload);
            match (frame.code, decoded) {
                (CODE_PM_UPLOAD_PLACE_IN_LINE, Ok(PeerMessage::UploadPlaceInLine(payload))) => {
                    bail!(
                        "peer queued transfer (place={} user={} path={})",
                        payload.place,
                        payload.username,
                        payload.virtual_path
                    );
                }
                (CODE_PM_UPLOAD_DENIED, Ok(PeerMessage::UploadDenied(payload)))
                | (CODE_PM_UPLOAD_FAILED, Ok(PeerMessage::UploadFailed(payload))) => {
                    let reason = if payload.reason.is_empty() {
                        "upload denied by peer".to_string()
                    } else {
                        payload.reason
                    };
                    bail!(
                        "peer denied transfer after allow (user={} path={}): {}",
                        payload.username,
                        payload.virtual_path,
                        reason
                    );
                }
                _ => {
                    transfer_debug("control frame is not file bytes; using dedicated F transfer path");
                    return Ok(None);
                }
            }
        }
    }

    let expected = expected_size.min(128 * 1024 * 1024) as usize;
    let mut first = [0_u8; 16 * 1024];
    let first_read = tokio::time::timeout(resolve_transfer_body_chunk_timeout(), stream.read(&mut first))
        .await
        .context("timed out reading transfer body first chunk")?
        .context("read transfer body first chunk")?;
    if first_read == 0 {
        return Ok(Some(Vec::new()));
    }
    let mut content = Vec::with_capacity(expected.max(first_read));
    content.extend_from_slice(&first[..first_read]);
    if expected_size == 0 {
        loop {
            let n = match tokio::time::timeout(resolve_transfer_body_chunk_timeout(), stream.read(&mut first))
                .await
            {
                Ok(Ok(n)) => n,
                Ok(Err(_)) => break,
                Err(_) => break,
            };
            if n == 0 {
                break;
            }
            content.extend_from_slice(&first[..n]);
        }
        return Ok(Some(content));
    }
    while content.len() < expected {
        let remaining = expected - content.len();
        let read_len = remaining.min(first.len());
        let n = match tokio::time::timeout(
            resolve_transfer_body_chunk_timeout(),
            stream.read(&mut first[..read_len]),
        )
                .await
        {
            Ok(Ok(n)) => n,
            Ok(Err(_)) => break,
            Err(_) => break,
        };
        if n == 0 {
            break;
        }
        content.extend_from_slice(&first[..n]);
    }
    Ok(Some(content))
}

fn build_queue_upload_frame_path_only(virtual_path: &str) -> Frame {
    let mut payload = PayloadWriter::new();
    payload.write_string(virtual_path);
    Frame::new(CODE_PM_QUEUE_UPLOAD, payload.into_inner())
}

fn build_queue_upload_frame_with_username(username: &str, virtual_path: &str) -> Frame {
    encode_peer_message(&PeerMessage::QueueUpload(QueueUploadPayload {
        username: username.to_owned(),
        virtual_path: virtual_path.to_owned(),
    }))
}

fn queue_upload_target(virtual_path: &str) -> String {
    normalize_peer_virtual_path(virtual_path)
}

fn push_unique_queue_target(out: &mut Vec<String>, candidate: String) {
    let trimmed = candidate.trim().trim_matches(char::from(0)).to_owned();
    if trimmed.is_empty() {
        return;
    }
    if !out.iter().any(|entry| entry == &trimmed) {
        out.push(trimmed);
    }
}

fn queue_upload_basename(path: &str) -> Option<String> {
    path.rsplit(['\\', '/'])
        .find(|segment| !segment.is_empty())
        .map(str::to_owned)
}

fn queue_upload_suffix_variants(path: &str) -> Vec<String> {
    let normalized = path.replace('/', "\\");
    let segments: Vec<&str> = normalized.split('\\').filter(|segment| !segment.is_empty()).collect();
    if segments.len() < 2 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(segments.len().saturating_sub(1));
    for index in 1..segments.len() {
        out.push(segments[index..].join("\\"));
    }
    out
}

fn queue_upload_targets(virtual_path: &str) -> Vec<String> {
    let raw = sanitize_peer_virtual_path(virtual_path);
    let normalized = queue_upload_target(&raw);
    let mut out = Vec::with_capacity(8);

    push_unique_queue_target(&mut out, raw.clone());
    push_unique_queue_target(&mut out, normalized.clone());
    push_unique_queue_target(&mut out, raw.replace('/', "\\"));
    push_unique_queue_target(&mut out, raw.replace('\\', "/"));
    if !normalized.is_empty() {
        push_unique_queue_target(&mut out, format!("\\{normalized}"));
    }
    if let Some((_, suffix)) = raw.strip_prefix("@@").and_then(|value| value.split_once('\\')) {
        push_unique_queue_target(&mut out, suffix.to_owned());
    }

    if let Some(base) = queue_upload_basename(&raw) {
        push_unique_queue_target(&mut out, base);
    }
    if let Some(base) = queue_upload_basename(&normalized) {
        push_unique_queue_target(&mut out, base);
    }
    for suffix in queue_upload_suffix_variants(&raw) {
        push_unique_queue_target(&mut out, suffix);
    }
    for suffix in queue_upload_suffix_variants(&normalized) {
        push_unique_queue_target(&mut out, suffix);
    }
    if out.is_empty() {
        push_unique_queue_target(&mut out, normalized);
    }
    out
}

async fn send_queue_upload_and_wait_transfer_request(
    stream: &mut TcpStream,
    login_username: &str,
    virtual_path: &str,
    timeout: Duration,
) -> Result<TransferRequestPayload> {
    let mut last_rejection = None::<String>;
    let include_username = resolve_queue_upload_include_username();
    for target in queue_upload_targets(virtual_path) {
        let mut queue_frames = vec![("path-only", build_queue_upload_frame_path_only(&target))];
        if include_username {
            queue_frames.push((
                "user+path",
                build_queue_upload_frame_with_username(login_username, &target),
            ));
        }
        for (variant, queue_frame) in queue_frames {
            transfer_debug(format!(
                "queue-upload request target={target} variant={variant}"
            ));
            write_frame(stream, &queue_frame).await?;
            match read_peer_transfer_request(stream, timeout).await {
                Ok(payload) => return Ok(payload),
                Err(err) if is_file_not_shared_error(&err) => {
                    transfer_debug(format!(
                        "queue-upload target rejected: target={target} variant={variant} err={err}"
                    ));
                    last_rejection = Some(format!("target={target} variant={variant}: {err}"));
                }
                Err(err) => return Err(err),
            }
        }
    }

    if let Some(rejection) = last_rejection {
        bail!("queue-upload rejected for all path variants: {rejection}");
    }
    bail!("queue-upload failed without transfer request");
}

async fn read_peer_transfer_request(
    stream: &mut TcpStream,
    timeout: Duration,
) -> Result<TransferRequestPayload> {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        let remaining = deadline.saturating_duration_since(Instant::now());
        let frame = tokio::time::timeout(remaining, read_frame(stream))
            .await
            .context("timed out waiting for transfer request frame")??;
        transfer_debug(format!(
            "peer frame while waiting transfer request: code={}",
            frame.code
        ));
        let decoded = decode_peer_message(frame.code, &frame.payload);
        match (frame.code, decoded) {
            (CODE_PM_TRANSFER_REQUEST, Ok(PeerMessage::TransferRequest(payload))) => {
                return Ok(payload);
            }
            (CODE_PM_UPLOAD_PLACE_IN_LINE, Ok(PeerMessage::UploadPlaceInLine(payload))) => {
                bail!(
                    "queued by peer (place={} user={} path={})",
                    payload.place,
                    payload.username,
                    payload.virtual_path
                );
            }
            (CODE_PM_UPLOAD_DENIED, Ok(PeerMessage::UploadDenied(payload)))
            | (CODE_PM_UPLOAD_FAILED, Ok(PeerMessage::UploadFailed(payload))) => {
                transfer_debug(format!(
                    "upload status raw payload (code={}): {}",
                    frame.code,
                    hex_prefix(&frame.payload, 96)
                ));
                let reason = if payload.reason.is_empty() {
                    "upload denied by peer".to_string()
                } else {
                    payload.reason
                };
                let reason_lower = reason.to_ascii_lowercase();
                let reason_looks_like_path =
                    reason.starts_with("@@") || reason.contains('\\') || reason.contains('/');
                if reason_lower.contains("file not shared")
                    || reason_lower.contains("not shared")
                    || reason_looks_like_path
                {
                    bail!(
                        "file not shared (user={} path={}): {}",
                        payload.username,
                        payload.virtual_path,
                        reason
                    );
                }
                bail!(
                    "peer denied upload (user={} path={}): {}",
                    payload.username,
                    payload.virtual_path,
                    reason
                );
            }
            (CODE_PM_UPLOAD_DENIED, Err(err)) | (CODE_PM_UPLOAD_FAILED, Err(err)) => {
                transfer_debug(format!(
                    "failed to decode upload status code={} err={} raw={}",
                    frame.code,
                    err,
                    hex_prefix(&frame.payload, 96)
                ));
            }
            _ => {}
        }
    }
    bail!("timed out waiting for transfer request after queue-upload");
}

fn is_file_not_shared_error(err: &anyhow::Error) -> bool {
    err.to_string()
        .to_ascii_lowercase()
        .contains("file not shared")
}

async fn read_transfer_body(stream: &mut TcpStream, expected_size: u64) -> Result<Vec<u8>> {
    if expected_size == 0 {
        let mut content = Vec::new();
        stream
            .read_to_end(&mut content)
            .await
            .context("read file body")?;
        return Ok(content);
    }

    let expected = expected_size.min(128 * 1024 * 1024) as usize;
    let mut content = Vec::with_capacity(expected);
    let mut buffer = [0_u8; 16 * 1024];
    while content.len() < expected {
        let remaining = expected - content.len();
        let read_len = remaining.min(buffer.len());
        let n = tokio::time::timeout(
            resolve_transfer_body_chunk_timeout(),
            stream.read(&mut buffer[..read_len]),
        )
        .await
        .context("timed out reading file body chunk")?
        .context("read file body chunk")?;
        if n == 0 {
            break;
        }
        content.extend_from_slice(&buffer[..n]);
    }
    Ok(content)
}

fn validate_transfer_response(
    expected_token: u32,
    response: &TransferResponsePayload,
) -> Result<()> {
    if response.token != expected_token {
        bail!(
            "token mismatch: expected {} got {}",
            expected_token,
            response.token
        );
    }

    if !response.allowed {
        let reason = if response.queue_or_reason.is_empty() {
            "peer denied transfer".to_string()
        } else {
            response.queue_or_reason.clone()
        };
        bail!(reason);
    }

    Ok(())
}

async fn ensure_parent_dir(path: &Path) -> Result<()> {
    match path.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => {
            fs::create_dir_all(parent)
                .await
                .with_context(|| format!("create parent dir: {}", parent.display()))?;
            Ok(())
        }
        _ => Ok(()),
    }
}

pub fn decode_frames_from_bytes(bytes: &[u8]) -> Result<Vec<Frame>> {
    let mut frames = Vec::new();
    let mut offset = 0;

    while offset < bytes.len() {
        let Some((frame, consumed)) = split_first_frame(&bytes[offset..])? else {
            return Err(anyhow!("incomplete trailing frame at offset {offset}"));
        };
        frames.push(frame);
        offset += consumed;
    }

    Ok(frames)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UploadPolicy {
    Manual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UploadDecisionKind {
    Accept,
    Deny,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManualUploadDecision {
    pub decision: UploadDecisionKind,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct UploadSessionResult {
    pub bind_addr: SocketAddr,
    pub peer_addr: SocketAddr,
    pub request: TransferRequestPayload,
    pub decision: UploadDecisionKind,
    pub bytes_sent: u64,
}

#[derive(Debug)]
pub struct UploadAgent {
    listener: TcpListener,
    policy: UploadPolicy,
}

impl UploadAgent {
    pub async fn bind_manual(bind_addr: &str) -> Result<Self> {
        let listener = TcpListener::bind(bind_addr)
            .await
            .with_context(|| format!("bind upload agent failed: {bind_addr}"))?;
        Ok(Self {
            listener,
            policy: UploadPolicy::Manual,
        })
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        Ok(self.listener.local_addr()?)
    }

    pub async fn serve_single_manual(
        self,
        decision: ManualUploadDecision,
        source_file: Option<PathBuf>,
    ) -> Result<UploadSessionResult> {
        if self.policy != UploadPolicy::Manual {
            bail!("only manual policy is supported in this stage");
        }

        let bind_addr = self.listener.local_addr()?;
        let (mut socket, peer_addr) = self.listener.accept().await.context("accept upload peer")?;

        let first_frame = read_frame(&mut socket).await?;
        let request = match decode_peer_message(first_frame.code, &first_frame.payload)? {
            PeerMessage::TransferRequest(payload) => payload,
            other => bail!("expected transfer request, got {other:?}"),
        };

        let allowed = decision.decision == UploadDecisionKind::Accept;
        let reason = if allowed {
            String::new()
        } else if decision.reason.is_empty() {
            "manual deny".to_string()
        } else {
            decision.reason
        };

        let response_frame =
            encode_peer_message(&PeerMessage::TransferResponse(TransferResponsePayload {
                token: request.token,
                allowed,
                queue_or_reason: reason,
            }));
        write_frame(&mut socket, &response_frame).await?;

        let mut bytes_sent = 0_u64;
        if allowed {
            if let Some(path) = source_file {
                let bytes = fs::read(&path)
                    .await
                    .with_context(|| format!("read upload source file: {}", path.display()))?;
                socket
                    .write_all(&bytes)
                    .await
                    .context("write upload bytes")?;
                socket.flush().await.context("flush upload bytes")?;
                bytes_sent = bytes.len() as u64;
            }
        }
        socket.shutdown().await.context("shutdown upload socket")?;

        Ok(UploadSessionResult {
            bind_addr,
            peer_addr,
            request,
            decision: decision.decision,
            bytes_sent,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{
        CODE_SM_ADD_CHATROOM, CODE_SM_ADD_LIKE_TERM, CODE_SM_ADD_ROOM_MEMBER,
        CODE_SM_ADD_ROOM_OPERATOR, CODE_SM_BAN_USER, CODE_SM_CONNECT_TO_PEER, CODE_SM_FILE_SEARCH,
        CODE_SM_GET_GLOBAL_RECOMMENDATIONS, CODE_SM_GET_MY_RECOMMENDATIONS,
        CODE_SM_GET_OWN_PRIVILEGES_STATUS, CODE_SM_GET_PEER_ADDRESS,
        CODE_SM_GET_RECOMMENDATION_USERS, CODE_SM_GET_RECOMMENDATIONS,
        CODE_SM_GET_RECOMMENDED_USERS, CODE_SM_GET_ROOM_TICKER, CODE_SM_GET_SIMILAR_TERMS,
        CODE_SM_GET_TERM_RECOMMENDATIONS, CODE_SM_GET_USER_PRIVILEGES_STATUS,
        CODE_SM_GET_USER_RECOMMENDATIONS, CODE_SM_GET_USER_STATS, CODE_SM_GET_USER_STATUS,
        CODE_SM_GIVE_PRIVILEGE, CODE_SM_IGNORE_USER, CODE_SM_INFORM_USER_OF_PRIVILEGES,
        CODE_SM_INFORM_USER_OF_PRIVILEGES_ACK, CODE_SM_JOIN_ROOM, CODE_SM_LEAVE_ROOM,
        CODE_SM_LOGIN, CODE_SM_MESSAGE_USER, CODE_SM_MESSAGE_USERS, CODE_SM_PRIVILEGED_LIST,
        CODE_SM_REMOVE_LIKE_TERM, CODE_SM_REMOVE_ROOM_MEMBER, CODE_SM_REMOVE_ROOM_OPERATOR,
        CODE_SM_ROOM_LIST, CODE_SM_UNIGNORE_USER, CODE_SM_UPLOAD_SPEED, JoinRoomPayload,
        LoginResponsePayload, LoginResponseSuccessPayload, MessageUserIncomingPayload,
        OwnPrivilegesStatusPayload, PeerAddressResponsePayload, PrivilegedListPayload,
        RecommendationEntry, RecommendationUsersPayload, RecommendationsPayload,
        RecommendedUsersPayload, RoomListPayload, RoomPresenceEventPayload, RoomTickerEntry,
        RoomTickerPayload, SearchFileSummary, SearchResponseSummary, ServerMessage,
        SimilarTermsPayload, SimilarTermsRequestPayload, SpeedPayload, TermRecommendationsPayload,
        UserPrivilegesStatusPayload, UserRecommendationsPayload, UserStatsResponsePayload,
        UserStatusResponsePayload, encode_server_message, parse_transfer_request,
        parse_transfer_response,
    };

    fn login_success_frame() -> Frame {
        encode_server_message(&ServerMessage::LoginResponse(
            LoginResponsePayload::Success(LoginResponseSuccessPayload {
                greeting: String::new(),
                ip_address: "127.0.0.1".into(),
                md5hash: "0123456789abcdef0123456789abcdef".into(),
                is_supporter: false,
            }),
        ))
    }

    fn login_failure_frame(reason: &str) -> Frame {
        let mut payload = protocol::PayloadWriter::new();
        payload.write_u8(0);
        payload.write_string(reason);
        Frame::new(CODE_SM_LOGIN, payload.into_inner())
    }

    #[test]
    fn collect_search_summaries_returns_only_summary_rows() {
        let messages = vec![
            ServerMessage::DownloadSpeed(SpeedPayload { bytes_per_sec: 9 }),
            ServerMessage::FileSearchResponseSummary(SearchResponseSummary {
                username: "alice".into(),
                token: 9001,
                files_count: 1,
                slots_free: 2,
                speed: 10,
                in_queue: false,
                files: vec![SearchFileSummary {
                    file_path: "Music\\\\Runtime\\\\track.flac".into(),
                    file_size: 123,
                    extension: "flac".into(),
                    attr_count: 0,
                }],
            }),
        ];

        let summaries = super::collect_search_summaries(&messages);
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].username, "alice");
        assert_eq!(summaries[0].files.len(), 1);
    }

    #[tokio::test]
    async fn search_select_and_download_reports_invalid_result_index() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");
            let request = read_frame(&mut socket).await.expect("search request");
            assert_eq!(request.code, CODE_SM_FILE_SEARCH);
            let summary = encode_server_message(&ServerMessage::FileSearchResponseSummary(
                SearchResponseSummary {
                    username: "alice".into(),
                    token: 9001,
                    files_count: 1,
                    slots_free: 1,
                    speed: 10,
                    in_queue: false,
                    files: vec![SearchFileSummary {
                        file_path: "Music\\\\Runtime\\\\track.flac".into(),
                        file_size: 123,
                        extension: "flac".into(),
                        attr_count: 0,
                    }],
                },
            ));
            write_frame(&mut socket, &summary)
                .await
                .expect("write summary");
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");

        let request = SearchSelectDownloadRequest {
            search_token: 9001,
            query: "runtime query".into(),
            search_timeout: Duration::from_secs(1),
            max_messages: 16,
            result_index: 1,
            file_index: 0,
            transfer_token: 555,
            output_path: PathBuf::from("/tmp/soul-core-test-unused.bin"),
            peer_addr_override: Some("127.0.0.1:2242".into()),
            peer_lookup_timeout: Duration::from_secs(1),
            connection_type: "P".into(),
            wait_port: None,
            skip_connect_probe: true,
            search_mode: SearchMode::Auto,
            strict_track: None,
        };
        let err = client
            .search_select_and_download(&request)
            .await
            .expect_err("invalid index expected");
        assert!(matches!(
            err,
            SearchSelectDownloadError::InvalidSearchResultIndex { .. }
        ));

        server.await.expect("server task");
    }

    #[tokio::test]
    async fn search_select_and_download_mock_flow_works_for_flim_query() {
        let server_listener = TcpListener::bind("127.0.0.1:0").await.expect("bind server");
        let server_addr = server_listener.local_addr().expect("server addr");

        let peer_listener = TcpListener::bind("127.0.0.1:0").await.expect("bind peer");
        let peer_addr = peer_listener.local_addr().expect("peer addr");

        let server_task = tokio::spawn(async move {
            let (mut socket, _) = server_listener.accept().await.expect("accept server");
            let login = read_frame(&mut socket).await.expect("login frame");
            assert_eq!(login.code, CODE_SM_LOGIN);
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            let request = read_frame(&mut socket).await.expect("search request");
            assert_eq!(request.code, CODE_SM_FILE_SEARCH);

            let summary = encode_server_message(&ServerMessage::FileSearchResponseSummary(
                SearchResponseSummary {
                    username: "peer_aphex".into(),
                    token: 9101,
                    files_count: 1,
                    slots_free: 1,
                    speed: 1024,
                    in_queue: false,
                    files: vec![SearchFileSummary {
                        file_path: "Music\\Aphex Twin\\Flim.mp3".into(),
                        file_size: 6,
                        extension: "mp3".into(),
                        attr_count: 0,
                    }],
                },
            ));
            write_frame(&mut socket, &summary)
                .await
                .expect("write summary");
        });

        let peer_task = tokio::spawn(async move {
            let (mut socket, _) = peer_listener.accept().await.expect("accept peer");
            let transfer_request = read_frame(&mut socket).await.expect("transfer request");
            assert_eq!(transfer_request.code, protocol::CODE_PM_TRANSFER_REQUEST);
            let parsed = parse_transfer_request(&transfer_request.payload).expect("parse request");
            assert_eq!(parsed.token, 9202);
            assert_eq!(parsed.virtual_path, "Music\\Aphex Twin\\Flim.mp3");

            let response = protocol::build_transfer_response(9202, true, "");
            write_frame(&mut socket, &response)
                .await
                .expect("write transfer response");
            socket.write_all(b"flim!!").await.expect("write payload");
            socket.shutdown().await.expect("shutdown peer");
        });

        let mut client = SessionClient::connect(&server_addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "fede_test1234".into(),
                password: "fede1234".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");

        let output = std::env::temp_dir().join("neosoulseek-flim-mock-test.bin");
        let request = SearchSelectDownloadRequest {
            search_token: 9101,
            query: "aphex twin flim".into(),
            search_timeout: Duration::from_secs(1),
            max_messages: 16,
            result_index: 0,
            file_index: 0,
            transfer_token: 9202,
            output_path: output.clone(),
            peer_addr_override: Some(peer_addr.to_string()),
            peer_lookup_timeout: Duration::from_secs(1),
            connection_type: "P".into(),
            wait_port: None,
            skip_connect_probe: true,
            search_mode: SearchMode::Auto,
            strict_track: None,
        };
        let result = client
            .search_select_and_download(&request)
            .await
            .expect("download result");

        assert_eq!(result.selected_username, "peer_aphex");
        assert_eq!(result.selected_virtual_path, "Music\\Aphex Twin\\Flim.mp3");
        assert_eq!(result.bytes_written, 6);

        let written = fs::read(&result.output_path).await.expect("read output");
        assert_eq!(written, b"flim!!");

        let _ = fs::remove_file(output).await;
        server_task.await.expect("join server task");
        peer_task.await.expect("join peer task");
    }

    #[tokio::test]
    async fn login_and_search_send_expected_codes() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let first = read_frame(&mut socket).await.expect("first");
            let login_response = login_success_frame();
            write_frame(&mut socket, &login_response)
                .await
                .expect("write login response");
            let second = read_frame(&mut socket).await.expect("second");
            (first.code, second.code)
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 157,
                minor_version: 19,
            })
            .await
            .expect("login");
        client.search(12345, "aphex twin").await.expect("search");

        let (first_code, second_code) = server.await.expect("server task");
        assert_eq!(first_code, CODE_SM_LOGIN);
        assert_eq!(second_code, CODE_SM_FILE_SEARCH);
    }

    #[tokio::test]
    async fn search_and_collect_returns_server_messages() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            let login_response = login_success_frame();
            write_frame(&mut socket, &login_response)
                .await
                .expect("write login response");
            let _search = read_frame(&mut socket).await.expect("search frame");

            let speed_frame = encode_server_message(&ServerMessage::DownloadSpeed(SpeedPayload {
                bytes_per_sec: 4096,
            }));
            write_frame(&mut socket, &speed_frame)
                .await
                .expect("write speed");
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 157,
                minor_version: 19,
            })
            .await
            .expect("login");

        let messages = client
            .search_and_collect(12345, "ambient", Duration::from_millis(250), 3)
            .await
            .expect("collect");
        assert!(!messages.is_empty());

        server.await.expect("server task");
    }

    #[tokio::test]
    async fn private_message_ack_flow_works() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            let message = read_frame(&mut socket).await.expect("message frame");
            assert_eq!(message.code, CODE_SM_MESSAGE_USER);

            let ack_frame = encode_server_message(&ServerMessage::MessageAcked(
                protocol::MessageAckedPayload { message_id: 42 },
            ));
            write_frame(&mut socket, &ack_frame)
                .await
                .expect("write ack frame");
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");
        client
            .send_private_message("alice", "hello from test")
            .await
            .expect("send private message");
        let ack = client
            .wait_message_ack(Duration::from_secs(1))
            .await
            .expect("wait ack");
        assert_eq!(ack.message_id, 42);

        server.await.expect("server task");
    }

    #[tokio::test]
    async fn user_state_and_peer_address_operations_parse_typed_payloads() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            let status_request = read_frame(&mut socket).await.expect("status request");
            assert_eq!(status_request.code, CODE_SM_GET_USER_STATUS);
            let status_response = encode_server_message(&ServerMessage::GetUserStatusResponse(
                UserStatusResponsePayload {
                    username: "alice".into(),
                    status: 2,
                    privileged: true,
                },
            ));
            write_frame(&mut socket, &status_response)
                .await
                .expect("write status response");

            let stats_request = read_frame(&mut socket).await.expect("stats request");
            assert_eq!(stats_request.code, CODE_SM_GET_USER_STATS);
            let stats_response = encode_server_message(&ServerMessage::GetUserStatsResponse(
                UserStatsResponsePayload {
                    username: "alice".into(),
                    avg_speed: 4096,
                    download_num: 11,
                    files: 100,
                    dirs: 12,
                },
            ));
            write_frame(&mut socket, &stats_response)
                .await
                .expect("write stats response");

            let peer_addr_request = read_frame(&mut socket).await.expect("peer addr request");
            assert_eq!(peer_addr_request.code, CODE_SM_GET_PEER_ADDRESS);
            let peer_addr_response = encode_server_message(&ServerMessage::GetPeerAddressResponse(
                PeerAddressResponsePayload {
                    username: "alice".into(),
                    ip_address: "203.0.113.10".into(),
                    port: 2242,
                    obfuscation_type: 1,
                    obfuscated_port: 5555,
                },
            ));
            write_frame(&mut socket, &peer_addr_response)
                .await
                .expect("write peer addr response");
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");

        let status = client
            .get_user_status("alice", Duration::from_secs(1))
            .await
            .expect("status payload");
        assert_eq!(status.status, 2);
        assert!(status.privileged);

        let stats = client
            .get_user_stats("alice", Duration::from_secs(1))
            .await
            .expect("stats payload");
        assert_eq!(stats.avg_speed, 4096);
        assert_eq!(stats.download_num, 11);

        let peer_address = client
            .get_peer_address("alice", Duration::from_secs(1))
            .await
            .expect("peer address payload");
        assert_eq!(peer_address.ip_address, "203.0.113.10");
        assert_eq!(peer_address.port, 2242);

        server.await.expect("server task");
    }

    #[tokio::test]
    async fn connect_peer_message_users_and_watch_private_flow() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            let connect = read_frame(&mut socket).await.expect("connect request");
            assert_eq!(connect.code, CODE_SM_CONNECT_TO_PEER);
            let message_users = read_frame(&mut socket)
                .await
                .expect("message-users request");
            assert_eq!(message_users.code, CODE_SM_MESSAGE_USERS);

            let incoming = encode_server_message(&ServerMessage::MessageUserIncoming(
                MessageUserIncomingPayload {
                    message_id: 77,
                    timestamp: 1_705_000_000,
                    username: "bob".into(),
                    message: "hello".into(),
                    is_new: true,
                },
            ));
            write_frame(&mut socket, &incoming)
                .await
                .expect("write incoming message");
            let ack = encode_server_message(&ServerMessage::MessageAcked(
                protocol::MessageAckedPayload { message_id: 77 },
            ));
            write_frame(&mut socket, &ack).await.expect("write ack");
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");
        client
            .connect_to_peer("bob", 77, "P")
            .await
            .expect("connect peer request");
        client
            .send_message_users(&["alice".to_string(), "bob".to_string()], "broadcast")
            .await
            .expect("message users request");

        let events = client
            .collect_private_events(Duration::from_secs(1), 4)
            .await
            .expect("private events");
        assert!(!events.is_empty());
        assert!(
            events
                .iter()
                .any(|event| matches!(event, PrivateEvent::Message(_)))
        );
        assert!(
            events
                .iter()
                .any(|event| matches!(event, PrivateEvent::Ack(_)))
        );

        server.await.expect("server task");
    }

    #[tokio::test]
    async fn list_rooms_returns_room_list_payload() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");
            let room_list_request = read_frame(&mut socket).await.expect("room list request");
            assert_eq!(room_list_request.code, CODE_SM_ROOM_LIST);

            let room_list_frame =
                encode_server_message(&ServerMessage::RoomList(RoomListPayload {
                    room_count: 2,
                    rooms: vec!["nicotine".into(), "electronic".into()],
                }));
            write_frame(&mut socket, &room_list_frame)
                .await
                .expect("write room list");
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");

        let room_list = client
            .list_rooms(Duration::from_secs(1))
            .await
            .expect("list rooms");
        assert_eq!(room_list.room_count, 2);
        assert_eq!(room_list.rooms.len(), 2);

        server.await.expect("server task");
    }

    #[tokio::test]
    async fn request_room_ticker_returns_payload() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            let ticker_request = read_frame(&mut socket).await.expect("ticker request");
            assert_eq!(ticker_request.code, CODE_SM_GET_ROOM_TICKER);

            let ticker = encode_server_message(&ServerMessage::RoomTicker(RoomTickerPayload {
                room: "nicotine".into(),
                entries: vec![
                    RoomTickerEntry {
                        username: "alice".into(),
                        ticker: "Now playing: Test".into(),
                    },
                    RoomTickerEntry {
                        username: "bob".into(),
                        ticker: "AFK".into(),
                    },
                ],
            }));
            write_frame(&mut socket, &ticker)
                .await
                .expect("write ticker response");
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");

        let payload = client
            .request_room_ticker("nicotine", Duration::from_secs(1))
            .await
            .expect("request ticker");
        assert_eq!(payload.room, "nicotine");
        assert_eq!(payload.entries.len(), 2);
        assert_eq!(payload.entries[0].username, "alice");

        server.await.expect("server task");
    }

    #[tokio::test]
    async fn collect_room_events_includes_ticker_snapshot() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            let ticker = encode_server_message(&ServerMessage::RoomTicker(RoomTickerPayload {
                room: "nicotine".into(),
                entries: vec![RoomTickerEntry {
                    username: "alice".into(),
                    ticker: "Now playing".into(),
                }],
            }));
            write_frame(&mut socket, &ticker)
                .await
                .expect("write ticker event");
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");

        let events = client
            .collect_room_events(Duration::from_millis(400), 4)
            .await
            .expect("collect room events");
        assert!(events.iter().any(|event| {
            matches!(
                event,
                RoomEvent::TickerSnapshot(payload)
                    if payload.room == "nicotine" && payload.entries.len() == 1
            )
        }));

        server.await.expect("server task");
    }

    #[tokio::test]
    async fn read_next_message_decodes_server_join_room_on_server_socket() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            // JoinRoom with empty users serializes to a single room string and is ambiguous
            // with peer code 14 payload shape. Server socket reads must remain server-scoped.
            let join_frame = encode_server_message(&ServerMessage::JoinRoom(JoinRoomPayload {
                room: "nicotine".into(),
                users: Vec::new(),
            }));
            write_frame(&mut socket, &join_frame)
                .await
                .expect("write join room");
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");

        let message = client.read_next_message().await.expect("read next message");
        let ProtocolMessage::Server(ServerMessage::JoinRoom(payload)) = message else {
            panic!("expected server join-room message");
        };
        assert_eq!(payload.room, "nicotine");
        assert!(payload.users.is_empty());

        server.await.expect("server task");
    }

    #[tokio::test]
    async fn join_and_leave_room_send_expected_codes() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");
            let join = read_frame(&mut socket).await.expect("join");
            let leave = read_frame(&mut socket).await.expect("leave");
            (join.code, leave.code)
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");
        client.join_room("nicotine").await.expect("join room");
        client.leave_room("nicotine").await.expect("leave room");

        let (join_code, leave_code) = server.await.expect("server task");
        assert_eq!(join_code, CODE_SM_JOIN_ROOM);
        assert_eq!(leave_code, CODE_SM_LEAVE_ROOM);
    }

    #[tokio::test]
    async fn room_moderation_operations_send_expected_codes() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            let add_member = read_frame(&mut socket).await.expect("add member");
            let remove_member = read_frame(&mut socket).await.expect("remove member");
            let add_operator = read_frame(&mut socket).await.expect("add operator");
            let remove_operator = read_frame(&mut socket).await.expect("remove operator");
            (
                add_member.code,
                remove_member.code,
                add_operator.code,
                remove_operator.code,
            )
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");
        client
            .add_room_member("private-room", "bob")
            .await
            .expect("add member");
        client
            .remove_room_member("private-room", "bob")
            .await
            .expect("remove member");
        client
            .add_room_operator("private-room", "alice")
            .await
            .expect("add operator");
        client
            .remove_room_operator("private-room", "alice")
            .await
            .expect("remove operator");

        let (add_member, remove_member, add_operator, remove_operator) =
            server.await.expect("server task");
        assert_eq!(add_member, CODE_SM_ADD_ROOM_MEMBER);
        assert_eq!(remove_member, CODE_SM_REMOVE_ROOM_MEMBER);
        assert_eq!(add_operator, CODE_SM_ADD_ROOM_OPERATOR);
        assert_eq!(remove_operator, CODE_SM_REMOVE_ROOM_OPERATOR);
    }

    #[tokio::test]
    async fn room_and_term_control_operations_send_expected_codes() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            let add_chatroom = read_frame(&mut socket).await.expect("add chatroom");
            let add_like_term = read_frame(&mut socket).await.expect("add like term");
            let remove_like_term = read_frame(&mut socket).await.expect("remove like term");
            (add_chatroom.code, add_like_term.code, remove_like_term.code)
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");
        client.add_chatroom("neo-room").await.expect("add chatroom");
        client.add_like_term("idm").await.expect("add like term");
        client
            .remove_like_term("idm")
            .await
            .expect("remove like term");

        let (add_chatroom, add_like_term, remove_like_term) = server.await.expect("server task");
        assert_eq!(add_chatroom, CODE_SM_ADD_CHATROOM);
        assert_eq!(add_like_term, CODE_SM_ADD_LIKE_TERM);
        assert_eq!(remove_like_term, CODE_SM_REMOVE_LIKE_TERM);
    }

    #[tokio::test]
    async fn social_control_operations_send_expected_codes() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            let ignore = read_frame(&mut socket).await.expect("ignore user");
            let unignore = read_frame(&mut socket).await.expect("unignore user");
            let give = read_frame(&mut socket).await.expect("give privilege");
            let inform = read_frame(&mut socket).await.expect("inform privilege");
            let ack = read_frame(&mut socket).await.expect("inform ack");
            let ban = read_frame(&mut socket).await.expect("ban user");
            (
                ignore.code,
                unignore.code,
                give.code,
                inform.code,
                ack.code,
                ban.code,
            )
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");
        client.ignore_user("bob").await.expect("ignore");
        client.unignore_user("bob").await.expect("unignore");
        client
            .give_privilege("bob", 7)
            .await
            .expect("give privilege");
        client
            .inform_user_of_privileges(42, "bob")
            .await
            .expect("inform privilege");
        client
            .inform_user_of_privileges_ack(42)
            .await
            .expect("inform ack");
        client.ban_user("eve").await.expect("ban");

        let (ignore, unignore, give, inform, ack, ban) = server.await.expect("server task");
        assert_eq!(ignore, CODE_SM_IGNORE_USER);
        assert_eq!(unignore, CODE_SM_UNIGNORE_USER);
        assert_eq!(give, CODE_SM_GIVE_PRIVILEGE);
        assert_eq!(inform, CODE_SM_INFORM_USER_OF_PRIVILEGES);
        assert_eq!(ack, CODE_SM_INFORM_USER_OF_PRIVILEGES_ACK);
        assert_eq!(ban, CODE_SM_BAN_USER);
    }

    #[tokio::test]
    async fn get_own_privileges_status_returns_payload() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            let request = read_frame(&mut socket)
                .await
                .expect("own privileges request");
            assert_eq!(request.code, CODE_SM_GET_OWN_PRIVILEGES_STATUS);

            let response = encode_server_message(&ServerMessage::OwnPrivilegesStatus(
                OwnPrivilegesStatusPayload {
                    time_left_seconds: 7_200,
                },
            ));
            write_frame(&mut socket, &response)
                .await
                .expect("write own privileges response");
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");

        let payload = client
            .get_own_privileges_status(Duration::from_secs(1))
            .await
            .expect("own privileges");
        assert_eq!(payload.time_left_seconds, 7_200);
        server.await.expect("server task");
    }

    #[tokio::test]
    async fn get_user_privileges_status_returns_payload() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            let request = read_frame(&mut socket)
                .await
                .expect("user privileges request");
            assert_eq!(request.code, CODE_SM_GET_USER_PRIVILEGES_STATUS);

            let response = encode_server_message(&ServerMessage::UserPrivilegesStatus(
                UserPrivilegesStatusPayload {
                    username: "bob".into(),
                    privileged: true,
                },
            ));
            write_frame(&mut socket, &response)
                .await
                .expect("write user privileges response");
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");

        let payload = client
            .get_user_privileges_status("bob", Duration::from_secs(1))
            .await
            .expect("user privileges");
        assert_eq!(payload.username, "bob");
        assert!(payload.privileged);
        server.await.expect("server task");
    }

    #[tokio::test]
    async fn set_upload_speed_sends_expected_code_and_payload() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            let request = read_frame(&mut socket).await.expect("upload speed request");
            assert_eq!(request.code, CODE_SM_UPLOAD_SPEED);
            assert_eq!(request.payload, 256_000_u32.to_le_bytes());
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");
        client
            .set_upload_speed(256_000)
            .await
            .expect("set upload speed");

        server.await.expect("server task");
    }

    #[tokio::test]
    async fn collect_room_events_decodes_presence_messages() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            let joined =
                encode_server_message(&ServerMessage::UserJoinedRoom(RoomPresenceEventPayload {
                    room: "nicotine".into(),
                    username: "bob".into(),
                }));
            let left =
                encode_server_message(&ServerMessage::UserLeftRoom(RoomPresenceEventPayload {
                    room: "nicotine".into(),
                    username: "bob".into(),
                }));

            write_frame(&mut socket, &joined)
                .await
                .expect("write joined event");
            write_frame(&mut socket, &left)
                .await
                .expect("write left event");
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");

        let events = client
            .collect_room_events(Duration::from_millis(400), 4)
            .await
            .expect("collect room events");
        assert!(
            events
                .iter()
                .any(|event| matches!(event, RoomEvent::UserJoined { room, username } if room == "nicotine" && username == "bob"))
        );
        assert!(
            events
                .iter()
                .any(|event| matches!(event, RoomEvent::UserLeft { room, username } if room == "nicotine" && username == "bob"))
        );

        server.await.expect("server task");
    }

    #[tokio::test]
    async fn get_recommendations_returns_payload() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            let request = read_frame(&mut socket)
                .await
                .expect("recommendations request");
            assert_eq!(request.code, CODE_SM_GET_RECOMMENDATIONS);

            let response = encode_server_message(&ServerMessage::GetRecommendationsResponse(
                RecommendationsPayload {
                    recommendations: vec![RecommendationEntry {
                        term: "flac".into(),
                        score: 3,
                    }],
                    unrecommendations: vec![],
                },
            ));
            write_frame(&mut socket, &response)
                .await
                .expect("write recommendations response");
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");

        let payload = client
            .get_recommendations(Duration::from_secs(1))
            .await
            .expect("get recommendations");
        assert_eq!(payload.recommendations.len(), 1);
        assert_eq!(payload.recommendations[0].term, "flac");

        server.await.expect("server task");
    }

    #[tokio::test]
    async fn get_global_recommendations_returns_payload() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            let request = read_frame(&mut socket)
                .await
                .expect("global recommendations request");
            assert_eq!(request.code, CODE_SM_GET_GLOBAL_RECOMMENDATIONS);

            let response = encode_server_message(&ServerMessage::GetGlobalRecommendationsResponse(
                RecommendationsPayload {
                    recommendations: vec![RecommendationEntry {
                        term: "lossless".into(),
                        score: 8,
                    }],
                    unrecommendations: vec![],
                },
            ));
            write_frame(&mut socket, &response)
                .await
                .expect("write global recommendations response");
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");

        let payload = client
            .get_global_recommendations(Duration::from_secs(1))
            .await
            .expect("get global recommendations");
        assert_eq!(payload.recommendations.len(), 1);
        assert_eq!(payload.recommendations[0].term, "lossless");

        server.await.expect("server task");
    }

    #[tokio::test]
    async fn get_user_recommendations_returns_payload() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            let request = read_frame(&mut socket)
                .await
                .expect("user recommendations request");
            assert_eq!(request.code, CODE_SM_GET_USER_RECOMMENDATIONS);

            let response = encode_server_message(&ServerMessage::GetUserRecommendationsResponse(
                UserRecommendationsPayload {
                    username: "bob".into(),
                    recommendations: RecommendationsPayload {
                        recommendations: vec![RecommendationEntry {
                            term: "ambient".into(),
                            score: 7,
                        }],
                        unrecommendations: vec![],
                    },
                },
            ));
            write_frame(&mut socket, &response)
                .await
                .expect("write user recommendations response");
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");

        let payload = client
            .get_user_recommendations("bob", Duration::from_secs(1))
            .await
            .expect("get user recommendations");
        assert_eq!(payload.username, "bob");
        assert_eq!(payload.recommendations.recommendations.len(), 1);

        server.await.expect("server task");
    }

    #[tokio::test]
    async fn get_similar_terms_returns_payload() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            let request = read_frame(&mut socket)
                .await
                .expect("similar terms request");
            assert_eq!(request.code, CODE_SM_GET_SIMILAR_TERMS);

            let response = encode_server_message(&ServerMessage::GetSimilarTermsResponse(
                SimilarTermsPayload {
                    term: "electronic".into(),
                    entries: vec![RecommendationEntry {
                        term: "idm".into(),
                        score: 5,
                    }],
                },
            ));
            write_frame(&mut socket, &response)
                .await
                .expect("write similar terms response");
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");

        let payload = client
            .get_similar_terms("electronic", Duration::from_secs(1))
            .await
            .expect("get similar terms");
        assert_eq!(payload.term, "electronic");
        assert_eq!(payload.entries.len(), 1);
        assert_eq!(payload.entries[0].term, "idm");

        server.await.expect("server task");
    }

    #[tokio::test]
    async fn get_privileged_list_returns_payload() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            let request = read_frame(&mut socket)
                .await
                .expect("privileged list request");
            assert_eq!(request.code, CODE_SM_PRIVILEGED_LIST);

            let response =
                encode_server_message(&ServerMessage::PrivilegedList(PrivilegedListPayload {
                    users: vec!["alice".into(), "bob".into()],
                }));
            write_frame(&mut socket, &response)
                .await
                .expect("write privileged list response");
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");

        let payload = client
            .get_privileged_list(Duration::from_secs(1))
            .await
            .expect("get privileged list");
        assert_eq!(payload.users, vec!["alice".to_string(), "bob".to_string()]);
        server.await.expect("server task");
    }

    #[tokio::test]
    async fn get_recommended_users_returns_payload() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            let request = read_frame(&mut socket)
                .await
                .expect("recommended users request");
            assert_eq!(request.code, CODE_SM_GET_RECOMMENDED_USERS);

            let response = encode_server_message(&ServerMessage::GetRecommendedUsersResponse(
                RecommendedUsersPayload {
                    users: vec![protocol::ScoredUserEntry {
                        username: "alice".into(),
                        score: 9,
                    }],
                },
            ));
            write_frame(&mut socket, &response)
                .await
                .expect("write recommended users response");
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");

        let payload = client
            .get_recommended_users(Duration::from_secs(1))
            .await
            .expect("get recommended users");
        assert_eq!(payload.users.len(), 1);
        assert_eq!(payload.users[0].username, "alice");
        assert_eq!(payload.users[0].score, 9);
        server.await.expect("server task");
    }

    #[tokio::test]
    async fn get_term_recommendations_returns_payload() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            let request = read_frame(&mut socket)
                .await
                .expect("term recommendations request");
            assert_eq!(request.code, CODE_SM_GET_TERM_RECOMMENDATIONS);

            let response = encode_server_message(&ServerMessage::GetTermRecommendationsResponse(
                TermRecommendationsPayload {
                    term: "idm".into(),
                    recommendations: vec![RecommendationEntry {
                        term: "ambient".into(),
                        score: 5,
                    }],
                },
            ));
            write_frame(&mut socket, &response)
                .await
                .expect("write term recommendations response");
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");

        let payload = client
            .get_term_recommendations("idm", Duration::from_secs(1))
            .await
            .expect("get term recommendations");
        assert_eq!(payload.term, "idm");
        assert_eq!(payload.recommendations.len(), 1);
        assert_eq!(payload.recommendations[0].term, "ambient");
        server.await.expect("server task");
    }

    #[tokio::test]
    async fn get_term_recommendations_accepts_term_only_response_variant() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            let request = read_frame(&mut socket)
                .await
                .expect("term recommendations request");
            assert_eq!(request.code, CODE_SM_GET_TERM_RECOMMENDATIONS);

            // Term-only payload is wire-ambiguous for code 111. Simulate server emitting this
            // shape so SessionClient normalizes it as an empty response instead of timing out.
            let response = encode_server_message(&ServerMessage::GetTermRecommendations(
                SimilarTermsRequestPayload { term: "idm".into() },
            ));
            write_frame(&mut socket, &response)
                .await
                .expect("write term-only recommendations payload");
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");

        let payload = client
            .get_term_recommendations("idm", Duration::from_secs(1))
            .await
            .expect("get term recommendations");
        assert_eq!(payload.term, "idm");
        assert!(payload.recommendations.is_empty());
        server.await.expect("server task");
    }

    #[tokio::test]
    async fn get_recommendation_users_returns_payload() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            let request = read_frame(&mut socket)
                .await
                .expect("recommendation users request");
            assert_eq!(request.code, CODE_SM_GET_RECOMMENDATION_USERS);

            let response = encode_server_message(&ServerMessage::GetRecommendationUsersResponse(
                RecommendationUsersPayload {
                    term: "idm".into(),
                    users: vec![protocol::ScoredUserEntry {
                        username: "charlie".into(),
                        score: 4,
                    }],
                },
            ));
            write_frame(&mut socket, &response)
                .await
                .expect("write recommendation users response");
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");

        let payload = client
            .get_recommendation_users("idm", Duration::from_secs(1))
            .await
            .expect("get recommendation users");
        assert_eq!(payload.term, "idm");
        assert_eq!(payload.users.len(), 1);
        assert_eq!(payload.users[0].username, "charlie");
        server.await.expect("server task");
    }

    #[tokio::test]
    async fn get_recommendation_users_accepts_term_only_response_variant() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            let request = read_frame(&mut socket)
                .await
                .expect("recommendation users request");
            assert_eq!(request.code, CODE_SM_GET_RECOMMENDATION_USERS);

            // Term-only payload is wire-ambiguous for code 112. Treat it as empty users response.
            let response = encode_server_message(&ServerMessage::GetRecommendationUsers(
                SimilarTermsRequestPayload { term: "idm".into() },
            ));
            write_frame(&mut socket, &response)
                .await
                .expect("write term-only recommendation-users payload");
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");

        let payload = client
            .get_recommendation_users("idm", Duration::from_secs(1))
            .await
            .expect("get recommendation users");
        assert_eq!(payload.term, "idm");
        assert!(payload.users.is_empty());
        server.await.expect("server task");
    }

    #[tokio::test]
    async fn get_my_recommendations_tolerates_no_response() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            write_frame(&mut socket, &login_success_frame())
                .await
                .expect("write login success");

            let request = read_frame(&mut socket)
                .await
                .expect("my recommendations request");
            assert_eq!(request.code, CODE_SM_GET_MY_RECOMMENDATIONS);
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect("login");

        let payload = client
            .get_my_recommendations(Duration::from_millis(200))
            .await
            .expect("get my recommendations");
        assert!(payload.recommendations.is_empty());
        assert!(payload.unrecommendations.is_empty());

        server.await.expect("server task");
    }

    #[tokio::test]
    async fn login_failure_keeps_connected_state() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            let failure = login_failure_frame("INVALIDPASS");
            write_frame(&mut socket, &failure)
                .await
                .expect("write login failure");
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        let err = client
            .login(&Credentials {
                username: "alice".into(),
                password: "wrong-pass".into(),
                client_version: 157,
                minor_version: 19,
            })
            .await
            .expect_err("must fail");
        assert_eq!(err, AuthError::InvalidPass);
        assert_eq!(client.state(), SessionState::Connected);
        server.await.expect("server task");
    }

    #[tokio::test]
    async fn login_invalid_version_returns_typed_error() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _login = read_frame(&mut socket).await.expect("login frame");
            let failure = login_failure_frame("INVALIDVERSION");
            write_frame(&mut socket, &failure)
                .await
                .expect("write login failure");
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        let err = client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 157,
                minor_version: 19,
            })
            .await
            .expect_err("must fail");
        assert_eq!(err, AuthError::InvalidVersion);
        assert_eq!(client.state(), SessionState::Connected);
        server.await.expect("server task");
    }

    #[tokio::test]
    async fn login_server_close_before_response_returns_typed_error() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (socket, _) = listener.accept().await.expect("accept");
            drop(socket);
        });

        let mut client = SessionClient::connect(&addr.to_string())
            .await
            .expect("connect");
        let err = client
            .login(&Credentials {
                username: "alice".into(),
                password: "secret-pass".into(),
                client_version: 160,
                minor_version: 1,
            })
            .await
            .expect_err("must fail");
        assert_eq!(err, AuthError::ServerClosedBeforeLoginResponse);
        assert_eq!(client.state(), SessionState::Connected);
        server.await.expect("server task");
    }

    #[tokio::test]
    async fn download_single_file_writes_output() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let request = read_frame(&mut socket).await.expect("transfer request");
            assert_eq!(request.code, protocol::CODE_PM_TRANSFER_REQUEST);

            let parsed = parse_transfer_request(&request.payload).expect("parse transfer request");
            assert_eq!(parsed.token, 555);

            let response = protocol::build_transfer_response(555, true, "");
            write_frame(&mut socket, &response)
                .await
                .expect("write response");
            socket.write_all(b"abc123").await.expect("write payload");
            socket.shutdown().await.expect("shutdown");
        });

        let output = std::env::temp_dir().join("neosoulseek-download-test.bin");
        let result = download_single_file(&DownloadPlan {
            peer_addr: addr.to_string(),
            token: 555,
            virtual_path: "Music\\Aphex Twin\\Track.flac".into(),
            file_size: 6,
            output_path: output.clone(),
        })
        .await
        .expect("download");

        let written = fs::read(&result.output_path).await.expect("read output");
        assert_eq!(written, b"abc123");
        assert_eq!(result.bytes_written, 6);

        let _ = fs::remove_file(output).await;
        server.await.expect("server task");
    }

    #[tokio::test]
    async fn upload_agent_manual_accept_and_send_bytes() {
        let source = std::env::temp_dir().join("neosoulseek-upload-source.bin");
        fs::write(&source, b"payload-xyz")
            .await
            .expect("write source");

        let agent = UploadAgent::bind_manual("127.0.0.1:0")
            .await
            .expect("bind agent");
        let addr = agent.local_addr().expect("local addr");

        let task = tokio::spawn(async move {
            agent
                .serve_single_manual(
                    ManualUploadDecision {
                        decision: UploadDecisionKind::Accept,
                        reason: String::new(),
                    },
                    Some(source),
                )
                .await
                .expect("serve")
        });

        let mut client = TcpStream::connect(addr).await.expect("connect agent");
        let req = protocol::build_transfer_request(
            TransferDirection::Download,
            999,
            "Music\\queued.flac",
            11,
        );
        write_frame(&mut client, &req).await.expect("write req");

        let response = read_frame(&mut client).await.expect("read response");
        let response = parse_transfer_response(&response.payload).expect("parse response");
        assert!(response.allowed);

        let mut bytes = Vec::new();
        client.read_to_end(&mut bytes).await.expect("read payload");
        assert_eq!(bytes, b"payload-xyz");

        let result = task.await.expect("join task");
        assert_eq!(result.bytes_sent, 11);
    }

    #[tokio::test]
    async fn upload_agent_manual_deny() {
        let agent = UploadAgent::bind_manual("127.0.0.1:0")
            .await
            .expect("bind agent");
        let addr = agent.local_addr().expect("local addr");

        let task = tokio::spawn(async move {
            agent
                .serve_single_manual(
                    ManualUploadDecision {
                        decision: UploadDecisionKind::Deny,
                        reason: "blocked by policy".into(),
                    },
                    None,
                )
                .await
                .expect("serve")
        });

        let mut client = TcpStream::connect(addr).await.expect("connect agent");
        let req = protocol::build_transfer_request(
            TransferDirection::Download,
            404,
            "Music\\blocked.flac",
            99,
        );
        write_frame(&mut client, &req).await.expect("write req");

        let response = read_frame(&mut client).await.expect("read response");
        let response = parse_transfer_response(&response.payload).expect("parse response");
        assert!(!response.allowed);
        assert_eq!(response.queue_or_reason, "blocked by policy");

        let result = task.await.expect("join task");
        assert_eq!(result.bytes_sent, 0);
    }

    #[test]
    fn normalize_peer_virtual_path_handles_prefix_and_separators() {
        assert_eq!(
            normalize_peer_virtual_path("@@alice/Music//Aphex Twin/Flim.flac"),
            "Music\\Aphex Twin\\Flim.flac"
        );
        assert_eq!(
            normalize_peer_virtual_path("  Music\\\\Aphex Twin\\\\Flim.flac\0\0"),
            "Music\\Aphex Twin\\Flim.flac"
        );
    }

    #[test]
    fn queue_upload_targets_include_raw_normalized_and_basename_variants() {
        let targets = queue_upload_targets("Music/Aphex Twin/02 Flim.flac");
        assert_eq!(targets[0], "Music/Aphex Twin/02 Flim.flac");
        assert!(targets.contains(&"Music\\Aphex Twin\\02 Flim.flac".to_string()));
        assert!(targets.contains(&"Aphex Twin\\02 Flim.flac".to_string()));
        assert!(targets.contains(&"02 Flim.flac".to_string()));
    }

    #[test]
    fn queue_upload_targets_preserve_share_prefixed_and_stripped_variants() {
        let targets = queue_upload_targets("@@alice\\Music\\Aphex Twin\\Flim.flac");
        assert_eq!(targets[0], "@@alice\\Music\\Aphex Twin\\Flim.flac");
        assert!(targets.contains(&"Music\\Aphex Twin\\Flim.flac".to_string()));
        assert!(targets.contains(&"Aphex Twin\\Flim.flac".to_string()));
        assert!(targets.contains(&"Flim.flac".to_string()));
    }

    #[test]
    fn file_not_shared_error_classifier_accepts_path_echo_variant() {
        let err = anyhow!("file not shared (user= path=): @@share\\Music\\Flim.flac");
        assert!(is_file_not_shared_error(&err));
    }

    #[tokio::test]
    async fn control_channel_upload_denied_frame_returns_explicit_error() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let denied = encode_peer_message(&PeerMessage::UploadDenied(protocol::UploadStatusPayload {
                username: "peer".into(),
                virtual_path: "Music\\Aphex Twin\\Flim.flac".into(),
                reason: "File not shared.".into(),
            }));
            write_frame(&mut socket, &denied)
                .await
                .expect("write denied frame");
        });

        let mut client = TcpStream::connect(addr).await.expect("connect");
        let err = try_read_transfer_body_on_control_channel(&mut client, 1_024)
            .await
            .expect_err("framed upload denied must surface as explicit error");
        assert!(
            err.to_string()
                .contains("peer denied transfer after allow"),
            "unexpected error: {err}"
        );

        server.await.expect("server task");
    }
}
