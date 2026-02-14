use anyhow::{Context, Result, anyhow, bail};
use protocol::{
    CODE_SM_FILE_SEARCH_RESPONSE, CODE_SM_LOGIN, Frame, LoginFailureReason, LoginResponsePayload,
    PeerMessage, ProtocolMessage, ServerMessage, TransferDirection, TransferRequestPayload,
    TransferResponsePayload, build_file_search_request, build_login_request,
    build_transfer_request, decode_message, decode_server_message, encode_peer_message,
    encode_server_message, parse_search_response_summary, split_first_frame,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Disconnected,
    Connected,
    LoggedIn,
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
            .map_err(|err| AuthError::ProtocolDecode(format!("read login response: {err}")))?;

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
        decode_message(&frame)
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
                    if frame.code == CODE_SM_FILE_SEARCH_RESPONSE {
                        if let Ok(summary) = parse_search_response_summary(&frame.payload) {
                            collected.push(ServerMessage::FileSearchResponseSummary(summary));
                        }
                        continue;
                    }

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
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AuthError {
    #[error("login rejected: INVALIDVERSION")]
    InvalidVersion,
    #[error("login rejected: INVALIDPASS")]
    InvalidPass,
    #[error("login rejected: INVALIDUSERNAME")]
    InvalidUsername,
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
    let response = match decode_message(&response_frame)? {
        ProtocolMessage::Peer(PeerMessage::TransferResponse(payload)) => payload,
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
        let request = match decode_message(&first_frame)? {
            ProtocolMessage::Peer(PeerMessage::TransferRequest(payload)) => payload,
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
        CODE_SM_FILE_SEARCH, CODE_SM_LOGIN, LoginResponsePayload, LoginResponseSuccessPayload,
        ServerMessage, SpeedPayload, encode_server_message, parse_transfer_request,
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
