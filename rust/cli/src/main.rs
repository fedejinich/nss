use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand, ValueEnum};
use protocol::{
    Frame, TransferDirection, build_file_search_request, build_get_shared_files_in_folder_request,
    build_login_request, build_transfer_request, build_transfer_response,
};
use soul_core::{
    Credentials, DownloadPlan, ManualUploadDecision, PrivateEvent, RoomEvent, SessionClient,
    UploadAgent, UploadDecisionKind, download_single_file, probe_login_versions,
};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use verify::{
    CaptureRunReport, ComparisonMode, FrameComparison, compare_capture_run_with_mode,
    compare_fixture_to_frame, write_capture_report, write_report,
};

#[derive(Debug, Parser)]
#[command(name = "soul-cli")]
#[command(about = "NeoSoulSeek protocol SDK/CLI", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    BuildLogin {
        #[arg(long)]
        username: String,
        #[arg(long)]
        password: String,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long, default_value_t = 157)]
        client_version: u32,
        #[arg(long, default_value_t = 19)]
        minor_version: u32,
    },
    BuildSearch {
        #[arg(long)]
        token: u32,
        #[arg(long)]
        query: String,
    },
    BuildTransferRequest {
        #[arg(long)]
        token: u32,
        #[arg(long)]
        path: String,
        #[arg(long)]
        size: u64,
    },
    BuildTransferResponse {
        #[arg(long)]
        token: u32,
        #[arg(long, default_value_t = true)]
        allowed: bool,
        #[arg(long, default_value = "")]
        queue_or_reason: String,
    },
    BuildPeerFolderRequest {
        #[arg(long)]
        directory: String,
    },
    RunLogin {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long, default_value_t = 157)]
        client_version: u32,
        #[arg(long, default_value_t = 19)]
        minor_version: u32,
    },
    RunSearch {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        token: u32,
        #[arg(long)]
        query: String,
        #[arg(long, default_value_t = 157)]
        client_version: u32,
        #[arg(long, default_value_t = 19)]
        minor_version: u32,
    },
    Download {
        #[arg(long)]
        peer: String,
        #[arg(long)]
        token: u32,
        #[arg(long)]
        path: String,
        #[arg(long)]
        size: u64,
        #[arg(long)]
        output: PathBuf,
    },
    VerifyFixtures {
        #[arg(long, default_value = "captures/fixtures")]
        fixtures_dir: PathBuf,
        #[arg(long, default_value = "captures/fixtures/verify-report.json")]
        report: PathBuf,
    },
    Session {
        #[command(subcommand)]
        command: SessionCommand,
    },
    Transfer {
        #[command(subcommand)]
        command: TransferCommand,
    },
    Room {
        #[command(subcommand)]
        command: RoomCommand,
    },
    Discover {
        #[command(subcommand)]
        command: DiscoverCommand,
    },
    Verify {
        #[command(subcommand)]
        command: VerifyCommand,
    },
}

#[derive(Debug, Subcommand)]
enum SessionCommand {
    Login {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long, default_value_t = 157)]
        client_version: u32,
        #[arg(long, default_value_t = 19)]
        minor_version: u32,
    },
    Search {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        token: u32,
        #[arg(long)]
        query: String,
        #[arg(long, default_value_t = 5)]
        timeout_secs: u64,
        #[arg(long, default_value_t = 10)]
        max_messages: usize,
        #[arg(long, default_value_t = 157)]
        client_version: u32,
        #[arg(long, default_value_t = 19)]
        minor_version: u32,
    },
    Message {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        target_user: String,
        #[arg(long)]
        message: String,
        #[arg(long, default_value_t = false)]
        wait_ack: bool,
        #[arg(long, default_value_t = 5)]
        timeout_secs: u64,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long)]
        verbose: bool,
    },
    MessageUsers {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long, value_delimiter = ',')]
        targets: Vec<String>,
        #[arg(long)]
        message: String,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
    },
    Status {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        target_user: String,
        #[arg(long, default_value_t = 5)]
        timeout_secs: u64,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long)]
        verbose: bool,
    },
    Stats {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        target_user: String,
        #[arg(long, default_value_t = 5)]
        timeout_secs: u64,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long)]
        verbose: bool,
    },
    PeerAddress {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        target_user: String,
        #[arg(long, default_value_t = 5)]
        timeout_secs: u64,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long)]
        verbose: bool,
    },
    ConnectPeer {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        target_user: String,
        #[arg(long)]
        token: u32,
        #[arg(long, default_value = "P")]
        connection_type: String,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
    },
    WatchPrivate {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long, default_value_t = 15)]
        timeout_secs: u64,
        #[arg(long, default_value_t = 128)]
        max_events: usize,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long)]
        verbose: bool,
    },
    IgnoreUser {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        target_user: String,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
    },
    UnignoreUser {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        target_user: String,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
    },
    BanUser {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        target_user: String,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
    },
    PrivilegedList {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long, default_value_t = 5)]
        timeout_secs: u64,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long)]
        verbose: bool,
    },
    OwnPrivileges {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long, default_value_t = 5)]
        timeout_secs: u64,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long)]
        verbose: bool,
    },
    UserPrivileges {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        target_user: String,
        #[arg(long, default_value_t = 5)]
        timeout_secs: u64,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long)]
        verbose: bool,
    },
    UploadSpeed {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        bytes_per_sec: u32,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
    },
    GivePrivilege {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        target_user: String,
        #[arg(long)]
        days: u32,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
    },
    InformPrivileges {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        token: u32,
        #[arg(long)]
        target_user: String,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
    },
    InformPrivilegesAck {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        token: u32,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
    },
    ProbeLoginVersion {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
enum TransferCommand {
    Download {
        #[arg(long)]
        peer: String,
        #[arg(long)]
        token: u32,
        #[arg(long)]
        path: String,
        #[arg(long)]
        size: u64,
        #[arg(long)]
        output: PathBuf,
    },
    ServeUpload {
        #[arg(long, default_value = "127.0.0.1:2242")]
        bind: String,
        #[arg(long)]
        manual: bool,
        #[arg(long, value_enum, default_value_t = ManualDecisionArg::Deny)]
        decision: ManualDecisionArg,
        #[arg(long, default_value = "")]
        reason: String,
        #[arg(long)]
        source_file: Option<PathBuf>,
    },
}

#[derive(Debug, Subcommand)]
enum RoomCommand {
    List {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long)]
        verbose: bool,
    },
    Join {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        room: String,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long)]
        verbose: bool,
    },
    Leave {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        room: String,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long)]
        verbose: bool,
    },
    Add {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        room: String,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long)]
        verbose: bool,
    },
    Members {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        room: String,
        #[arg(long, default_value_t = 6)]
        timeout_secs: u64,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long)]
        verbose: bool,
    },
    Ticker {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        room: String,
        #[arg(long, default_value_t = 6)]
        timeout_secs: u64,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long)]
        verbose: bool,
    },
    Watch {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        room: String,
        #[arg(long, default_value_t = 15)]
        timeout_secs: u64,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long)]
        verbose: bool,
    },
    AddMember {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        room: String,
        #[arg(long)]
        target_user: String,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long)]
        verbose: bool,
    },
    RemoveMember {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        room: String,
        #[arg(long)]
        target_user: String,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long)]
        verbose: bool,
    },
    AddOperator {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        room: String,
        #[arg(long)]
        target_user: String,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long)]
        verbose: bool,
    },
    RemoveOperator {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        room: String,
        #[arg(long)]
        target_user: String,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long)]
        verbose: bool,
    },
}

