use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const CODE_SM_LOGIN: u32 = 1;
pub const CODE_SM_FILE_SEARCH: u32 = 26;
pub const CODE_PM_TRANSFER_REQUEST: u32 = 40;
pub const CODE_PM_TRANSFER_RESPONSE: u32 = 41;

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
            bail!("frame length mismatch: declared={} actual={}", declared, buf.len() - 4);
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
        Ok(u32::from_le_bytes(bytes.try_into().expect("u32 slice length")))
    }

    pub fn read_u64(&mut self) -> Result<u64, DecoderError> {
        let bytes = self.take(8)?;
        Ok(u64::from_le_bytes(bytes.try_into().expect("u64 slice length")))
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferDirection {
    Upload = 0,
    Download = 1,
}

impl TransferDirection {
    pub fn as_u32(self) -> u32 {
        self as u32
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferResponse {
    pub token: u32,
    pub allowed: bool,
    pub queue_or_reason: String,
}

pub fn build_login_request(
    username: &str,
    password_md5: &str,
    client_version: u32,
    minor_version: u32,
) -> Frame {
    let mut w = PayloadWriter::new();
    w.write_string(username);
    w.write_string(password_md5);
    w.write_u32(client_version);
    w.write_u32(minor_version);
    Frame::new(CODE_SM_LOGIN, w.into_inner())
}

pub fn build_file_search_request(token: u32, search_text: &str) -> Frame {
    let mut w = PayloadWriter::new();
    w.write_u32(token);
    w.write_string(search_text);
    Frame::new(CODE_SM_FILE_SEARCH, w.into_inner())
}

pub fn build_transfer_request(
    direction: TransferDirection,
    token: u32,
    virtual_path: &str,
    file_size: u64,
) -> Frame {
    let mut w = PayloadWriter::new();
    w.write_u32(direction.as_u32());
    w.write_u32(token);
    w.write_string(virtual_path);
    w.write_u64(file_size);
    Frame::new(CODE_PM_TRANSFER_REQUEST, w.into_inner())
}

pub fn build_transfer_response(token: u32, allowed: bool, queue_or_reason: &str) -> Frame {
    let mut w = PayloadWriter::new();
    w.write_u32(token);
    w.write_bool_u32(allowed);
    w.write_string(queue_or_reason);
    Frame::new(CODE_PM_TRANSFER_RESPONSE, w.into_inner())
}

pub fn parse_transfer_response(payload: &[u8]) -> Result<TransferResponse> {
    let mut r = PayloadReader::new(payload);
    let token = r.read_u32()?;
    let allowed = r.read_bool_u32()?;
    let queue_or_reason = r.read_string()?;

    if r.remaining() != 0 {
        return Err(anyhow!("unexpected trailing bytes in transfer response: {}", r.remaining()));
    }

    Ok(TransferResponse {
        token,
        allowed,
        queue_or_reason,
    })
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

    #[test]
    fn roundtrip_frame() {
        let f = Frame::new(26, vec![1, 2, 3, 4]);
        let encoded = f.encode();
        let decoded = Frame::decode(&encoded).expect("decode");
        assert_eq!(decoded, f);
    }

    #[test]
    fn login_fixture_matches() {
        let frame = build_login_request("alice", "0123456789abcdef0123456789abcdef", 157, 19);
        let expected = decode_hex(
            "390000000100000005000000616c6963652000000030313233343536373839616263646566303132333435363738396162636465669d00000013000000",
        );
        assert_eq!(frame.encode(), expected);
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
