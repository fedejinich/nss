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
}