#[derive(Debug, Subcommand)]
enum DiscoverCommand {
    Recommendations {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long, default_value_t = 5)]
        timeout_secs: u64,
        #[arg(long)]
        verbose: bool,
    },
    Global {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long, default_value_t = 5)]
        timeout_secs: u64,
        #[arg(long)]
        verbose: bool,
    },
    Mine {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long, default_value_t = 2)]
        timeout_secs: u64,
        #[arg(long)]
        verbose: bool,
    },
    User {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        target_user: String,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long, default_value_t = 5)]
        timeout_secs: u64,
        #[arg(long)]
        verbose: bool,
    },
    SimilarTerms {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        term: String,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long, default_value_t = 5)]
        timeout_secs: u64,
        #[arg(long)]
        verbose: bool,
    },
    AddLikeTerm {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        term: String,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long)]
        verbose: bool,
    },
    RemoveLikeTerm {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        term: String,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long)]
        verbose: bool,
    },
    RecommendedUsers {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long, default_value_t = 5)]
        timeout_secs: u64,
        #[arg(long)]
        verbose: bool,
    },
    TermRecommendations {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        term: String,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long, default_value_t = 5)]
        timeout_secs: u64,
        #[arg(long)]
        verbose: bool,
    },
    RecommendationUsers {
        #[arg(long)]
        server: Option<String>,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long, hide = true)]
        password_md5: Option<String>,
        #[arg(long)]
        term: String,
        #[arg(long, default_value_t = 160)]
        client_version: u32,
        #[arg(long, default_value_t = 1)]
        minor_version: u32,
        #[arg(long, default_value_t = 5)]
        timeout_secs: u64,
        #[arg(long)]
        verbose: bool,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ManualDecisionArg {
    Accept,
    Deny,
}

