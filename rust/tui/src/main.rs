use std::env;
use std::io;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use soul_core::{
    Credentials, SearchSelectDownloadRequest, SessionClient, SessionState,
};

#[derive(Debug, Clone)]
struct SearchRow {
    username: String,
    file_path: String,
    file_size: u64,
}

#[derive(Debug, Clone)]
struct TransferRow {
    status: String,
    user: String,
    file_path: String,
    bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UploadDecision {
    Accept,
    Deny,
}

impl UploadDecision {
    fn as_str(self) -> &'static str {
        match self {
            Self::Accept => "accept",
            Self::Deny => "deny",
        }
    }
}

struct App {
    server: String,
    username: String,
    password: String,
    client_version: u32,
    minor_version: u32,
    query: String,
    transfer_token: u32,
    output_dir: PathBuf,
    results: Vec<SearchRow>,
    selected_result: usize,
    transfers: Vec<TransferRow>,
    logs: Vec<String>,
    upload_decision: UploadDecision,
    session_state: SessionState,
    session: Option<SessionClient>,
}

impl App {
    fn from_env() -> Self {
        Self {
            server: env::var("NSS_TEST_SERVER").unwrap_or_else(|_| "server.slsknet.org:2242".to_string()),
            username: env::var("NSS_TEST_USERNAME").unwrap_or_default(),
            password: env::var("NSS_TEST_PASSWORD").unwrap_or_default(),
            client_version: 160,
            minor_version: 1,
            query: "aphex twin".to_string(),
            transfer_token: 555,
            output_dir: PathBuf::from("/tmp"),
            results: Vec::new(),
            selected_result: 0,
            transfers: Vec::new(),
            logs: vec!["ready: press l=login s=search d=download q=quit".to_string()],
            upload_decision: UploadDecision::Deny,
            session_state: SessionState::Disconnected,
            session: None,
        }
    }

    fn log(&mut self, line: impl Into<String>) {
        self.logs.push(line.into());
        if self.logs.len() > 120 {
            let drain = self.logs.len() - 120;
            self.logs.drain(0..drain);
        }
    }

    fn selected(&self) -> Option<&SearchRow> {
        self.results.get(self.selected_result)
    }

    async fn login(&mut self) {
        if self.username.is_empty() || self.password.is_empty() {
            self.log("login error: NSS_TEST_USERNAME/NSS_TEST_PASSWORD are required");
            return;
        }

        match SessionClient::connect(&self.server).await {
            Ok(mut client) => {
                let creds = Credentials {
                    username: self.username.clone(),
                    password: self.password.clone(),
                    client_version: self.client_version,
                    minor_version: self.minor_version,
                };
                match client.login(&creds).await {
                    Ok(()) => {
                        self.session_state = client.state();
                        self.session = Some(client);
                        self.log(format!("login ok: {}", self.server));
                    }
                    Err(err) => {
                        self.session = None;
                        self.session_state = SessionState::Disconnected;
                        self.log(format!("login failed: {err}"));
                    }
                }
            }
            Err(err) => {
                self.log(format!("connect failed: {err}"));
            }
        }
    }

