use anyhow::{Context, Result, anyhow, bail};
use protocol::{
    CODE_SM_GET_OWN_PRIVILEGES_STATUS, CODE_SM_GET_PEER_ADDRESS, CODE_SM_GET_RECOMMENDATION_USERS,
    CODE_SM_GET_RECOMMENDED_USERS, CODE_SM_GET_ROOM_TICKER, CODE_SM_GET_TERM_RECOMMENDATIONS,
    CODE_SM_GET_USER_PRIVILEGES_STATUS, CODE_SM_GET_USER_STATS, CODE_SM_GET_USER_STATUS,
    CODE_SM_LOGIN, CODE_SM_MESSAGE_ACKED, CODE_SM_PRIVILEGED_LIST, CODE_SM_ROOM_LIST, Frame,
    LoginFailureReason, LoginResponsePayload, MessageAckedPayload, MessageUserIncomingPayload,
    OwnPrivilegesStatusPayload, PeerAddressResponsePayload, PeerMessage, PrivilegedListPayload,
    ProtocolMessage, RecommendationUsersPayload, RecommendationsPayload, RecommendedUsersPayload,
    RoomListPayload, RoomMembersPayload, RoomOperatorsPayload, RoomTickerPayload,
    SearchResponseSummary, ServerMessage, SimilarTermsPayload, TermRecommendationsPayload,
    TransferDirection, TransferRequestPayload, TransferResponsePayload,
    UserPrivilegesStatusPayload, UserRecommendationsPayload, UserStatsResponsePayload,
    UserStatusResponsePayload, build_add_chatroom_request, build_add_like_term_request,
    build_add_room_member_request, build_add_room_operator_request, build_ban_user_request,
    build_connect_to_peer_request, build_file_search_request,
    build_get_global_recommendations_request, build_get_my_recommendations_request,
    build_get_own_privileges_status_request, build_get_peer_address_request,
    build_get_recommendation_users_request, build_get_recommendations_request,
    build_get_recommended_users_request, build_get_room_ticker_request,
    build_get_similar_terms_request, build_get_term_recommendations_request,
    build_get_user_privileges_status_request, build_get_user_recommendations_request,
    build_get_user_stats_request, build_get_user_status_request, build_give_privilege_request,
    build_ignore_user_request, build_inform_user_of_privileges_ack_request,
    build_inform_user_of_privileges_request, build_join_room_request, build_leave_room_request,
    build_login_request, build_message_user_request, build_message_users_request,
    build_privileged_list_request, build_remove_like_term_request,
    build_remove_room_member_request, build_remove_room_operator_request, build_room_list_request,
    build_room_members_request, build_room_operators_request, build_say_chatroom,
    build_transfer_request, build_unignore_user_request, build_upload_speed_request,
    decode_peer_message, decode_server_message, encode_peer_message, encode_server_message,
    split_first_frame,
};
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
    pub skip_connect_probe: bool,
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
}

pub type SoulClient = SessionClient;

impl SessionClient {
    const DEFAULT_LOGIN_RESPONSE_TIMEOUT: Duration = Duration::from_secs(5);

    pub fn new_disconnected() -> Self {
        Self {
            stream: None,
            state: SessionState::Disconnected,
            login_response_timeout: Self::DEFAULT_LOGIN_RESPONSE_TIMEOUT,
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
                Ok(())
            }
            LoginResponsePayload::Failure(failure) => Err(AuthError::from_failure_reason(
                failure.reason,
                failure.detail,
            )),
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

        let collected = self
            .search_and_collect(
                request.search_token,
                &request.query,
                request.search_timeout,
                request.max_messages,
            )
            .await
            .map_err(|err| SearchSelectDownloadError::Search(err.to_string()))?;
        let summaries = collect_search_summaries(&collected);
        if summaries.is_empty() {
            return Err(SearchSelectDownloadError::NoSearchResults {
                query: request.query.clone(),
            });
        }

        let selected_summary = summaries.get(request.result_index).ok_or_else(|| {
            SearchSelectDownloadError::InvalidSearchResultIndex {
                index: request.result_index,
                available: summaries.len(),
            }
        })?;

        let selected_file = selected_summary
            .files
            .get(request.file_index)
            .ok_or_else(|| SearchSelectDownloadError::InvalidSearchFileIndex {
                result_index: request.result_index,
                file_index: request.file_index,
                available: selected_summary.files.len(),
            })?;

        let peer_addr = if let Some(override_addr) = request.peer_addr_override.clone() {
            override_addr
        } else {
            let payload = self
                .get_peer_address(&selected_summary.username, request.peer_lookup_timeout)
                .await
                .map_err(|err| SearchSelectDownloadError::PeerLookup(err.to_string()))?;
            format!("{}:{}", payload.ip_address, payload.port)
        };

        if !request.skip_connect_probe {
            self.connect_to_peer(
                &selected_summary.username,
                request.transfer_token,
                &request.connection_type,
            )
            .await
            .map_err(|err| SearchSelectDownloadError::ConnectToPeer(err.to_string()))?;
        }

        let download_result = download_single_file(&DownloadPlan {
            peer_addr: peer_addr.clone(),
            token: request.transfer_token,
            virtual_path: selected_file.file_path.clone(),
            file_size: selected_file.file_size,
            output_path: request.output_path.clone(),
        })
        .await
        .map_err(|err| SearchSelectDownloadError::Download(err.to_string()))?;

        Ok(SearchSelectDownloadResult {
            selected_username: selected_summary.username.clone(),
            selected_virtual_path: selected_file.file_path.clone(),
            selected_file_size: selected_file.file_size,
            peer_addr,
            transfer_token: request.transfer_token,
            output_path: download_result.output_path,
            bytes_written: download_result.bytes_written,
        })
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

    let request = build_transfer_request(
        TransferDirection::Download,
        plan.token,
        &plan.virtual_path,
        plan.file_size,
    );
    write_frame(&mut stream, &request).await?;

    let response_frame = read_frame(&mut stream).await?;
    let response = match decode_peer_message(response_frame.code, &response_frame.payload)? {
        PeerMessage::TransferResponse(payload) => payload,
        other => bail!("unexpected first peer message during download: {other:?}"),
    };

    validate_transfer_response(plan.token, &response)?;
    ensure_parent_dir(&plan.output_path).await?;

    let mut content = Vec::new();
    stream
        .read_to_end(&mut content)
        .await
        .context("read file body")?;
    fs::write(&plan.output_path, &content)
        .await
        .with_context(|| format!("write output file: {}", plan.output_path.display()))?;

    Ok(DownloadResult {
        output_path: plan.output_path.clone(),
        bytes_written: content.len() as u64,
    })
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
            skip_connect_probe: true,
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
}
