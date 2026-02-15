use std::env;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use protocol::ServerMessage;
use soul_core::{
    Credentials, SearchSelectDownloadRequest, SessionClient, SessionState, probe_login_versions,
};

use crate::state::{
    PersistedAppStateV1, PersistedDownloadEntry, PersistedDownloadStatus,
    recover_in_progress_downloads,
};
use crate::storage;

const LOG_LIMIT: usize = 120;

#[derive(Debug, Clone)]
pub struct SearchRow {
    pub username: String,
    pub file_path: String,
    pub file_size: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiPhase {
    LoginModal,
    Main,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    EditingQuery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginField {
    Server,
    Username,
    Password,
}

impl LoginField {
    pub fn next(self) -> Self {
        match self {
            Self::Server => Self::Username,
            Self::Username => Self::Password,
            Self::Password => Self::Server,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            Self::Server => Self::Password,
            Self::Username => Self::Server,
            Self::Password => Self::Username,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PendingAction {
    None,
    Login,
    Search,
    Download,
    RunDiagnostics,
    Quit,
}

pub struct App {
    pub phase: UiPhase,
    pub login_focus: LoginField,
    pub input_mode: InputMode,
    pub query_buffer: String,
    pub selected_result: usize,
    pub session_state: SessionState,
    pub login_error: Option<String>,
    pub logs: Vec<String>,
    pub results: Vec<SearchRow>,
    pub diagnostics_visible: bool,
    pub diagnostics_lines: Vec<String>,
    pub state: PersistedAppStateV1,
    pub output_dir: PathBuf,
    auto_login_pending: bool,
    transfer_token: u32,
    session: Option<SessionClient>,
}

impl App {
    pub fn bootstrap() -> anyhow::Result<Self> {
        let mut state = storage::load_state()?;
        let mut changed = false;

        if let Ok(server) = env::var("NSS_TEST_SERVER") {
            state.server = server;
            changed = true;
        }
        if let Ok(username) = env::var("NSS_TEST_USERNAME") {
            state.username = username;
            changed = true;
        }
        if let Ok(password) = env::var("NSS_TEST_PASSWORD") {
            state.password = password;
            changed = true;
        }
        if let Ok(query) = env::var("NSS_TUI_QUERY") {
            state.last_query = query;
            changed = true;
        }
        if let Ok(output_dir) = env::var("NSS_TUI_OUTPUT_DIR") {
            state.output_dir = output_dir;
            changed = true;
        }

        let recovered = recover_in_progress_downloads(&mut state.downloads, now_unix_secs());
        if changed || recovered {
            storage::save_state(&state)?;
        }

        let auto_login_pending = !state.username.trim().is_empty() && !state.password.is_empty();
        let output_dir = PathBuf::from(state.output_dir.clone());
        let query_buffer = state.last_query.clone();

        Ok(Self {
            phase: UiPhase::LoginModal,
            login_focus: LoginField::Username,
            input_mode: InputMode::Normal,
            query_buffer,
            selected_result: 0,
            session_state: SessionState::Disconnected,
            login_error: None,
            logs: vec!["NeoSoulSeek ready. Login is required before search/download.".to_string()],
            results: Vec::new(),
            diagnostics_visible: false,
            diagnostics_lines: vec![
                "Press g to run diagnostics.".to_string(),
                "The wizard checks server address parsing, DNS, TCP connect, and login probe."
                    .to_string(),
            ],
            state,
            output_dir,
            auto_login_pending,
            transfer_token: 555,
            session: None,
        })
    }

    pub fn query_for_display(&self) -> &str {
        if self.input_mode == InputMode::EditingQuery {
            self.query_buffer.as_str()
        } else {
            self.state.last_query.as_str()
        }
    }

    pub fn password_mask(&self) -> String {
        if self.state.password.is_empty() {
            "(empty)".to_string()
        } else {
            "*".repeat(self.state.password.len().max(8))
        }
    }

    pub fn selected_search_row(&self) -> Option<&SearchRow> {
        self.results.get(self.selected_result)
    }

    pub fn downloads_visible(&self) -> bool {
        self.state.ui.downloads_visible
    }

    pub fn push_log(&mut self, line: impl Into<String>) {
        self.logs.push(line.into());
        if self.logs.len() > LOG_LIMIT {
            let overflow = self.logs.len() - LOG_LIMIT;
            self.logs.drain(0..overflow);
        }
    }

    pub async fn attempt_auto_login_if_needed(&mut self) {
        if !self.auto_login_pending {
            return;
        }
        self.auto_login_pending = false;
        self.push_log("Attempting auto-login with persisted credentials...");
        self.login().await;
    }

    pub async fn login(&mut self) {
        let username = self.state.username.trim().to_string();
        if username.is_empty() || self.state.password.is_empty() {
            self.login_error = Some("Username and password are required.".to_string());
            self.push_log("Login failed: missing username or password.");
            return;
        }
        self.state.username = username;

        match SessionClient::connect(&self.state.server).await {
            Ok(mut client) => {
                let creds = Credentials {
                    username: self.state.username.clone(),
                    password: self.state.password.clone(),
                    client_version: 160,
                    minor_version: 1,
                };
                match client.login(&creds).await {
                    Ok(()) => {
                        self.session_state = client.state();
                        self.session = Some(client);
                        self.phase = UiPhase::Main;
                        self.login_error = None;
                        self.query_buffer = self.state.last_query.clone();
                        self.push_log(format!("Login ok: {}", self.state.server));
                        self.persist_state();
                    }
                    Err(err) => {
                        self.session = None;
                        self.session_state = SessionState::Disconnected;
                        self.phase = UiPhase::LoginModal;
                        self.login_error = Some(format!("Login failed: {err}"));
                        self.push_log(format!("Login failed: {err}"));
                    }
                }
            }
            Err(err) => {
                self.session = None;
                self.session_state = SessionState::Disconnected;
                self.phase = UiPhase::LoginModal;
                self.login_error = Some(format!("Connect failed: {err}"));
                self.push_log(format!("Connect failed: {err}"));
            }
        }
    }

    pub async fn search(&mut self) {
        if self.phase != UiPhase::Main {
            self.push_log("Search blocked: login is required.");
            return;
        }

        if self.state.last_query.trim().is_empty() {
            self.push_log("Search blocked: query cannot be empty.");
            return;
        }

        let mut client = match self.session.take() {
            Some(client) => client,
            None => {
                self.phase = UiPhase::LoginModal;
                self.session_state = SessionState::Disconnected;
                self.push_log("Search blocked: session is not logged in.");
                return;
            }
        };

        let response = client
            .search_and_collect(
                self.transfer_token,
                &self.state.last_query,
                Duration::from_secs(6),
                32,
            )
            .await;

        match response {
            Ok(messages) => {
                self.results.clear();
                for message in messages {
                    if let ServerMessage::FileSearchResponseSummary(summary) = message {
                        for file in summary.files {
                            self.results.push(SearchRow {
                                username: summary.username.clone(),
                                file_path: file.file_path,
                                file_size: file.file_size,
                            });
                        }
                    }
                }
                self.selected_result = 0;
                self.push_log(format!(
                    "Search ok: query='{}' rows={}",
                    self.state.last_query,
                    self.results.len()
                ));
            }
            Err(err) => {
                self.push_log(format!("Search failed: {err}"));
            }
        }

        self.session_state = client.state();
        self.session = Some(client);
        self.persist_state();
    }

    pub async fn download_selected(&mut self) {
        if self.phase != UiPhase::Main {
            self.push_log("Download blocked: login is required.");
            return;
        }

        let selected = match self.selected_search_row() {
            Some(row) => row.clone(),
            None => {
                self.push_log("Download blocked: no selected result.");
                return;
            }
        };

        let mut client = match self.session.take() {
            Some(client) => client,
            None => {
                self.phase = UiPhase::LoginModal;
                self.session_state = SessionState::Disconnected;
                self.push_log("Download blocked: session is not logged in.");
                return;
            }
        };

        let now = now_unix_secs();
        let download_id = format!("dl-{now}-{}", self.state.downloads.len() + 1);
        self.state.downloads.push(PersistedDownloadEntry {
            id: download_id.clone(),
            username: selected.username.clone(),
            file_path: selected.file_path.clone(),
            bytes: 0,
            status: PersistedDownloadStatus::InProgress,
            started_at: now,
            ended_at: None,
        });
        let download_index = self.state.downloads.len().saturating_sub(1);
        self.persist_state();

        let safe_name = selected.file_path.replace('\\', "_").replace('/', "_");
        let output_path = self.output_dir.join(format!("download-auto-{safe_name}"));
        let request = SearchSelectDownloadRequest {
            search_token: self.transfer_token,
            query: self.state.last_query.clone(),
            search_timeout: Duration::from_secs(6),
            max_messages: 32,
            result_index: self.selected_result,
            file_index: 0,
            transfer_token: self.transfer_token,
            output_path,
            peer_addr_override: None,
            peer_lookup_timeout: Duration::from_secs(5),
            connection_type: "P".to_string(),
            skip_connect_probe: false,
        };

        match client.search_select_and_download(&request).await {
            Ok(result) => {
                if let Some(entry) = self.state.downloads.get_mut(download_index) {
                    entry.status = PersistedDownloadStatus::Done;
                    entry.bytes = result.bytes_written;
                    entry.ended_at = Some(now_unix_secs());
                }
                self.push_log(format!(
                    "Download ok: user={} bytes={} path={}",
                    result.selected_username, result.bytes_written, result.selected_virtual_path
                ));
            }
            Err(err) => {
                if let Some(entry) = self.state.downloads.get_mut(download_index) {
                    entry.status = PersistedDownloadStatus::Failed;
                    entry.ended_at = Some(now_unix_secs());
                }
                self.push_log(format!("Download failed: {err}"));
            }
        }

        self.session_state = client.state();
        self.session = Some(client);
        self.persist_state();
    }

    pub fn clear_download_history(&mut self) {
        if cfg!(test) {
            self.state.downloads.clear();
            self.push_log("Downloads history cleared (files on disk were not removed).");
            return;
        }

        match storage::clear_download_history(&mut self.state) {
            Ok(()) => self.push_log("Downloads history cleared (files on disk were not removed)."),
            Err(err) => self.push_log(format!("Failed to clear downloads history: {err}")),
        }
    }

    pub fn toggle_downloads_panel(&mut self) {
        self.state.ui.downloads_visible = !self.state.ui.downloads_visible;
        self.persist_state();
        if self.state.ui.downloads_visible {
            self.push_log("Downloads panel is now visible.");
        } else {
            self.push_log("Downloads panel is now hidden.");
        }
    }

    pub fn move_selection(&mut self, delta: isize) {
        if self.results.is_empty() {
            self.selected_result = 0;
            return;
        }
        let len = self.results.len() as isize;
        let current = self.selected_result as isize;
        self.selected_result = (current + delta).clamp(0, len - 1) as usize;
    }

    pub async fn run_diagnostics(&mut self) {
        self.diagnostics_visible = true;
        self.diagnostics_lines.clear();
        self.diagnostics_lines
            .push("Running diagnostics wizard...".to_string());

        let server = self.state.server.trim().to_string();
        if server.is_empty() {
            self.diagnostics_lines
                .push("Server is empty. Fill Server and rerun (g).".to_string());
            self.push_log("Diagnostics failed: empty server.");
            return;
        }

        let (host, port) = match parse_server_host_port(&server) {
            Ok(value) => value,
            Err(err) => {
                self.diagnostics_lines
                    .push(format!("Server parse failed: {err}"));
                self.diagnostics_lines.push(
                    "Expected format is host:port (example: server.slsknet.org:2242).".to_string(),
                );
                self.push_log("Diagnostics failed: invalid server format.");
                return;
            }
        };
        self.diagnostics_lines
            .push(format!("Server parsed: host={host} port={port}"));

        match tokio::time::timeout(
            Duration::from_secs(4),
            tokio::net::lookup_host((host.as_str(), port)),
        )
        .await
        {
            Ok(Ok(mut addrs)) => {
                if let Some(addr) = addrs.next() {
                    self.diagnostics_lines
                        .push(format!("DNS lookup ok: first resolved endpoint {addr}"));
                } else {
                    self.diagnostics_lines.push(
                        "DNS lookup returned no addresses for the configured host.".to_string(),
                    );
                }
            }
            Ok(Err(err)) => {
                self.diagnostics_lines
                    .push(format!("DNS lookup failed: {err}"));
            }
            Err(_) => {
                self.diagnostics_lines
                    .push("DNS lookup timed out after 4s.".to_string());
            }
        }

        match tokio::time::timeout(Duration::from_secs(4), SessionClient::connect(&server)).await {
            Ok(Ok(_)) => self
                .diagnostics_lines
                .push("TCP connect check: ok.".to_string()),
            Ok(Err(err)) => self
                .diagnostics_lines
                .push(format!("TCP connect check failed: {err}")),
            Err(_) => self
                .diagnostics_lines
                .push("TCP connect check timed out after 4s.".to_string()),
        }

        let username = self.state.username.trim();
        if username.is_empty() || self.state.password.is_empty() {
            self.diagnostics_lines.push(
                "Login probe skipped: username/password are missing in the login form.".to_string(),
            );
            self.push_log("Diagnostics completed (auth probe skipped).");
            return;
        }

        self.diagnostics_lines
            .push("Login probe (version matrix):".to_string());
        match probe_login_versions(&server, username, &self.state.password).await {
            Ok(attempts) => {
                for attempt in attempts {
                    self.diagnostics_lines.push(format!(
                        " - {}/{} => {}",
                        attempt.client_version, attempt.minor_version, attempt.result
                    ));
                }
            }
            Err(err) => {
                self.diagnostics_lines
                    .push(format!("Login probe failed: {err}"));
            }
        }

        self.diagnostics_lines.push(
            "If login still fails with 'server closed before login response', verify account exists in official client."
                .to_string(),
        );
        self.push_log("Diagnostics completed.");
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> PendingAction {
        if self.diagnostics_visible {
            return self.handle_diagnostics_key(key);
        }
        if self.phase == UiPhase::LoginModal {
            return self.handle_login_modal_key(key);
        }
        self.handle_main_key(key)
    }

    fn handle_diagnostics_key(&mut self, key: KeyEvent) -> PendingAction {
        match key.code {
            KeyCode::Char('q') => PendingAction::Quit,
            KeyCode::Esc | KeyCode::Char('g') => {
                self.diagnostics_visible = false;
                PendingAction::None
            }
            KeyCode::Char('r') => PendingAction::RunDiagnostics,
            _ => PendingAction::None,
        }
    }

    fn handle_login_modal_key(&mut self, key: KeyEvent) -> PendingAction {
        match key.code {
            KeyCode::Char('q') => PendingAction::Quit,
            KeyCode::Char('g') => PendingAction::RunDiagnostics,
            KeyCode::Tab => {
                self.login_focus = self.login_focus.next();
                PendingAction::None
            }
            KeyCode::BackTab => {
                self.login_focus = self.login_focus.previous();
                PendingAction::None
            }
            KeyCode::Esc => {
                self.login_error = None;
                PendingAction::None
            }
            KeyCode::Enter => PendingAction::Login,
            KeyCode::Backspace => {
                let target = self.active_login_field_mut();
                target.pop();
                PendingAction::None
            }
            KeyCode::Char(ch) => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    return PendingAction::None;
                }
                self.active_login_field_mut().push(ch);
                PendingAction::None
            }
            _ => PendingAction::None,
        }
    }

    fn handle_main_key(&mut self, key: KeyEvent) -> PendingAction {
        if self.input_mode == InputMode::EditingQuery {
            return self.handle_query_edit_key(key);
        }
        match key.code {
            KeyCode::Char('q') => PendingAction::Quit,
            KeyCode::Char('g') => PendingAction::RunDiagnostics,
            KeyCode::Char('l') => {
                self.phase = UiPhase::LoginModal;
                self.session = None;
                self.session_state = SessionState::Disconnected;
                self.login_error = None;
                self.push_log("Logged out. Login is required.");
                PendingAction::None
            }
            KeyCode::Char('/') => {
                self.input_mode = InputMode::EditingQuery;
                self.query_buffer = self.state.last_query.clone();
                self.push_log("Query edit mode enabled. Press Enter to search.");
                PendingAction::None
            }
            KeyCode::Enter => PendingAction::Search,
            KeyCode::Char('d') => PendingAction::Download,
            KeyCode::Char('t') => {
                self.toggle_downloads_panel();
                PendingAction::None
            }
            KeyCode::Char('c') => {
                self.clear_download_history();
                PendingAction::None
            }
            KeyCode::Up => {
                self.move_selection(-1);
                PendingAction::None
            }
            KeyCode::Down => {
                self.move_selection(1);
                PendingAction::None
            }
            _ => PendingAction::None,
        }
    }

    fn handle_query_edit_key(&mut self, key: KeyEvent) -> PendingAction {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.query_buffer = self.state.last_query.clone();
                self.push_log("Query edit canceled.");
                PendingAction::None
            }
            KeyCode::Backspace => {
                self.query_buffer.pop();
                PendingAction::None
            }
            KeyCode::Enter => {
                let trimmed = self.query_buffer.trim();
                if trimmed.is_empty() {
                    self.push_log("Query cannot be empty.");
                    return PendingAction::None;
                }
                self.state.last_query = trimmed.to_string();
                self.query_buffer = self.state.last_query.clone();
                self.input_mode = InputMode::Normal;
                self.persist_state();
                PendingAction::Search
            }
            KeyCode::Char(ch) => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    return PendingAction::None;
                }
                self.query_buffer.push(ch);
                PendingAction::None
            }
            _ => PendingAction::None,
        }
    }

