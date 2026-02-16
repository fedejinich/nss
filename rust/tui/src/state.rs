use serde::{Deserialize, Serialize};

pub const PERSISTED_STATE_VERSION: u8 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PersistedDownloadStatus {
    InProgress,
    Done,
    Failed,
    Interrupted,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PersistedDownloadEntry {
    pub id: String,
    pub username: String,
    pub file_path: String,
    pub bytes: u64,
    pub status: PersistedDownloadStatus,
    pub started_at: i64,
    pub ended_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PersistedUiState {
    pub downloads_visible: bool,
}

impl Default for PersistedUiState {
    fn default() -> Self {
        Self {
            downloads_visible: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PersistedAppStateV1 {
    pub schema_version: u8,
    pub server: String,
    pub username: String,
    pub password: String,
    pub last_query: String,
    pub output_dir: String,
    pub downloads: Vec<PersistedDownloadEntry>,
    pub ui: PersistedUiState,
}

impl Default for PersistedAppStateV1 {
    fn default() -> Self {
        Self {
            schema_version: PERSISTED_STATE_VERSION,
            server: "server.slsknet.org:2416".to_string(),
            username: String::new(),
            password: String::new(),
            last_query: "aphex twin".to_string(),
            output_dir: "/tmp".to_string(),
            downloads: Vec::new(),
            ui: PersistedUiState::default(),
        }
    }
}

pub fn recover_in_progress_downloads(
    entries: &mut [PersistedDownloadEntry],
    now_unix_secs: i64,
) -> bool {
    let mut changed = false;
    for entry in entries {
        if entry.status == PersistedDownloadStatus::InProgress {
            entry.status = PersistedDownloadStatus::Interrupted;
            if entry.ended_at.is_none() {
                entry.ended_at = Some(now_unix_secs);
            }
            changed = true;
        }
    }
    changed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recovery_marks_in_progress_as_interrupted() {
        let mut rows = vec![
            PersistedDownloadEntry {
                id: "d1".to_string(),
                username: "alice".to_string(),
                file_path: "song-a.mp3".to_string(),
                bytes: 10,
                status: PersistedDownloadStatus::InProgress,
                started_at: 1,
                ended_at: None,
            },
            PersistedDownloadEntry {
                id: "d2".to_string(),
                username: "bob".to_string(),
                file_path: "song-b.mp3".to_string(),
                bytes: 20,
                status: PersistedDownloadStatus::Done,
                started_at: 2,
                ended_at: Some(3),
            },
        ];

        let changed = recover_in_progress_downloads(&mut rows, 42);
        assert!(changed);
        assert_eq!(rows[0].status, PersistedDownloadStatus::Interrupted);
        assert_eq!(rows[0].ended_at, Some(42));
        assert_eq!(rows[1].status, PersistedDownloadStatus::Done);
    }
}