#[derive(Debug, Subcommand)]
enum VerifyCommand {
    Fixtures {
        #[arg(long, default_value = "captures/fixtures")]
        fixtures_dir: PathBuf,
        #[arg(long, default_value = "captures/fixtures/verify-report.json")]
        report: PathBuf,
    },
    Captures {
        #[arg(long)]
        run: String,
        #[arg(long, default_value = "captures/redacted")]
        base_dir: PathBuf,
        #[arg(long, value_enum, default_value_t = VerifyModeArg::Semantic)]
        mode: VerifyModeArg,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum VerifyModeArg {
    Bytes,
    Semantic,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::BuildLogin {
            username,
            password,
            password_md5,
            client_version,
            minor_version,
        } => {
            reject_password_md5(password_md5.as_deref())?;
            let frame = build_login_request(&username, &password, client_version, minor_version);
            println!("{}", hex_string(&frame));
        }
        Commands::BuildSearch { token, query } => {
            let frame = build_file_search_request(token, &query);
            println!("{}", hex_string(&frame));
        }
        Commands::BuildTransferRequest { token, path, size } => {
            let frame = build_transfer_request(TransferDirection::Download, token, &path, size);
            println!("{}", hex_string(&frame));
        }
        Commands::BuildTransferResponse {
            token,
            allowed,
            queue_or_reason,
        } => {
            let frame = build_transfer_response(token, allowed, &queue_or_reason);
            println!("{}", hex_string(&frame));
        }
        Commands::BuildPeerFolderRequest { directory } => {
            let frame = build_get_shared_files_in_folder_request(&directory);
            println!("{}", hex_string(&frame));
        }
        Commands::RunLogin {
            server,
            username,
            password,
            password_md5,
            client_version,
            minor_version,
        } => {
            run_login(
                runtime_server(server.as_deref())?.as_str(),
                runtime_username(username.as_deref())?.as_str(),
                runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                client_version,
                minor_version,
            )
            .await?
        }
        Commands::RunSearch {
            server,
            username,
            password,
            password_md5,
            token,
            query,
            client_version,
            minor_version,
        } => {
            run_search(
                runtime_server(server.as_deref())?.as_str(),
                runtime_username(username.as_deref())?.as_str(),
                runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                token,
                &query,
                client_version,
                minor_version,
                5,
                10,
            )
            .await?
        }
        Commands::Download {
            peer,
            token,
            path,
            size,
            output,
        } => run_download(peer, token, path, size, output).await?,
        Commands::VerifyFixtures {
            fixtures_dir,
            report,
        } => {
            run_verify_fixtures(&fixtures_dir, &report)?;
        }
        Commands::Session { command } => match command {
            SessionCommand::Login {
                server,
                username,
                password,
                password_md5,
                client_version,
                minor_version,
            } => {
                run_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?
            }
            SessionCommand::Search {
                server,
                username,
                password,
                password_md5,
                token,
                query,
                timeout_secs,
                max_messages,
                client_version,
                minor_version,
            } => {
                run_search(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    token,
                    &query,
                    client_version,
                    minor_version,
                    timeout_secs,
                    max_messages,
                )
                .await?
            }
            SessionCommand::Message {
                server,
                username,
                password,
                password_md5,
                target_user,
                message,
                wait_ack,
                timeout_secs,
                client_version,
                minor_version,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_message(
                    &mut client,
                    &target_user,
                    &message,
                    wait_ack,
                    timeout_secs,
                    verbose,
                )
                .await?;
            }
            SessionCommand::MessageUsers {
                server,
                username,
                password,
                password_md5,
                targets,
                message,
                client_version,
                minor_version,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_message_users(&mut client, &targets, &message).await?;
            }
            SessionCommand::Status {
                server,
                username,
                password,
                password_md5,
                target_user,
                timeout_secs,
                client_version,
                minor_version,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_user_status(&mut client, &target_user, timeout_secs, verbose).await?;
            }
            SessionCommand::Stats {
                server,
                username,
                password,
                password_md5,
                target_user,
                timeout_secs,
                client_version,
                minor_version,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_user_stats(&mut client, &target_user, timeout_secs, verbose).await?;
            }
            SessionCommand::PeerAddress {
                server,
                username,
                password,
                password_md5,
                target_user,
                timeout_secs,
                client_version,
                minor_version,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_peer_address(&mut client, &target_user, timeout_secs, verbose).await?;
            }
            SessionCommand::ConnectPeer {
                server,
                username,
                password,
                password_md5,
                target_user,
                token,
                connection_type,
                client_version,
                minor_version,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_connect_peer(&mut client, &target_user, token, &connection_type).await?;
            }
            SessionCommand::WatchPrivate {
                server,
                username,
                password,
                password_md5,
                timeout_secs,
                max_events,
                client_version,
                minor_version,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_watch_private(&mut client, timeout_secs, max_events, verbose).await?;
            }
            SessionCommand::IgnoreUser {
                server,
                username,
                password,
                password_md5,
                target_user,
                client_version,
                minor_version,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_ignore_user(&mut client, &target_user).await?;
            }
            SessionCommand::UnignoreUser {
                server,
                username,
                password,
                password_md5,
                target_user,
                client_version,
                minor_version,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_unignore_user(&mut client, &target_user).await?;
            }
            SessionCommand::BanUser {
                server,
                username,
                password,
                password_md5,
                target_user,
                client_version,
                minor_version,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_ban_user(&mut client, &target_user).await?;
            }
            SessionCommand::PrivilegedList {
                server,
                username,
                password,
                password_md5,
                timeout_secs,
                client_version,
                minor_version,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_privileged_list(&mut client, timeout_secs, verbose).await?;
            }
            SessionCommand::OwnPrivileges {
                server,
                username,
                password,
                password_md5,
                timeout_secs,
                client_version,
                minor_version,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_own_privileges(&mut client, timeout_secs, verbose).await?;
            }
            SessionCommand::UserPrivileges {
                server,
                username,
                password,
                password_md5,
                target_user,
                timeout_secs,
                client_version,
                minor_version,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_user_privileges(&mut client, &target_user, timeout_secs, verbose).await?;
            }
            SessionCommand::UploadSpeed {
                server,
                username,
                password,
                password_md5,
                bytes_per_sec,
                client_version,
                minor_version,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_upload_speed(&mut client, bytes_per_sec).await?;
            }
            SessionCommand::GivePrivilege {
                server,
                username,
                password,
                password_md5,
                target_user,
                days,
                client_version,
                minor_version,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_give_privilege(&mut client, &target_user, days).await?;
            }
            SessionCommand::InformPrivileges {
                server,
                username,
                password,
                password_md5,
                token,
                target_user,
                client_version,
                minor_version,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_inform_privileges(&mut client, token, &target_user).await?;
            }
            SessionCommand::InformPrivilegesAck {
                server,
                username,
                password,
                password_md5,
                token,
                client_version,
                minor_version,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_inform_privileges_ack(&mut client, token).await?;
            }
            SessionCommand::ProbeLoginVersion {
                server,
                username,
                password,
                password_md5,
            } => {
                run_probe_login_version(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                )
                .await?;
            }
        },
        Commands::Transfer { command } => match command {
            TransferCommand::Download {
                peer,
                token,
                path,
                size,
                output,
            } => run_download(peer, token, path, size, output).await?,
            TransferCommand::ServeUpload {
                bind,
                manual,
                decision,
                reason,
                source_file,
            } => {
                if !manual {
                    bail!("stage2 supports manual upload policy only, use --manual");
                }
                run_serve_upload(&bind, decision, reason, source_file).await?;
            }
        },
        Commands::Room { command } => match command {
            RoomCommand::List {
                server,
                username,
                password,
                password_md5,
                client_version,
                minor_version,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_room_list(&mut client, verbose).await?;
            }
            RoomCommand::Join {
                server,
                username,
                password,
                password_md5,
                room,
                client_version,
                minor_version,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_room_join(&mut client, &room, verbose).await?;
            }
            RoomCommand::Leave {
                server,
                username,
                password,
                password_md5,
                room,
                client_version,
                minor_version,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_room_leave(&mut client, &room, verbose).await?;
            }
            RoomCommand::Add {
                server,
                username,
                password,
                password_md5,
                room,
                client_version,
                minor_version,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_room_add(&mut client, &room, verbose).await?;
            }
            RoomCommand::Members {
                server,
                username,
                password,
                password_md5,
                room,
                timeout_secs,
                client_version,
                minor_version,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_room_members(&mut client, &room, timeout_secs, verbose).await?;
            }
            RoomCommand::Ticker {
                server,
                username,
                password,
                password_md5,
                room,
                timeout_secs,
                client_version,
                minor_version,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_room_ticker(&mut client, &room, timeout_secs, verbose).await?;
            }
            RoomCommand::Watch {
                server,
                username,
                password,
                password_md5,
                room,
                timeout_secs,
                client_version,
                minor_version,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_room_watch(&mut client, &room, timeout_secs, verbose).await?;
            }
            RoomCommand::AddMember {
                server,
                username,
                password,
                password_md5,
                room,
                target_user,
                client_version,
                minor_version,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_room_add_member(&mut client, &room, &target_user, verbose).await?;
            }
            RoomCommand::RemoveMember {
                server,
                username,
                password,
                password_md5,
                room,
                target_user,
                client_version,
                minor_version,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_room_remove_member(&mut client, &room, &target_user, verbose).await?;
            }
            RoomCommand::AddOperator {
                server,
                username,
                password,
                password_md5,
                room,
                target_user,
                client_version,
                minor_version,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_room_add_operator(&mut client, &room, &target_user, verbose).await?;
            }
            RoomCommand::RemoveOperator {
                server,
                username,
                password,
                password_md5,
                room,
                target_user,
                client_version,
                minor_version,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_room_remove_operator(&mut client, &room, &target_user, verbose).await?;
            }
        },
        Commands::Discover { command } => match command {
            DiscoverCommand::Recommendations {
                server,
                username,
                password,
                password_md5,
                client_version,
                minor_version,
                timeout_secs,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_discover_recommendations(&mut client, timeout_secs, verbose).await?;
            }
            DiscoverCommand::Global {
                server,
                username,
                password,
                password_md5,
                client_version,
                minor_version,
                timeout_secs,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_discover_global(&mut client, timeout_secs, verbose).await?;
            }
            DiscoverCommand::Mine {
                server,
                username,
                password,
                password_md5,
                client_version,
                minor_version,
                timeout_secs,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_discover_mine(&mut client, timeout_secs, verbose).await?;
            }
            DiscoverCommand::User {
                server,
                username,
                password,
                password_md5,
                target_user,
                client_version,
                minor_version,
                timeout_secs,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_discover_user(&mut client, &target_user, timeout_secs, verbose).await?;
            }
            DiscoverCommand::SimilarTerms {
                server,
                username,
                password,
                password_md5,
                term,
                client_version,
                minor_version,
                timeout_secs,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_discover_similar_terms(&mut client, &term, timeout_secs, verbose).await?;
            }
            DiscoverCommand::AddLikeTerm {
                server,
                username,
                password,
                password_md5,
                term,
                client_version,
                minor_version,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_discover_add_like_term(&mut client, &term, verbose).await?;
            }
            DiscoverCommand::RemoveLikeTerm {
                server,
                username,
                password,
                password_md5,
                term,
                client_version,
                minor_version,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_discover_remove_like_term(&mut client, &term, verbose).await?;
            }
            DiscoverCommand::RecommendedUsers {
                server,
                username,
                password,
                password_md5,
                client_version,
                minor_version,
                timeout_secs,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_discover_recommended_users(&mut client, timeout_secs, verbose).await?;
            }
            DiscoverCommand::TermRecommendations {
                server,
                username,
                password,
                password_md5,
                term,
                client_version,
                minor_version,
                timeout_secs,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_discover_term_recommendations(&mut client, &term, timeout_secs, verbose)
                    .await?;
            }
            DiscoverCommand::RecommendationUsers {
                server,
                username,
                password,
                password_md5,
                term,
                client_version,
                minor_version,
                timeout_secs,
                verbose,
            } => {
                let mut client = connect_and_login(
                    runtime_server(server.as_deref())?.as_str(),
                    runtime_username(username.as_deref())?.as_str(),
                    runtime_password(password.as_deref(), password_md5.as_deref())?.as_str(),
                    client_version,
                    minor_version,
                )
                .await?;
                run_discover_recommendation_users(&mut client, &term, timeout_secs, verbose)
                    .await?;
            }
        },
        Commands::Verify { command } => match command {
            VerifyCommand::Fixtures {
                fixtures_dir,
                report,
            } => {
                run_verify_fixtures(&fixtures_dir, &report)?;
            }
            VerifyCommand::Captures {
                run,
                base_dir,
                mode,
            } => {
                run_verify_capture_run(&run, &base_dir, to_comparison_mode(mode))?;
            }
        },
    }

    Ok(())
}

fn reject_password_md5(password_md5: Option<&str>) -> Result<()> {
    if password_md5.is_some() {
        bail!("--password-md5 is deprecated for runtime auth; use --password");
    }
    Ok(())
}

fn read_env_local() {
    let manifest_repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(".env.local");
    let candidates = [
        PathBuf::from(".env.local"),
        PathBuf::from("../.env.local"),
        manifest_repo,
    ];

    for path in candidates {
        if !path.exists() {
            continue;
        }
        let Ok(raw) = fs::read_to_string(path) else {
            continue;
        };
        for line in raw.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            let Some((key, value)) = trimmed.split_once('=') else {
                continue;
            };
            let key = key.trim();
            if key.is_empty() || env::var_os(key).is_some() {
                continue;
            }
            let value = value.trim().trim_matches('"').trim_matches('\'');
            // SAFETY: key/value come from trusted local dotenv file and are UTF-8 strings.
            unsafe { env::set_var(key, value) };
        }
        break;
    }
}