    fn active_login_field_mut(&mut self) -> &mut String {
        match self.login_focus {
            LoginField::Server => &mut self.state.server,
            LoginField::Username => &mut self.state.username,
            LoginField::Password => &mut self.state.password,
        }
    }

    pub fn active_login_field(&self) -> LoginField {
        self.login_focus
    }

    pub fn persist_state(&mut self) {
        if cfg!(test) {
            return;
        }
        self.state.output_dir = self.output_dir.to_string_lossy().to_string();
        if let Err(err) = storage::save_state(&self.state) {
            self.push_log(format!("State save failed: {err}"));
        }
    }

    #[cfg(test)]
    fn new_for_test(state: PersistedAppStateV1) -> Self {
        let output_dir = PathBuf::from(state.output_dir.clone());
        Self {
            phase: UiPhase::LoginModal,
            login_focus: LoginField::Username,
            input_mode: InputMode::Normal,
            query_buffer: state.last_query.clone(),
            selected_result: 0,
            session_state: SessionState::Disconnected,
            login_error: None,
            logs: vec!["test".to_string()],
            results: Vec::new(),
            diagnostics_visible: false,
            diagnostics_lines: Vec::new(),
            state,
            output_dir,
            auto_login_pending: false,
            transfer_token: 555,
            session: None,
        }
    }
}

