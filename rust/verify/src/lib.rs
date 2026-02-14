use anyhow::{Context, Result};
use protocol::{Frame, decode_message};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ComparisonMode {
    Bytes,
    Semantic,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FrameComparison {
    pub fixture: String,
    pub matches: bool,
    pub expected_len: usize,
    pub actual_len: usize,
    pub first_diff_offset: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CaptureFrameComparison {
    pub index: usize,
    pub matches: bool,
    pub bytes_match: bool,
    pub semantic_matches: bool,
    pub semantic_first_diff_field: Option<String>,
    pub official_len: usize,
    pub neo_len: usize,
    pub first_diff_offset: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CaptureRunReport {
    pub run_id: String,
    pub comparison_mode: ComparisonMode,
    pub total_pairs: usize,
    pub matched_pairs: usize,
    pub mismatched_pairs: usize,
    pub official_only: usize,
    pub neo_only: usize,
    pub frame_comparisons: Vec<CaptureFrameComparison>,
}

pub fn load_hex_fixture(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let path_ref = path.as_ref();
    let raw = fs::read_to_string(path_ref)
        .with_context(|| format!("read fixture: {}", path_ref.display()))?;
    decode_hex(raw.trim()).with_context(|| format!("decode fixture hex: {}", path_ref.display()))
}

pub fn compare_fixture_hex(fixture_name: &str, expected: &[u8], actual: &[u8]) -> FrameComparison {
    let first_diff = first_diff_offset(expected, actual);
    FrameComparison {
        fixture: fixture_name.to_string(),
        matches: first_diff.is_none() && expected.len() == actual.len(),
        expected_len: expected.len(),
        actual_len: actual.len(),
        first_diff_offset: first_diff,
    }
}

pub fn compare_fixture_to_frame(path: impl AsRef<Path>, frame: &Frame) -> Result<FrameComparison> {
    let path_ref = path.as_ref();
    let expected = load_hex_fixture(path_ref)?;
    let actual = frame.encode();
    Ok(compare_fixture_hex(
        path_ref
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .as_deref()
            .unwrap_or("fixture"),
        &expected,
        &actual,
    ))
}

pub fn write_report(path: impl AsRef<Path>, comparisons: &[FrameComparison]) -> Result<()> {
    let report = serde_json::to_string_pretty(comparisons).context("serialize comparisons")?;
    fs::write(path.as_ref(), report + "\n")
        .with_context(|| format!("write report: {}", path.as_ref().display()))?;
    Ok(())
}

pub fn load_hex_lines(path: impl AsRef<Path>) -> Result<Vec<Vec<u8>>> {
    let path_ref = path.as_ref();
    let raw = fs::read_to_string(path_ref)
        .with_context(|| format!("read hex lines: {}", path_ref.display()))?;

    let mut out = Vec::new();
    for (line_no, line) in raw.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let bytes = decode_hex(trimmed).with_context(|| {
            format!(
                "decode hex line {} from {}",
                line_no + 1,
                path_ref.display()
            )
        })?;
        out.push(bytes);
    }

    Ok(out)
}

pub fn compare_capture_sequences(
    run_id: &str,
    official_frames: &[Vec<u8>],
    neo_frames: &[Vec<u8>],
) -> CaptureRunReport {
    compare_capture_sequences_with_mode(run_id, official_frames, neo_frames, ComparisonMode::Bytes)
}

pub fn compare_capture_sequences_with_mode(
    run_id: &str,
    official_frames: &[Vec<u8>],
    neo_frames: &[Vec<u8>],
    mode: ComparisonMode,
) -> CaptureRunReport {
    let pair_count = official_frames.len().min(neo_frames.len());
    let mut comparisons = Vec::with_capacity(pair_count);
    let mut matched_pairs = 0;

    for idx in 0..pair_count {
        let official = &official_frames[idx];
        let neo = &neo_frames[idx];
        let first_diff = first_diff_offset(official, neo);
        let bytes_match = first_diff.is_none() && official.len() == neo.len();

        let (semantic_matches, semantic_first_diff_field) = semantic_compare(official, neo);
        let matches = match mode {
            ComparisonMode::Bytes => bytes_match,
            ComparisonMode::Semantic => semantic_matches,
        };
        if matches {
            matched_pairs += 1;
        }

        comparisons.push(CaptureFrameComparison {
            index: idx,
            matches,
            bytes_match,
            semantic_matches,
            semantic_first_diff_field,
            official_len: official.len(),
            neo_len: neo.len(),
            first_diff_offset: first_diff,
        });
    }

    CaptureRunReport {
        run_id: run_id.to_owned(),
        comparison_mode: mode,
        total_pairs: pair_count,
        matched_pairs,
        mismatched_pairs: pair_count.saturating_sub(matched_pairs),
        official_only: official_frames.len().saturating_sub(pair_count),
        neo_only: neo_frames.len().saturating_sub(pair_count),
        frame_comparisons: comparisons,
    }
}

pub fn compare_capture_run(run_dir: impl AsRef<Path>) -> Result<CaptureRunReport> {
    compare_capture_run_with_mode(run_dir, ComparisonMode::Bytes)
}

pub fn compare_capture_run_with_mode(
    run_dir: impl AsRef<Path>,
    mode: ComparisonMode,
) -> Result<CaptureRunReport> {
    let run_dir = run_dir.as_ref();
    let run_id = run_dir
        .file_name()
        .map(|v| v.to_string_lossy().to_string())
        .unwrap_or_else(|| "run".to_string());

    let official_path = run_dir.join("official_frames.hex");
    let neo_path = run_dir.join("neo_frames.hex");

    let official = load_hex_lines(&official_path)?;
    let neo = load_hex_lines(&neo_path)?;

    Ok(compare_capture_sequences_with_mode(
        &run_id, &official, &neo, mode,
    ))
}

pub fn write_capture_report(path: impl AsRef<Path>, report: &CaptureRunReport) -> Result<()> {
    let rendered = serde_json::to_string_pretty(report).context("serialize capture report")?;
    fs::write(path.as_ref(), rendered + "\n")
        .with_context(|| format!("write capture report: {}", path.as_ref().display()))?;
    Ok(())
}

fn semantic_compare(official: &[u8], neo: &[u8]) -> (bool, Option<String>) {
    let official_norm = normalize_semantic_frame(official);
    let neo_norm = normalize_semantic_frame(neo);
    if official_norm == neo_norm {
        return (true, None);
    }

    (
        false,
        first_semantic_diff(&official_norm, &neo_norm, "semantic")
            .or_else(|| Some("semantic".to_string())),
    )
}

fn normalize_semantic_frame(bytes: &[u8]) -> Value {
    let frame = match Frame::decode(bytes) {
        Ok(frame) => frame,
        Err(err) => {
            return json!({
                "decode_error": err.to_string(),
                "frame_md5": format!("{:x}", md5::compute(bytes)),
            });
        }
    };

    match decode_message(&frame) {
        Ok(message) => json!({
            "code": frame.code,
            "known": true,
            "decoded": message,
        }),
        Err(_) => json!({
            "code": frame.code,
            "known": false,
            "payload_len": frame.payload.len(),
            "payload_md5": format!("{:x}", md5::compute(&frame.payload)),
        }),
    }
}

fn first_semantic_diff(expected: &Value, actual: &Value, path: &str) -> Option<String> {
    match (expected, actual) {
        (Value::Object(left), Value::Object(right)) => {
            let mut keys = BTreeSet::new();
            keys.extend(left.keys().cloned());
            keys.extend(right.keys().cloned());
            for key in keys {
                let next = format!("{path}.{key}");
                match (left.get(&key), right.get(&key)) {
                    (Some(a), Some(b)) => {
                        if let Some(diff) = first_semantic_diff(a, b, &next) {
                            return Some(diff);
                        }
                    }
                    _ => return Some(next),
                }
            }
            None
        }
        (Value::Array(left), Value::Array(right)) => {
            let min = left.len().min(right.len());
            for idx in 0..min {
                let next = format!("{path}[{idx}]");
                if let Some(diff) = first_semantic_diff(&left[idx], &right[idx], &next) {
                    return Some(diff);
                }
            }
            if left.len() != right.len() {
                Some(format!("{path}.len"))
            } else {
                None
            }
        }
        _ => {
            if expected == actual {
                None
            } else {
                Some(path.to_string())
            }
        }
    }
}

fn first_diff_offset(expected: &[u8], actual: &[u8]) -> Option<usize> {
    let min = expected.len().min(actual.len());
    for i in 0..min {
        if expected[i] != actual[i] {
            return Some(i);
        }
    }
    if expected.len() == actual.len() {
        None
    } else {
        Some(min)
    }
}

fn decode_hex(input: &str) -> Result<Vec<u8>> {
    let clean = input.trim();
    if clean.len() % 2 != 0 {
        anyhow::bail!("hex string length must be even");
    }

    let mut out = Vec::with_capacity(clean.len() / 2);
    let bytes = clean.as_bytes();
    for i in (0..bytes.len()).step_by(2) {
        let hi = (bytes[i] as char)
            .to_digit(16)
            .ok_or_else(|| anyhow::anyhow!("invalid hex char at {i}"))?;
        let lo = (bytes[i + 1] as char)
            .to_digit(16)
            .ok_or_else(|| anyhow::anyhow!("invalid hex char at {}", i + 1))?;
        out.push(((hi << 4) | lo) as u8);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{
        CODE_PM_TRANSFER_RESPONSE, CODE_SM_DOWNLOAD_SPEED, CODE_SM_GET_RECOMMENDATIONS,
        CODE_SM_USER_JOINED_ROOM, PayloadWriter,
    };

    fn transfer_response_frame_bytes(token: u32, allowed_raw: u32) -> Vec<u8> {
        let mut payload = Vec::new();
        payload.extend_from_slice(&token.to_le_bytes());
        payload.extend_from_slice(&allowed_raw.to_le_bytes());
        payload.extend_from_slice(&0_u32.to_le_bytes());
        Frame::new(CODE_PM_TRANSFER_RESPONSE, payload).encode()
    }

    fn server_speed_frame_bytes(speed: u32) -> Vec<u8> {
        let mut payload = Vec::new();
        payload.extend_from_slice(&speed.to_le_bytes());
        Frame::new(CODE_SM_DOWNLOAD_SPEED, payload).encode()
    }

    fn room_presence_frame_bytes(room: &str, username: &str) -> Vec<u8> {
        let mut writer = PayloadWriter::new();
        writer.write_string(room);
        writer.write_string(username);
        Frame::new(CODE_SM_USER_JOINED_ROOM, writer.into_inner()).encode()
    }

    fn recommendations_frame_bytes(term: &str, score: u32) -> Vec<u8> {
        let mut writer = PayloadWriter::new();
        writer.write_u32(1);
        writer.write_string(term);
        writer.write_u32(score);
        writer.write_u32(0);
        Frame::new(CODE_SM_GET_RECOMMENDATIONS, writer.into_inner()).encode()
    }

    #[test]
    fn comparison_detects_diff() {
        let cmp = compare_fixture_hex("fixture", &[0x00, 0x01], &[0x00, 0x02]);
        assert!(!cmp.matches);
        assert_eq!(cmp.first_diff_offset, Some(1));
    }

    #[test]
    fn comparison_matches_when_identical() {
        let cmp = compare_fixture_hex("fixture", &[0xaa, 0xbb], &[0xaa, 0xbb]);
        assert!(cmp.matches);
        assert_eq!(cmp.first_diff_offset, None);
    }

    #[test]
    fn capture_comparison_reports_pair_diffs() {
        let official = vec![vec![1, 2, 3], vec![9, 9]];
        let neo = vec![vec![1, 2, 3], vec![9, 8], vec![7, 7, 7]];
        let report = compare_capture_sequences("run-a", &official, &neo);

        assert_eq!(report.comparison_mode, ComparisonMode::Bytes);
        assert_eq!(report.total_pairs, 2);
        assert_eq!(report.matched_pairs, 1);
        assert_eq!(report.mismatched_pairs, 1);
        assert_eq!(report.official_only, 0);
        assert_eq!(report.neo_only, 1);
    }

    #[test]
    fn semantic_mode_accepts_different_bytes_when_fields_match() {
        let official = vec![transfer_response_frame_bytes(555, 1)];
        let neo = vec![transfer_response_frame_bytes(555, 2)];
        let report = compare_capture_sequences_with_mode(
            "run-semantic",
            &official,
            &neo,
            ComparisonMode::Semantic,
        );

        assert_eq!(report.total_pairs, 1);
        assert_eq!(report.matched_pairs, 1);
        assert!(!report.frame_comparisons[0].bytes_match);
        assert!(report.frame_comparisons[0].semantic_matches);
        assert_eq!(report.frame_comparisons[0].semantic_first_diff_field, None);
    }

    #[test]
    fn semantic_mode_reports_first_mismatch_field() {
        let official = vec![server_speed_frame_bytes(2048)];
        let neo = vec![server_speed_frame_bytes(1024)];
        let report = compare_capture_sequences_with_mode(
            "run-semantic-diff",
            &official,
            &neo,
            ComparisonMode::Semantic,
        );

        assert_eq!(report.total_pairs, 1);
        assert_eq!(report.matched_pairs, 0);
        assert!(!report.frame_comparisons[0].semantic_matches);
        assert!(
            report.frame_comparisons[0]
                .semantic_first_diff_field
                .as_deref()
                .unwrap_or_default()
                .contains("bytes_per_sec")
        );
    }

    #[test]
    fn semantic_mode_reports_room_presence_field_diff() {
        let official = vec![room_presence_frame_bytes("nicotine", "alice")];
        let neo = vec![room_presence_frame_bytes("nicotine", "bob")];
        let report = compare_capture_sequences_with_mode(
            "run-room-diff",
            &official,
            &neo,
            ComparisonMode::Semantic,
        );

        assert_eq!(report.total_pairs, 1);
        assert_eq!(report.matched_pairs, 0);
        assert!(!report.frame_comparisons[0].semantic_matches);
        assert!(
            report.frame_comparisons[0]
                .semantic_first_diff_field
                .as_deref()
                .unwrap_or_default()
                .contains("username")
        );
    }

    #[test]
    fn semantic_mode_reports_recommendation_term_diff() {
        let official = vec![recommendations_frame_bytes("flac", 3)];
        let neo = vec![recommendations_frame_bytes("lossless", 3)];
        let report = compare_capture_sequences_with_mode(
            "run-recommendation-diff",
            &official,
            &neo,
            ComparisonMode::Semantic,
        );

        assert_eq!(report.total_pairs, 1);
        assert_eq!(report.matched_pairs, 0);
        assert!(!report.frame_comparisons[0].semantic_matches);
        assert!(
            report.frame_comparisons[0]
                .semantic_first_diff_field
                .as_deref()
                .unwrap_or_default()
                .contains("term")
        );
    }
}