fn env_or_arg(arg: Option<&str>, env_key: &str) -> Result<String> {
    if let Some(value) = arg {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }
    read_env_local();
    let value = env::var(env_key).with_context(|| {
        format!(
            "missing runtime credential: provide --{} or set {}",
            env_key_to_arg(env_key),
            env_key
        )
    })?;
    let trimmed = value.trim();
    if trimmed.is_empty() {
        bail!(
            "missing runtime credential: provide --{} or set {}",
            env_key_to_arg(env_key),
            env_key
        );
    }
    Ok(trimmed.to_string())
}

fn env_key_to_arg(env_key: &str) -> &'static str {
    match env_key {
        "NSS_TEST_SERVER" => "server",
        "NSS_TEST_USERNAME" => "username",
        "NSS_TEST_PASSWORD" => "password",
        _ => "value",
    }
}

fn runtime_server(arg: Option<&str>) -> Result<String> {
    env_or_arg(arg, "NSS_TEST_SERVER")
}

fn runtime_username(arg: Option<&str>) -> Result<String> {
    env_or_arg(arg, "NSS_TEST_USERNAME")
}

fn runtime_password(password: Option<&str>, password_md5: Option<&str>) -> Result<String> {
    reject_password_md5(password_md5)?;
    env_or_arg(password, "NSS_TEST_PASSWORD")
}

fn to_comparison_mode(mode: VerifyModeArg) -> ComparisonMode {
    match mode {
        VerifyModeArg::Bytes => ComparisonMode::Bytes,
        VerifyModeArg::Semantic => ComparisonMode::Semantic,
    }
}

async fn connect_and_login(
    server: &str,
    username: &str,
    password: &str,
    client_version: u32,
    minor_version: u32,
) -> Result<SessionClient> {
    let mut client = SessionClient::connect(server).await?;
    client
        .login(&Credentials {
            username: username.to_owned(),
            password: password.to_owned(),
            client_version,
            minor_version,
        })
        .await?;
    Ok(client)
}