fn parse_server_host_port(server: &str) -> Result<(String, u16), String> {
    let trimmed = server.trim();
    let (host, port_raw) = trimmed
        .rsplit_once(':')
        .ok_or_else(|| "missing ':' separator".to_string())?;
    if host.trim().is_empty() {
        return Err("host is empty".to_string());
    }
    let port = port_raw
        .parse::<u16>()
        .map_err(|_| format!("invalid port: {port_raw}"))?;
    Ok((host.to_string(), port))
}

fn now_unix_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn login_modal_blocks_main_actions() {
        let mut app = App::new_for_test(PersistedAppStateV1::default());
        app.phase = UiPhase::LoginModal;
        let action = app.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
        assert_eq!(action, PendingAction::None);
        assert_eq!(app.phase, UiPhase::LoginModal);
    }

    #[test]
    fn query_edit_enter_submits_search_action() {
        let mut app = App::new_for_test(PersistedAppStateV1::default());
        app.phase = UiPhase::Main;
        app.input_mode = InputMode::EditingQuery;
        app.query_buffer = "boards of canada".to_string();

        let action = app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(action, PendingAction::Search);
        assert_eq!(app.state.last_query, "boards of canada");
    }

    #[test]
    fn toggle_and_clear_downloads_behave_as_expected() {
        let mut app = App::new_for_test(PersistedAppStateV1::default());
        let before = app.downloads_visible();
        app.toggle_downloads_panel();
        assert_ne!(before, app.downloads_visible());

        app.state.downloads.push(PersistedDownloadEntry {
            id: "d1".to_string(),
            username: "u".to_string(),
            file_path: "f.mp3".to_string(),
            bytes: 10,
            status: PersistedDownloadStatus::Done,
            started_at: 1,
            ended_at: Some(2),
        });
        app.clear_download_history();
        assert!(app.state.downloads.is_empty());
    }

    #[test]
    fn diagnostics_key_opens_wizard_from_login_modal() {
        let mut app = App::new_for_test(PersistedAppStateV1::default());
        app.phase = UiPhase::LoginModal;
        let action = app.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
        assert_eq!(action, PendingAction::RunDiagnostics);
    }

    #[test]
    fn diagnostics_modal_can_be_closed_with_escape() {
        let mut app = App::new_for_test(PersistedAppStateV1::default());
        app.diagnostics_visible = true;
        let action = app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        assert_eq!(action, PendingAction::None);
        assert!(!app.diagnostics_visible);
    }

    #[test]
    fn server_parser_rejects_invalid_port() {
        let err = parse_server_host_port("server.slsknet.org:not-a-port").expect_err("invalid");
        assert!(err.contains("invalid port"));
    }
}
