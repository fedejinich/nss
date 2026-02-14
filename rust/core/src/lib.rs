use anyhow::{anyhow, bail, Context, Result};
use protocol::{
    build_file_search_request, build_login_request, build_transfer_request, parse_transfer_response, split_first_frame,
    Frame, TransferDirection,
};
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password_md5: String,
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

#[derive(Debug)]
pub struct SoulClient {
    stream: TcpStream,
}

impl SoulClient {
    pub async fn connect(server_addr: &str) -> Result<Self> {
        let stream = TcpStream::connect(server_addr)
            .await
            .with_context(|| format!("connect failed: {server_addr}"))?;
        Ok(Self { stream })
    }

    pub async fn login(&mut self, credentials: &Credentials) -> Result<()> {
        let frame = build_login_request(
            &credentials.username,
            &credentials.password_md5,
            credentials.client_version,
            credentials.minor_version,
        );
        write_frame(&mut self.stream, &frame).await
    }

    pub async fn search(&mut self, token: u32, search_text: &str) -> Result<()> {
        let frame = build_file_search_request(token, search_text);
        write_frame(&mut self.stream, &frame).await
    }

    pub async fn read_next_frame(&mut self) -> Result<Frame> {
        read_frame(&mut self.stream).await
    }
}

pub async fn write_frame(stream: &mut TcpStream, frame: &Frame) -> Result<()> {
    let bytes = frame.encode();
    stream.write_all(&bytes).await.context("write frame")?;
    stream.flush().await.context("flush frame")?;
    Ok(())
}

pub async fn read_frame(stream: &mut TcpStream) -> Result<Frame> {
    let mut len_buf = [0_u8; 4];
    stream.read_exact(&mut len_buf).await.context("read frame len")?;
    let body_len = u32::from_le_bytes(len_buf) as usize;

    let mut body = vec![0_u8; body_len];
    stream.read_exact(&mut body).await.context("read frame body")?;

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

    let response = read_frame(&mut stream).await?;
    if response.code != protocol::CODE_PM_TRANSFER_RESPONSE {
        bail!("unexpected peer response code: {}", response.code);
    }

    let parsed = parse_transfer_response(&response.payload).context("parse transfer response")?;
    if parsed.token != plan.token {
        bail!("token mismatch: expected {} got {}", plan.token, parsed.token);
    }
    if !parsed.allowed {
        let reason = if parsed.queue_or_reason.is_empty() {
            "peer denied transfer".to_string()
        } else {
            parsed.queue_or_reason
        };
        bail!(reason);
    }

    ensure_parent_dir(&plan.output_path).await?;

    let mut content = Vec::new();
    stream.read_to_end(&mut content).await.context("read file body")?;
    fs::write(&plan.output_path, &content)
        .await
        .with_context(|| format!("write output file: {}", plan.output_path.display()))?;

    Ok(DownloadResult {
        output_path: plan.output_path.clone(),
        bytes_written: content.len() as u64,
    })
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

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{build_transfer_response, CODE_SM_FILE_SEARCH, CODE_SM_LOGIN};
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn login_and_search_send_expected_codes() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let first = read_frame(&mut socket).await.expect("first");
            let second = read_frame(&mut socket).await.expect("second");
            (first.code, second.code)
        });

        let mut client = SoulClient::connect(&addr.to_string()).await.expect("connect");
        client
            .login(&Credentials {
                username: "alice".into(),
                password_md5: "0123456789abcdef0123456789abcdef".into(),
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
    async fn download_single_file_writes_output() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let request = read_frame(&mut socket).await.expect("transfer request");
            assert_eq!(request.code, protocol::CODE_PM_TRANSFER_REQUEST);

            let response = build_transfer_response(555, true, "");
            write_frame(&mut socket, &response).await.expect("write response");
            socket.write_all(b"abc123").await.expect("write payload");
            socket.shutdown().await.expect("shutdown");
        });

        let output = std::env::temp_dir().join("soul-dec-download-test.bin");
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
}
