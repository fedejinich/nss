use std::io;
use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap};

use crate::app::{App, LoginField, PendingAction, UiPhase};
use crate::state::PersistedDownloadStatus;

const COLOR_BG: Color = Color::Rgb(20, 18, 26);
const COLOR_TEXT: Color = Color::Rgb(240, 236, 227);
const COLOR_MUTED: Color = Color::Rgb(180, 170, 150);
const COLOR_BORDER: Color = Color::Rgb(58, 44, 32);
const COLOR_ACCENT: Color = Color::Rgb(242, 140, 40);
const COLOR_ACCENT_STRONG: Color = Color::Rgb(255, 179, 71);
const COLOR_SUCCESS: Color = Color::Rgb(111, 207, 151);
const COLOR_ERROR: Color = Color::Rgb(235, 87, 87);

pub async fn run(app: &mut App) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = async {
        loop {
            app.attempt_auto_login_if_needed().await;
            terminal.draw(|frame| draw(frame, app))?;

            if !event::poll(Duration::from_millis(120))? {
                continue;
            }

            if let Event::Key(key) = event::read()? {
                match app.handle_key(key) {
                    PendingAction::None => {}
                    PendingAction::Login => app.login().await,
                    PendingAction::Search => app.search().await,
                    PendingAction::Download => app.download_selected().await,
                    PendingAction::RunDiagnostics => app.run_diagnostics().await,
                    PendingAction::Quit => break,
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

fn draw(frame: &mut ratatui::Frame<'_>, app: &App) {
    frame.render_widget(
        Block::default().style(Style::default().bg(COLOR_BG)),
        frame.area(),
    );

    if app.phase == UiPhase::LoginModal {
        draw_login_modal(frame, app);
    } else {
        draw_main(frame, app);
    }

    if app.diagnostics_visible {
        draw_diagnostics_modal(frame, app);
    }
}

fn draw_login_modal(frame: &mut ratatui::Frame<'_>, app: &App) {
    let popup = centered_rect(66, 56, frame.area());
    frame.render_widget(Clear, popup);

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Min(3),
        ])
        .split(popup);

    let title = Paragraph::new(Line::from(vec![
        Span::styled(
            "NeoSoulSeek",
            Style::default()
                .fg(COLOR_ACCENT_STRONG)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled("Login Required", Style::default().fg(COLOR_TEXT)),
    ]))
    .block(
        Block::default()
            .title("Login")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(COLOR_ACCENT)),
    )
    .style(Style::default().bg(COLOR_BG).fg(COLOR_TEXT));
    frame.render_widget(title, sections[0]);

    frame.render_widget(
        login_field(
            "Server",
            &app.state.server,
            app.active_login_field() == LoginField::Server,
        ),
        sections[1],
    );
    frame.render_widget(
        login_field(
            "Username",
            &app.state.username,
            app.active_login_field() == LoginField::Username,
        ),
        sections[2],
    );
    frame.render_widget(
        login_field(
            "Password",
            &app.password_mask(),
            app.active_login_field() == LoginField::Password,
        ),
        sections[3],
    );

    let help = Paragraph::new(
        "Tab/Shift+Tab focus | Enter login | g diagnostics | Esc clear error | q quit",
    )
    .style(Style::default().fg(COLOR_MUTED))
    .wrap(Wrap { trim: true });
    frame.render_widget(help, sections[4]);

    let error_line = app
        .login_error
        .as_deref()
        .unwrap_or("Use valid credentials to unlock search and downloads.");
    let error_style = if app.login_error.is_some() {
        Style::default()
            .fg(COLOR_ERROR)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(COLOR_MUTED)
    };
    frame.render_widget(Paragraph::new(error_line).style(error_style), sections[5]);

    let log_items: Vec<ListItem> = app
        .logs
        .iter()
        .rev()
        .take(8)
        .map(|line| ListItem::new(line.clone()))
        .collect();
    let logs = List::new(log_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Recent Events")
            .border_style(Style::default().fg(COLOR_BORDER)),
    );
    frame.render_widget(logs, sections[6]);
}

fn draw_main(frame: &mut ratatui::Frame<'_>, app: &App) {
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let last_log = app.logs.last().map_or("Ready.", String::as_str);
    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            "NeoSoulSeek",
            Style::default()
                .fg(COLOR_ACCENT_STRONG)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            format!("state={:?}", app.session_state),
            Style::default().fg(COLOR_TEXT),
        ),
        Span::raw("  "),
        Span::styled(
            if app.downloads_visible() {
                "downloads=visible"
            } else {
                "downloads=hidden"
            },
            Style::default().fg(COLOR_MUTED),
        ),
        Span::raw("  "),
        Span::styled(last_log, Style::default().fg(COLOR_MUTED)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(COLOR_BORDER))
            .title("Session"),
    )
    .style(Style::default().fg(COLOR_TEXT).bg(COLOR_BG));
    frame.render_widget(header, root[0]);

    if app.downloads_visible() {
        let body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(68), Constraint::Percentage(32)])
            .split(root[1]);
        frame.render_widget(results_widget(app), body[0]);
        frame.render_widget(downloads_widget(app), body[1]);
    } else {
        frame.render_widget(results_widget(app), root[1]);
    }

    let query_hint = "keys: /=edit query Enter=search d=download t=toggle downloads c=clear history l=login g=diagnostics q=quit";
    let footer = Paragraph::new(vec![
        Line::from(Span::styled(
            format!("Query: {}", app.query_for_display()),
            Style::default().fg(COLOR_TEXT),
        )),
        Line::from(Span::styled(query_hint, Style::default().fg(COLOR_MUTED))),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Search")
            .border_style(Style::default().fg(COLOR_ACCENT)),
    )
    .style(Style::default().bg(COLOR_BG));
    frame.render_widget(footer, root[2]);
}