    async fn search(&mut self) {
        let mut client = match self.session.take() {
            Some(client) => client,
            None => {
                self.log("search error: not logged in");
                return;
            }
        };

        let response = client
            .search_and_collect(
                self.transfer_token,
                &self.query,
                Duration::from_secs(6),
                32,
            )
            .await;

        match response {
            Ok(messages) => {
                self.results.clear();
                for message in messages {
                    if let protocol::ServerMessage::FileSearchResponseSummary(summary) = message {
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
                self.log(format!("search ok: query='{}' rows={}", self.query, self.results.len()));
            }
            Err(err) => {
                self.log(format!("search failed: {err}"));
            }
        }

        self.session_state = client.state();
        self.session = Some(client);
    }

    async fn download_selected(&mut self) {
        let selected = match self.selected() {
            Some(row) => row.clone(),
            None => {
                self.log("download error: no selected row");
                return;
            }
        };

        let mut client = match self.session.take() {
            Some(client) => client,
            None => {
                self.log("download error: not logged in");
                return;
            }
        };

        let output_name = selected
            .file_path
            .replace('\\', "_")
            .replace('/', "_");
        let output_path = self
            .output_dir
            .join(format!("download-auto-{}", output_name));

        let request = SearchSelectDownloadRequest {
            search_token: self.transfer_token,
            query: self.query.clone(),
            search_timeout: Duration::from_secs(6),
            max_messages: 32,
            result_index: self.selected_result,
            file_index: 0,
            transfer_token: self.transfer_token,
            output_path: output_path.clone(),
            peer_addr_override: None,
            peer_lookup_timeout: Duration::from_secs(5),
            connection_type: "P".to_string(),
            skip_connect_probe: false,
        };

        match client.search_select_and_download(&request).await {
            Ok(result) => {
                self.transfers.push(TransferRow {
                    status: "done".to_string(),
                    user: result.selected_username,
                    file_path: result.selected_virtual_path,
                    bytes: result.bytes_written,
                });
                self.log(format!(
                    "download ok: bytes={} output={}",
                    result.bytes_written,
                    result.output_path.display()
                ));
            }
            Err(err) => {
                self.transfers.push(TransferRow {
                    status: "failed".to_string(),
                    user: selected.username,
                    file_path: selected.file_path,
                    bytes: 0,
                });
                self.log(format!("download failed: {err}"));
            }
        }

        self.session_state = client.state();
        self.session = Some(client);
    }

    fn move_selection(&mut self, delta: isize) {
        if self.results.is_empty() {
            self.selected_result = 0;
            return;
        }
        let len = self.results.len() as isize;
        let current = self.selected_result as isize;
        let next = (current + delta).clamp(0, len - 1);
        self.selected_result = next as usize;
    }
}

fn draw(frame: &mut ratatui::Frame<'_>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Length(10),
            Constraint::Length(8),
            Constraint::Min(8),
        ])
        .split(frame.area());

    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("NeoSoulSeek TUI v1", Style::default().fg(Color::Cyan)),
            Span::raw("  |  "),
            Span::raw(format!("state={:?}", app.session_state)),
            Span::raw("  |  "),
            Span::raw(format!("upload-decision={}", app.upload_decision.as_str())),
        ]),
        Line::from(format!("server={} user={}", app.server, app.username)),
        Line::from(format!(
            "query='{}' transfer_token={} output_dir={} ",
            app.query,
            app.transfer_token,
            app.output_dir.display()
        )),
        Line::from("keys: l=login s=search d=download a=upload-accept x=upload-deny up/down=select q=quit"),
    ])
    .block(Block::default().borders(Borders::ALL).title("Session"));
    frame.render_widget(header, chunks[0]);

    let result_items: Vec<ListItem> = if app.results.is_empty() {
        vec![ListItem::new("no search rows")]
    } else {
        app.results
            .iter()
            .enumerate()
            .map(|(idx, row)| {
                let mut text = format!("{} | {} | {} bytes", row.username, row.file_path, row.file_size);
                if idx == app.selected_result {
                    text = format!("> {text}");
                }
                ListItem::new(text)
            })
            .collect()
    };
    let results = List::new(result_items)
        .block(Block::default().borders(Borders::ALL).title("Search Results"));
    frame.render_widget(results, chunks[1]);

    let transfer_items: Vec<ListItem> = if app.transfers.is_empty() {
        vec![ListItem::new("no transfer attempts")]
    } else {
        app.transfers
            .iter()
            .rev()
            .take(6)
            .map(|row| {
                ListItem::new(format!(
                    "{} | user={} | {} | bytes={}",
                    row.status, row.user, row.file_path, row.bytes
                ))
            })
            .collect()
    };
    let transfers = List::new(transfer_items)
        .block(Block::default().borders(Borders::ALL).title("Transfer Monitor"));
    frame.render_widget(transfers, chunks[2]);

    let log_items: Vec<ListItem> = app
        .logs
        .iter()
        .rev()
        .take(18)
        .map(|line| ListItem::new(line.clone()))
        .collect();
    let logs = List::new(log_items)
        .block(Block::default().borders(Borders::ALL).title("Logs"));
    frame.render_widget(logs, chunks[3]);
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::from_env();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = async {
        loop {
            terminal.draw(|frame| draw(frame, &app))?;

            if !event::poll(Duration::from_millis(150))? {
                continue;
            }

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('l') => app.login().await,
                    KeyCode::Char('s') => app.search().await,
                    KeyCode::Char('d') => app.download_selected().await,
                    KeyCode::Char('a') => {
                        app.upload_decision = UploadDecision::Accept;
                        app.log("upload decision set to accept");
                    }
                    KeyCode::Char('x') => {
                        app.upload_decision = UploadDecision::Deny;
                        app.log("upload decision set to deny");
                    }
                    KeyCode::Up => app.move_selection(-1),
                    KeyCode::Down => app.move_selection(1),
                    _ => {}
                }
            }
        }
        Ok::<(), anyhow::Error>(())
    }
    .await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upload_decision_label_is_stable() {
        assert_eq!(UploadDecision::Accept.as_str(), "accept");
        assert_eq!(UploadDecision::Deny.as_str(), "deny");
    }

    #[test]
    fn move_selection_clamps_bounds() {
        let mut app = App::from_env();
        app.results = vec![
            SearchRow {
                username: "u1".to_string(),
                file_path: "a.mp3".to_string(),
                file_size: 1,
            },
            SearchRow {
                username: "u2".to_string(),
                file_path: "b.mp3".to_string(),
                file_size: 2,
            },
        ];

        app.move_selection(1);
        assert_eq!(app.selected_result, 1);
        app.move_selection(1);
        assert_eq!(app.selected_result, 1);
        app.move_selection(-1);
        assert_eq!(app.selected_result, 0);
        app.move_selection(-1);
        assert_eq!(app.selected_result, 0);
    }
}