async fn run_login(
    server: &str,
    username: &str,
    password: &str,
    client_version: u32,
    minor_version: u32,
) -> Result<()> {
    let client =
        connect_and_login(server, username, password, client_version, minor_version).await?;
    println!(
        "session.login ok state={:?} server={}",
        client.state(),
        server
    );
    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn run_search(
    server: &str,
    username: &str,
    password: &str,
    token: u32,
    query: &str,
    client_version: u32,
    minor_version: u32,
    timeout_secs: u64,
    max_messages: usize,
) -> Result<()> {
    let mut client =
        connect_and_login(server, username, password, client_version, minor_version).await?;

    let messages = client
        .search_and_collect(
            token,
            query,
            Duration::from_secs(timeout_secs),
            max_messages,
        )
        .await?;

    println!(
        "session.search sent token={} query={} collected_server_messages={}",
        token,
        query,
        messages.len()
    );
    for (idx, msg) in messages.iter().enumerate() {
        println!("[{idx}] {:?}", msg);
    }
    Ok(())
}

async fn run_message(
    client: &mut SessionClient,
    target_user: &str,
    message: &str,
    wait_ack: bool,
    timeout_secs: u64,
    verbose: bool,
) -> Result<()> {
    client.send_private_message(target_user, message).await?;
    println!(
        "session.message sent target_user={} message_len={} wait_ack={}",
        target_user,
        message.len(),
        wait_ack
    );

    if wait_ack {
        let ack = client
            .wait_message_ack(Duration::from_secs(timeout_secs))
            .await?;
        println!("session.message acked message_id={}", ack.message_id);
        if verbose {
            println!("{ack:#?}");
        }
    }
    Ok(())
}

async fn run_message_users(
    client: &mut SessionClient,
    targets: &[String],
    message: &str,
) -> Result<()> {
    if targets.is_empty() {
        bail!("session.message-users requires at least one --targets entry");
    }
    client.send_message_users(targets, message).await?;
    println!(
        "session.message-users sent targets={} message_len={}",
        targets.len(),
        message.len()
    );
    Ok(())
}

async fn run_user_status(
    client: &mut SessionClient,
    target_user: &str,
    timeout_secs: u64,
    verbose: bool,
) -> Result<()> {
    let payload = client
        .get_user_status(target_user, Duration::from_secs(timeout_secs))
        .await?;
    println!(
        "session.status ok target_user={} status={} privileged={}",
        payload.username, payload.status, payload.privileged
    );
    if verbose {
        println!("{payload:#?}");
    }
    Ok(())
}

async fn run_user_stats(
    client: &mut SessionClient,
    target_user: &str,
    timeout_secs: u64,
    verbose: bool,
) -> Result<()> {
    let payload = client
        .get_user_stats(target_user, Duration::from_secs(timeout_secs))
        .await?;
    println!(
        "session.stats ok target_user={} avg_speed={} download_num={} files={} dirs={}",
        payload.username, payload.avg_speed, payload.download_num, payload.files, payload.dirs
    );
    if verbose {
        println!("{payload:#?}");
    }
    Ok(())
}

async fn run_peer_address(
    client: &mut SessionClient,
    target_user: &str,
    timeout_secs: u64,
    verbose: bool,
) -> Result<()> {
    let payload = client
        .get_peer_address(target_user, Duration::from_secs(timeout_secs))
        .await?;
    println!(
        "session.peer-address ok target_user={} ip={} port={} obfuscation_type={} obfuscated_port={}",
        payload.username,
        payload.ip_address,
        payload.port,
        payload.obfuscation_type,
        payload.obfuscated_port
    );
    if verbose {
        println!("{payload:#?}");
    }
    Ok(())
}

async fn run_connect_peer(
    client: &mut SessionClient,
    target_user: &str,
    token: u32,
    connection_type: &str,
) -> Result<()> {
    client
        .connect_to_peer(target_user, token, connection_type)
        .await?;
    println!(
        "session.connect-peer sent target_user={} token={} connection_type={}",
        target_user, token, connection_type
    );
    Ok(())
}

async fn run_watch_private(
    client: &mut SessionClient,
    timeout_secs: u64,
    max_events: usize,
    verbose: bool,
) -> Result<()> {
    let events = client
        .collect_private_events(Duration::from_secs(timeout_secs), max_events)
        .await?;
    let messages = events
        .iter()
        .filter(|event| matches!(event, PrivateEvent::Message(_)))
        .count();
    let acks = events
        .iter()
        .filter(|event| matches!(event, PrivateEvent::Ack(_)))
        .count();
    println!(
        "session.watch-private ok timeout_secs={} messages={} acks={} total_events={}",
        timeout_secs,
        messages,
        acks,
        events.len()
    );
    if verbose {
        for (idx, event) in events.iter().enumerate() {
            match event {
                PrivateEvent::Message(payload) => {
                    println!(
                        "[{idx}] message id={} user={} is_new={} message_len={}",
                        payload.message_id,
                        payload.username,
                        payload.is_new,
                        payload.message.len()
                    );
                }
                PrivateEvent::Ack(payload) => {
                    println!("[{idx}] ack id={}", payload.message_id);
                }
            }
        }
    }
    Ok(())
}

async fn run_ignore_user(client: &mut SessionClient, target_user: &str) -> Result<()> {
    client.ignore_user(target_user).await?;
    println!("session.ignore-user sent target_user={}", target_user);
    Ok(())
}

async fn run_unignore_user(client: &mut SessionClient, target_user: &str) -> Result<()> {
    client.unignore_user(target_user).await?;
    println!("session.unignore-user sent target_user={}", target_user);
    Ok(())
}

async fn run_ban_user(client: &mut SessionClient, target_user: &str) -> Result<()> {
    client.ban_user(target_user).await?;
    println!("session.ban-user sent target_user={}", target_user);
    Ok(())
}

async fn run_privileged_list(
    client: &mut SessionClient,
    timeout_secs: u64,
    verbose: bool,
) -> Result<()> {
    let payload = client
        .get_privileged_list(Duration::from_secs(timeout_secs))
        .await?;
    let sample = payload.users.iter().take(5).cloned().collect::<Vec<_>>();
    println!(
        "session.privileged-list ok users={} sample={}",
        payload.users.len(),
        sample.join(", ")
    );
    if verbose {
        println!("{payload:#?}");
    }
    Ok(())
}

async fn run_own_privileges(
    client: &mut SessionClient,
    timeout_secs: u64,
    verbose: bool,
) -> Result<()> {
    let payload = client
        .get_own_privileges_status(Duration::from_secs(timeout_secs))
        .await?;
    println!(
        "session.own-privileges ok time_left_seconds={}",
        payload.time_left_seconds
    );
    if verbose {
        println!("{payload:#?}");
    }
    Ok(())
}

async fn run_user_privileges(
    client: &mut SessionClient,
    target_user: &str,
    timeout_secs: u64,
    verbose: bool,
) -> Result<()> {
    let payload = client
        .get_user_privileges_status(target_user, Duration::from_secs(timeout_secs))
        .await?;
    println!(
        "session.user-privileges ok target_user={} privileged={}",
        payload.username, payload.privileged
    );
    if verbose {
        println!("{payload:#?}");
    }
    Ok(())
}

async fn run_upload_speed(client: &mut SessionClient, bytes_per_sec: u32) -> Result<()> {
    client.set_upload_speed(bytes_per_sec).await?;
    println!("session.upload-speed sent bytes_per_sec={}", bytes_per_sec);
    Ok(())
}

async fn run_give_privilege(
    client: &mut SessionClient,
    target_user: &str,
    days: u32,
) -> Result<()> {
    client.give_privilege(target_user, days).await?;
    println!(
        "session.give-privilege sent target_user={} days={}",
        target_user, days
    );
    Ok(())
}

async fn run_inform_privileges(
    client: &mut SessionClient,
    token: u32,
    target_user: &str,
) -> Result<()> {
    client.inform_user_of_privileges(token, target_user).await?;
    println!(
        "session.inform-privileges sent token={} target_user={}",
        token, target_user
    );
    Ok(())
}

async fn run_inform_privileges_ack(client: &mut SessionClient, token: u32) -> Result<()> {
    client.inform_user_of_privileges_ack(token).await?;
    println!("session.inform-privileges-ack sent token={}", token);
    Ok(())
}

async fn run_room_list(client: &mut SessionClient, verbose: bool) -> Result<()> {
    let payload = client.list_rooms(Duration::from_secs(6)).await?;
    println!(
        "room.list ok rooms={} sample={}",
        payload.room_count,
        payload
            .rooms
            .iter()
            .take(5)
            .cloned()
            .collect::<Vec<_>>()
            .join(", ")
    );
    if verbose {
        println!("{payload:#?}");
    }
    Ok(())
}

async fn run_room_join(client: &mut SessionClient, room: &str, verbose: bool) -> Result<()> {
    client.join_room(room).await?;
    let events = client
        .collect_room_events(Duration::from_secs(3), 64)
        .await?;
    println!(
        "room.join ok room={} collected_events={}",
        room,
        events.len()
    );
    if verbose {
        for (idx, event) in events.iter().enumerate() {
            println!("[{idx}] {event:#?}");
        }
    }
    Ok(())
}

async fn run_room_leave(client: &mut SessionClient, room: &str, verbose: bool) -> Result<()> {
    client.join_room(room).await?;
    client.leave_room(room).await?;
    let events = client
        .collect_room_events(Duration::from_secs(2), 32)
        .await?;
    println!(
        "room.leave ok room={} collected_events={}",
        room,
        events.len()
    );
    if verbose {
        for (idx, event) in events.iter().enumerate() {
            println!("[{idx}] {event:#?}");
        }
    }
    Ok(())
}

async fn run_room_add(client: &mut SessionClient, room: &str, verbose: bool) -> Result<()> {
    client.add_chatroom(room).await?;
    println!("room.add sent room={}", room);
    if verbose {
        let events = client
            .collect_room_events(Duration::from_secs(2), 64)
            .await?;
        for (idx, event) in events.iter().enumerate() {
            println!("[{idx}] {event:#?}");
        }
    }
    Ok(())
}

async fn run_room_members(
    client: &mut SessionClient,
    room: &str,
    timeout_secs: u64,
    verbose: bool,
) -> Result<()> {
    client.join_room(room).await?;
    client.request_room_members(room).await?;
    client.request_room_operators(room).await?;
    let events = client
        .collect_room_events(Duration::from_secs(timeout_secs), 256)
        .await?;

    let members = events.iter().find_map(|event| {
        if let RoomEvent::MembersSnapshot(payload) = event {
            Some(payload)
        } else {
            None
        }
    });
    let operators = events.iter().find_map(|event| {
        if let RoomEvent::OperatorsSnapshot(payload) = event {
            Some(payload)
        } else {
            None
        }
    });

    let members_count = members.map(|payload| payload.users.len()).unwrap_or(0);
    let operators_count = operators
        .map(|payload| payload.operators.len())
        .unwrap_or(0);
    println!(
        "room.members ok room={} members={} operators={}",
        room, members_count, operators_count
    );
    if verbose {
        for (idx, event) in events.iter().enumerate() {
            println!("[{idx}] {event:#?}");
        }
    }
    Ok(())
}

async fn run_room_ticker(
    client: &mut SessionClient,
    room: &str,
    timeout_secs: u64,
    verbose: bool,
) -> Result<()> {
    client.join_room(room).await?;
    let payload = client
        .request_room_ticker(room, Duration::from_secs(timeout_secs))
        .await?;
    println!(
        "room.ticker ok room={} entries={} sample={}",
        payload.room,
        payload.entries.len(),
        payload
            .entries
            .iter()
            .take(3)
            .map(|entry| format!("{}:{}", entry.username, entry.ticker))
            .collect::<Vec<_>>()
            .join(", ")
    );
    if verbose {
        println!("{payload:#?}");
    }
    Ok(())
}

async fn run_room_watch(
    client: &mut SessionClient,
    room: &str,
    timeout_secs: u64,
    verbose: bool,
) -> Result<()> {
    client.join_room(room).await?;
    client.request_room_members(room).await?;
    client.request_room_operators(room).await?;
    let events = client
        .collect_room_events(Duration::from_secs(timeout_secs), 512)
        .await?;

    let joined = events
        .iter()
        .filter(|event| matches!(event, RoomEvent::UserJoined { .. }))
        .count();
    let left = events
        .iter()
        .filter(|event| matches!(event, RoomEvent::UserLeft { .. }))
        .count();
    let messages = events
        .iter()
        .filter(|event| matches!(event, RoomEvent::RoomMessage { .. }))
        .count();
    let tickers = events
        .iter()
        .filter(|event| matches!(event, RoomEvent::TickerSnapshot(_)))
        .count();

    println!(
        "room.watch ok room={} timeout_secs={} joined={} left={} messages={} tickers={} total_events={}",
        room,
        timeout_secs,
        joined,
        left,
        messages,
        tickers,
        events.len()
    );
    if verbose {
        for (idx, event) in events.iter().enumerate() {
            println!("[{idx}] {event:#?}");
        }
    }
    Ok(())
}

async fn run_room_add_member(
    client: &mut SessionClient,
    room: &str,
    target_user: &str,
    verbose: bool,
) -> Result<()> {
    client.add_room_member(room, target_user).await?;
    println!("room.add-member sent room={} user={}", room, target_user);
    if verbose {
        let events = client
            .collect_room_events(Duration::from_secs(2), 64)
            .await?;
        for (idx, event) in events.iter().enumerate() {
            println!("[{idx}] {event:#?}");
        }
    }
    Ok(())
}

async fn run_room_remove_member(
    client: &mut SessionClient,
    room: &str,
    target_user: &str,
    verbose: bool,
) -> Result<()> {
    client.remove_room_member(room, target_user).await?;
    println!("room.remove-member sent room={} user={}", room, target_user);
    if verbose {
        let events = client
            .collect_room_events(Duration::from_secs(2), 64)
            .await?;
        for (idx, event) in events.iter().enumerate() {
            println!("[{idx}] {event:#?}");
        }
    }
    Ok(())
}

async fn run_room_add_operator(
    client: &mut SessionClient,
    room: &str,
    target_user: &str,
    verbose: bool,
) -> Result<()> {
    client.add_room_operator(room, target_user).await?;
    println!("room.add-operator sent room={} user={}", room, target_user);
    if verbose {
        let events = client
            .collect_room_events(Duration::from_secs(2), 64)
            .await?;
        for (idx, event) in events.iter().enumerate() {
            println!("[{idx}] {event:#?}");
        }
    }
    Ok(())
}

async fn run_room_remove_operator(
    client: &mut SessionClient,
    room: &str,
    target_user: &str,
    verbose: bool,
) -> Result<()> {
    client.remove_room_operator(room, target_user).await?;
    println!(
        "room.remove-operator sent room={} user={}",
        room, target_user
    );
    if verbose {
        let events = client
            .collect_room_events(Duration::from_secs(2), 64)
            .await?;
        for (idx, event) in events.iter().enumerate() {
            println!("[{idx}] {event:#?}");
        }
    }
    Ok(())
}

fn summarize_recommendation_terms(entries: &[protocol::RecommendationEntry]) -> String {
    entries
        .iter()
        .take(5)
        .map(|entry| format!("{}:{}", entry.term, entry.score))
        .collect::<Vec<_>>()
        .join(", ")
}

fn summarize_scored_users(entries: &[protocol::ScoredUserEntry]) -> String {
    entries
        .iter()
        .take(5)
        .map(|entry| format!("{}:{}", entry.username, entry.score))
        .collect::<Vec<_>>()
        .join(", ")
}

fn print_recommendations_summary(
    label: &str,
    payload: &protocol::RecommendationsPayload,
    verbose: bool,
) {
    println!(
        "discover.{label} ok recommendations={} unrecommendations={} sample={}",
        payload.recommendations.len(),
        payload.unrecommendations.len(),
        summarize_recommendation_terms(&payload.recommendations),
    );
    if verbose {
        println!("{payload:#?}");
    }
}

async fn run_discover_recommendations(
    client: &mut SessionClient,
    timeout_secs: u64,
    verbose: bool,
) -> Result<()> {
    let payload = client
        .get_recommendations(Duration::from_secs(timeout_secs))
        .await?;
    print_recommendations_summary("recommendations", &payload, verbose);
    Ok(())
}

async fn run_discover_global(
    client: &mut SessionClient,
    timeout_secs: u64,
    verbose: bool,
) -> Result<()> {
    let payload = client
        .get_global_recommendations(Duration::from_secs(timeout_secs))
        .await?;
    print_recommendations_summary("global", &payload, verbose);
    Ok(())
}

async fn run_discover_mine(
    client: &mut SessionClient,
    timeout_secs: u64,
    verbose: bool,
) -> Result<()> {
    let payload = client
        .get_my_recommendations(Duration::from_secs(timeout_secs))
        .await?;
    print_recommendations_summary("mine", &payload, verbose);
    Ok(())
}

async fn run_discover_user(
    client: &mut SessionClient,
    target_user: &str,
    timeout_secs: u64,
    verbose: bool,
) -> Result<()> {
    let payload = client
        .get_user_recommendations(target_user, Duration::from_secs(timeout_secs))
        .await?;
    println!(
        "discover.user ok target_user={} recommendations={} unrecommendations={} sample={}",
        payload.username,
        payload.recommendations.recommendations.len(),
        payload.recommendations.unrecommendations.len(),
        summarize_recommendation_terms(&payload.recommendations.recommendations),
    );
    if verbose {
        println!("{payload:#?}");
    }
    Ok(())
}

async fn run_discover_similar_terms(
    client: &mut SessionClient,
    term: &str,
    timeout_secs: u64,
    verbose: bool,
) -> Result<()> {
    let payload = client
        .get_similar_terms(term, Duration::from_secs(timeout_secs))
        .await?;
    println!(
        "discover.similar-terms ok term={} entries={} sample={}",
        payload.term,
        payload.entries.len(),
        summarize_recommendation_terms(&payload.entries),
    );
    if verbose {
        println!("{payload:#?}");
    }
    Ok(())
}

async fn run_discover_add_like_term(
    client: &mut SessionClient,
    term: &str,
    verbose: bool,
) -> Result<()> {
    client.add_like_term(term).await?;
    println!("discover.add-like-term sent term={}", term);
    if verbose {
        println!("discover.add-like-term verbose term_len={}", term.len());
    }
    Ok(())
}

async fn run_discover_remove_like_term(
    client: &mut SessionClient,
    term: &str,
    verbose: bool,
) -> Result<()> {
    client.remove_like_term(term).await?;
    println!("discover.remove-like-term sent term={}", term);
    if verbose {
        println!("discover.remove-like-term verbose term_len={}", term.len());
    }
    Ok(())
}

async fn run_discover_recommended_users(
    client: &mut SessionClient,
    timeout_secs: u64,
    verbose: bool,
) -> Result<()> {
    let payload = client
        .get_recommended_users(Duration::from_secs(timeout_secs))
        .await?;
    println!(
        "discover.recommended-users ok users={} sample={}",
        payload.users.len(),
        summarize_scored_users(&payload.users),
    );
    if verbose {
        println!("{payload:#?}");
    }
    Ok(())
}

async fn run_discover_term_recommendations(
    client: &mut SessionClient,
    term: &str,
    timeout_secs: u64,
    verbose: bool,
) -> Result<()> {
    let payload = client
        .get_term_recommendations(term, Duration::from_secs(timeout_secs))
        .await?;
    println!(
        "discover.term-recommendations ok term={} entries={} sample={}",
        payload.term,
        payload.recommendations.len(),
        summarize_recommendation_terms(&payload.recommendations),
    );
    if verbose {
        println!("{payload:#?}");
    }
    Ok(())
}

async fn run_discover_recommendation_users(
    client: &mut SessionClient,
    term: &str,
    timeout_secs: u64,
    verbose: bool,
) -> Result<()> {
    let payload = client
        .get_recommendation_users(term, Duration::from_secs(timeout_secs))
        .await?;
    println!(
        "discover.recommendation-users ok term={} users={} sample={}",
        payload.term,
        payload.users.len(),
        summarize_scored_users(&payload.users),
    );
    if verbose {
        println!("{payload:#?}");
    }
    Ok(())
}

async fn run_probe_login_version(server: &str, username: &str, password: &str) -> Result<()> {
    let attempts = probe_login_versions(server, username, password).await?;
    let rendered: Vec<serde_json::Value> = attempts
        .into_iter()
        .map(|attempt| {
            serde_json::json!({
                "client_version": attempt.client_version,
                "minor_version": attempt.minor_version,
                "result": attempt.result,
            })
        })
        .collect();
    println!(
        "{}",
        serde_json::to_string_pretty(&rendered).context("serialize probe attempts")?
    );
    Ok(())
}

async fn run_download(
    peer: String,
    token: u32,
    path: String,
    size: u64,
    output: PathBuf,
) -> Result<()> {
    let result = download_single_file(&DownloadPlan {
        peer_addr: peer,
        token,
        virtual_path: path,
        file_size: size,
        output_path: output,
    })
    .await?;
    println!(
        "transfer.download ok bytes={} output={}",
        result.bytes_written,
        result.output_path.display()
    );
    Ok(())
}

async fn run_serve_upload(
    bind: &str,
    decision: ManualDecisionArg,
    reason: String,
    source_file: Option<PathBuf>,
) -> Result<()> {
    let agent = UploadAgent::bind_manual(bind).await?;
    let addr = agent.local_addr()?;

    println!("transfer.serve-upload waiting bind={} policy=manual", addr);
    let decision = match decision {
        ManualDecisionArg::Accept => UploadDecisionKind::Accept,
        ManualDecisionArg::Deny => UploadDecisionKind::Deny,
    };

    let result = agent
        .serve_single_manual(ManualUploadDecision { decision, reason }, source_file)
        .await?;

    println!(
        "transfer.serve-upload handled peer={} decision={:?} bytes_sent={}",
        result.peer_addr, result.decision, result.bytes_sent
    );
    Ok(())
}

fn run_verify_fixtures(fixtures_dir: &PathBuf, report: &PathBuf) -> Result<()> {
    let comparisons = verify_fixtures(fixtures_dir)?;
    write_report(report, &comparisons)
        .with_context(|| format!("write report: {}", report.display()))?;

    for cmp in &comparisons {
        println!(
            "{} matches={} expected_len={} actual_len={} first_diff={:?}",
            cmp.fixture, cmp.matches, cmp.expected_len, cmp.actual_len, cmp.first_diff_offset
        );
    }

    if comparisons.iter().any(|c| !c.matches) {
        bail!("fixture verification failed");
    }

    Ok(())
}

fn run_verify_capture_run(run: &str, base_dir: &PathBuf, mode: ComparisonMode) -> Result<()> {
    let run_dir = base_dir.join(run);
    let report_path = run_dir.join("verify-captures-report.json");

    let report = compare_capture_run_with_mode(&run_dir, mode)
        .with_context(|| format!("compare capture run: {}", run_dir.display()))?;
    write_capture_report(&report_path, &report)?;

    print_capture_report_summary(&report);

    if report.mismatched_pairs != 0 || report.official_only != 0 || report.neo_only != 0 {
        bail!("capture verification failed for run {}", run);
    }

    Ok(())
}

fn print_capture_report_summary(report: &CaptureRunReport) {
    println!(
        "run={} mode={:?} pairs={} matched={} mismatched={} official_only={} neo_only={}",
        report.run_id,
        report.comparison_mode,
        report.total_pairs,
        report.matched_pairs,
        report.mismatched_pairs,
        report.official_only,
        report.neo_only
    );
}

fn verify_fixtures(fixtures_dir: &PathBuf) -> Result<Vec<FrameComparison>> {
    let login_fixture = fixtures_dir.join("server_login_request.hex");
    let search_fixture = fixtures_dir.join("server_file_search_request.hex");
    let transfer_req_fixture = fixtures_dir.join("peer_transfer_request.hex");
    let transfer_resp_fixture = fixtures_dir.join("peer_transfer_response.hex");

    let mut results = Vec::new();

    let login = build_login_request("alice", "secret-pass", 157, 19);
    results.push(compare_fixture_to_frame(&login_fixture, &login)?);

    let search = build_file_search_request(12345, "aphex twin");
    results.push(compare_fixture_to_frame(&search_fixture, &search)?);

    let transfer_req = build_transfer_request(
        TransferDirection::Download,
        555,
        "Music\\Aphex Twin\\Track.flac",
        123_456_789,
    );
    results.push(compare_fixture_to_frame(
        &transfer_req_fixture,
        &transfer_req,
    )?);

    let transfer_resp = build_transfer_response(555, true, "");
    results.push(compare_fixture_to_frame(
        &transfer_resp_fixture,
        &transfer_resp,
    )?);

    Ok(results)
}

fn hex_string(frame: &Frame) -> String {
    hex::encode(frame.encode())
}
