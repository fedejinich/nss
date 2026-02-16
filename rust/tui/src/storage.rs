use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use directories::ProjectDirs;

use crate::state::PersistedAppStateV1;

const STATE_FILE_NAME: &str = "tui-state-v1.json";

pub fn state_file_path() -> Result<PathBuf> {
    if let Ok(override_path) = std::env::var("NSS_TUI_STATE_FILE") {
        let trimmed = override_path.trim();
        if trimmed.is_empty() {
            bail!("NSS_TUI_STATE_FILE is set but empty");
        }
        return Ok(PathBuf::from(trimmed));
    }

    let project_dirs = ProjectDirs::from("org", "NeoSoulSeek", "NeoSoulSeek")
        .context("resolve project directories")?;
    Ok(project_dirs.data_local_dir().join(STATE_FILE_NAME))
}

pub fn load_state() -> Result<PersistedAppStateV1> {
    let path = state_file_path()?;
    load_state_from_path(&path)
}

pub fn save_state(state: &PersistedAppStateV1) -> Result<()> {
    let path = state_file_path()?;
    save_state_to_path(&path, state)
}

pub fn clear_download_history(state: &mut PersistedAppStateV1) -> Result<()> {
    state.downloads.clear();
    save_state(state)
}

#[cfg(test)]
fn clear_download_history_to_path(path: &Path, state: &mut PersistedAppStateV1) -> Result<()> {
    state.downloads.clear();
    save_state_to_path(path, state)
}

fn load_state_from_path(path: &Path) -> Result<PersistedAppStateV1> {
    if !path.exists() {
        return Ok(PersistedAppStateV1::default());
    }
    let raw = fs::read_to_string(path)
        .with_context(|| format!("read persisted state from {}", path.display()))?;
    let state: PersistedAppStateV1 = serde_json::from_str(&raw)
        .with_context(|| format!("parse persisted state from {}", path.display()))?;
    Ok(state)
}

fn save_state_to_path(path: &Path, state: &PersistedAppStateV1) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }

    let payload = serde_json::to_string_pretty(state).context("serialize persisted state")?;
    fs::write(path, payload)
        .with_context(|| format!("write persisted state to {}", path.display()))?;
    set_secure_permissions(path)?;
    Ok(())
}

fn set_secure_permissions(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600))
            .with_context(|| format!("set permissions on {}", path.display()))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_path() -> PathBuf {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock before unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("nss-tui-state-test-{now}.json"))
    }

    #[test]
    fn load_missing_state_uses_defaults() {
        let path = unique_path();
        let loaded = load_state_from_path(&path).expect("load default state");
        assert_eq!(loaded, PersistedAppStateV1::default());
    }

    #[test]
    fn save_then_load_roundtrip() {
        let path = unique_path();
        let mut state = PersistedAppStateV1::default();
        state.username = "alice".to_string();
        state.password = "secret".to_string();
        save_state_to_path(&path, &state).expect("save state");

        let loaded = load_state_from_path(&path).expect("load state");
        assert_eq!(loaded.username, "alice");
        assert_eq!(loaded.password, "secret");

        let _ = fs::remove_file(path);
    }

    #[test]
    fn clear_history_empties_downloads() {
        let path = unique_path();
        let mut state = PersistedAppStateV1::default();
        state.downloads.push(crate::state::PersistedDownloadEntry {
            id: "id-1".to_string(),
            username: "user".to_string(),
            file_path: "a.mp3".to_string(),
            bytes: 123,
            status: crate::state::PersistedDownloadStatus::Done,
            started_at: 1,
            ended_at: Some(2),
        });

        clear_download_history_to_path(&path, &mut state).expect("clear history");
        assert!(state.downloads.is_empty());

        let loaded = load_state_from_path(&path).expect("load state");
        assert!(loaded.downloads.is_empty());

        let _ = fs::remove_file(path);
    }
}
