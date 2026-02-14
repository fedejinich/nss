use anyhow::{Context, Result};
use protocol::Frame;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

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
    pub official_len: usize,
    pub neo_len: usize,
    pub first_diff_offset: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CaptureRunReport {
    pub run_id: String,
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
            format!("decode hex line {} from {}", line_no + 1, path_ref.display())
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
    let pair_count = official_frames.len().min(neo_frames.len());
    let mut comparisons = Vec::with_capacity(pair_count);
    let mut matched_pairs = 0;

    for idx in 0..pair_count {
        let official = &official_frames[idx];
        let neo = &neo_frames[idx];
        let diff = first_diff_offset(official, neo);
        let matches = diff.is_none() && official.len() == neo.len();
        if matches {
            matched_pairs += 1;
        }
        comparisons.push(CaptureFrameComparison {
            index: idx,
            matches,
            official_len: official.len(),
            neo_len: neo.len(),
            first_diff_offset: diff,
        });
    }

    CaptureRunReport {
        run_id: run_id.to_owned(),
        total_pairs: pair_count,
        matched_pairs,
        mismatched_pairs: pair_count.saturating_sub(matched_pairs),
        official_only: official_frames.len().saturating_sub(pair_count),
        neo_only: neo_frames.len().saturating_sub(pair_count),
        frame_comparisons: comparisons,
    }
}

pub fn compare_capture_run(run_dir: impl AsRef<Path>) -> Result<CaptureRunReport> {
    let run_dir = run_dir.as_ref();
    let run_id = run_dir
        .file_name()
        .map(|v| v.to_string_lossy().to_string())
        .unwrap_or_else(|| "run".to_string());

    let official_path = run_dir.join("official_frames.hex");
    let neo_path = run_dir.join("neo_frames.hex");

    let official = load_hex_lines(&official_path)?;
    let neo = load_hex_lines(&neo_path)?;

    Ok(compare_capture_sequences(&run_id, &official, &neo))
}

pub fn write_capture_report(path: impl AsRef<Path>, report: &CaptureRunReport) -> Result<()> {
    let rendered = serde_json::to_string_pretty(report).context("serialize capture report")?;
    fs::write(path.as_ref(), rendered + "\n")
        .with_context(|| format!("write capture report: {}", path.as_ref().display()))?;
    Ok(())
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

        assert_eq!(report.total_pairs, 2);
        assert_eq!(report.matched_pairs, 1);
        assert_eq!(report.mismatched_pairs, 1);
        assert_eq!(report.official_only, 0);
        assert_eq!(report.neo_only, 1);
    }
}