fn results_widget(app: &App) -> List<'static> {
    let items: Vec<ListItem> = if app.results.is_empty() {
        vec![ListItem::new("No search results yet.")]
    } else {
        app.results
            .iter()
            .enumerate()
            .map(|(idx, row)| {
                let marker = if idx == app.selected_result { ">" } else { " " };
                let style = if idx == app.selected_result {
                    Style::default()
                        .fg(COLOR_ACCENT)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(COLOR_TEXT)
                };
                let text = format!(
                    "{marker} {} | {} | {} bytes",
                    row.username, row.file_path, row.file_size
                );
                ListItem::new(Line::from(Span::styled(text, style)))
            })
            .collect()
    };

    List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Results")
            .border_style(Style::default().fg(COLOR_BORDER)),
    )
}

fn downloads_widget(app: &App) -> List<'static> {
    let items: Vec<ListItem> = if app.state.downloads.is_empty() {
        vec![ListItem::new("No downloads in history.")]
    } else {
        app.state
            .downloads
            .iter()
            .rev()
            .take(16)
            .map(|entry| {
                let (status_label, status_style) = match entry.status {
                    PersistedDownloadStatus::Done => ("done", Style::default().fg(COLOR_SUCCESS)),
                    PersistedDownloadStatus::Failed => ("failed", Style::default().fg(COLOR_ERROR)),
                    PersistedDownloadStatus::InProgress => {
                        ("in-progress", Style::default().fg(COLOR_ACCENT))
                    }
                    PersistedDownloadStatus::Interrupted => {
                        ("interrupted", Style::default().fg(COLOR_MUTED))
                    }
                };
                ListItem::new(Line::from(vec![
                    Span::styled(
                        format!("[{status_label}] "),
                        status_style.add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!(
                            "{} | {} | {} bytes",
                            entry.username, entry.file_path, entry.bytes
                        ),
                        Style::default().fg(COLOR_TEXT),
                    ),
                ]))
            })
            .collect()
    };

    List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Downloads")
            .border_style(Style::default().fg(COLOR_ACCENT)),
    )
}

fn login_field(label: &str, value: &str, focused: bool) -> Paragraph<'static> {
    let border = if focused {
        COLOR_ACCENT_STRONG
    } else {
        COLOR_BORDER
    };
    let title_style = if focused {
        Style::default()
            .fg(COLOR_ACCENT_STRONG)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(COLOR_MUTED)
    };

    Paragraph::new(value.to_string())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(label.to_string(), title_style))
                .border_style(Style::default().fg(border)),
        )
        .style(Style::default().fg(COLOR_TEXT).bg(COLOR_BG))
}

fn draw_diagnostics_modal(frame: &mut ratatui::Frame<'_>, app: &App) {
    let popup = centered_rect(74, 64, frame.area());
    frame.render_widget(Clear, popup);

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Min(5),
        ])
        .split(popup);

    let title = Paragraph::new(Line::from(vec![
        Span::styled(
            "Connection Diagnostics Wizard",
            Style::default()
                .fg(COLOR_ACCENT_STRONG)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            "login/connectivity checks",
            Style::default().fg(COLOR_MUTED),
        ),
    ]))
    .block(
        Block::default()
            .title("Diagnostics")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(COLOR_ACCENT)),
    )
    .style(Style::default().bg(COLOR_BG).fg(COLOR_TEXT));
    frame.render_widget(title, sections[0]);

    let help = Paragraph::new("r rerun checks | Esc or g close | q quit")
        .style(Style::default().fg(COLOR_MUTED));
    frame.render_widget(help, sections[1]);

    let items: Vec<ListItem> = app
        .diagnostics_lines
        .iter()
        .rev()
        .take(18)
        .rev()
        .map(|line| ListItem::new(line.clone()))
        .collect();
    let body = List::new(items).block(
        Block::default()
            .title("Results")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(COLOR_BORDER)),
    );
    frame.render_widget(body, sections[2]);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1]);

    horizontal[1]
}
