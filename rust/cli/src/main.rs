use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use protocol::{
    build_file_search_request, build_login_request, build_transfer_request, build_transfer_response, Frame,
    TransferDirection,
};
use soul_core::{
    download_single_file, Credentials, DownloadPlan, ManualUploadDecision, SessionClient, UploadAgent,
    UploadDecisionKind,
};
use std::path::PathBuf;
use std::time::Duration;
use verify::{
    compare_capture_run, compare_fixture_to_frame, write_capture_report, write_report, CaptureRunReport,
    FrameComparison,
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
        password_md5: String,
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
    RunLogin {
        #[arg(long)]
        server: String,
        #[arg(long)]
        username: String,
        #[arg(long)]
        password_md5: String,
        #[arg(long, default_value_t = 157)]
        client_version: u32,
        #[arg(long, default_value_t = 19)]
        minor_version: u32,
    },
    RunSearch {
        #[arg(long)]
        server: String,
        #[arg(long)]
        username: String,
        #[arg(long)]
        password_md5: String,
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
    Verify {
        #[command(subcommand)]
        command: VerifyCommand,
    },
}

#[derive(Debug, Subcommand)]
enum SessionCommand {
    Login {
        #[arg(long)]
        server: String,
        #[arg(long)]
        username: String,
        #[arg(long)]
        password_md5: String,
        #[arg(long, default_value_t = 157)]
        client_version: u32,
        #[arg(long, default_value_t = 19)]
        minor_version: u32,
    },
    Search {
        #[arg(long)]
        server: String,
        #[arg(long)]
        username: String,
        #[arg(long)]
        password_md5: String,
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
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::BuildLogin {
            username,
            password_md5,
            client_version,
            minor_version,
        } => {
            let frame = build_login_request(&username, &password_md5, client_version, minor_version);
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
        Commands::RunLogin {
            server,
            username,
            password_md5,
            client_version,
            minor_version,
        } => run_login(&server, &username, &password_md5, client_version, minor_version).await?,
        Commands::RunSearch {
            server,
            username,
            password_md5,
            token,
            query,
            client_version,
            minor_version,
        } => {
            run_search(
                &server,
                &username,
                &password_md5,
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
        Commands::VerifyFixtures { fixtures_dir, report } => {
            run_verify_fixtures(&fixtures_dir, &report)?;
        }
        Commands::Session { command } => match command {
            SessionCommand::Login {
                server,
                username,
                password_md5,
                client_version,
                minor_version,
            } => run_login(&server, &username, &password_md5, client_version, minor_version).await?,
            SessionCommand::Search {
                server,
                username,
                password_md5,
                token,
                query,
                timeout_secs,
                max_messages,
                client_version,
                minor_version,
            } => {
                run_search(
                    &server,
                    &username,
                    &password_md5,
                    token,
                    &query,
                    client_version,
                    minor_version,
                    timeout_secs,
                    max_messages,
                )
                .await?
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
        Commands::Verify { command } => match command {
            VerifyCommand::Fixtures { fixtures_dir, report } => {
                run_verify_fixtures(&fixtures_dir, &report)?;
            }
            VerifyCommand::Captures { run, base_dir } => {
                run_verify_capture_run(&run, &base_dir)?;
            }
        },
    }

    Ok(())
}

async fn run_login(
    server: &str,
    username: &str,
    password_md5: &str,
    client_version: u32,
    minor_version: u32,
) -> Result<()> {
    let mut client = SessionClient::connect(server).await?;
    client
        .login(&Credentials {
            username: username.to_owned(),
            password_md5: password_md5.to_owned(),
            client_version,
            minor_version,
        })
        .await?;
    println!("session.login ok state={:?} server={}", client.state(), server);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn run_search(
    server: &str,
    username: &str,
    password_md5: &str,
    token: u32,
    query: &str,
    client_version: u32,
    minor_version: u32,
    timeout_secs: u64,
    max_messages: usize,
) -> Result<()> {
    let mut client = SessionClient::connect(server).await?;
    client
        .login(&Credentials {
            username: username.to_owned(),
            password_md5: password_md5.to_owned(),
            client_version,
            minor_version,
        })
        .await?;

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

async fn run_download(peer: String, token: u32, path: String, size: u64, output: PathBuf) -> Result<()> {
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
        .serve_single_manual(
            ManualUploadDecision {
                decision,
                reason,
            },
            source_file,
        )
        .await?;

    println!(
        "transfer.serve-upload handled peer={} decision={:?} bytes_sent={}",
        result.peer_addr, result.decision, result.bytes_sent
    );
    Ok(())
}

fn run_verify_fixtures(fixtures_dir: &PathBuf, report: &PathBuf) -> Result<()> {
    let comparisons = verify_fixtures(fixtures_dir)?;
    write_report(report, &comparisons).with_context(|| format!("write report: {}", report.display()))?;

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

fn run_verify_capture_run(run: &str, base_dir: &PathBuf) -> Result<()> {
    let run_dir = base_dir.join(run);
    let report_path = run_dir.join("verify-captures-report.json");

    let report = compare_capture_run(&run_dir)
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
        "run={} pairs={} matched={} mismatched={} official_only={} neo_only={}",
        report.run_id,
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

    let login = build_login_request("alice", "0123456789abcdef0123456789abcdef", 157, 19);
    results.push(compare_fixture_to_frame(&login_fixture, &login)?);

    let search = build_file_search_request(12345, "aphex twin");
    results.push(compare_fixture_to_frame(&search_fixture, &search)?);

    let transfer_req = build_transfer_request(
        TransferDirection::Download,
        555,
        "Music\\Aphex Twin\\Track.flac",
        123_456_789,
    );
    results.push(compare_fixture_to_frame(&transfer_req_fixture, &transfer_req)?);

    let transfer_resp = build_transfer_response(555, true, "");
    results.push(compare_fixture_to_frame(&transfer_resp_fixture, &transfer_resp)?);

    Ok(results)
}

fn hex_string(frame: &Frame) -> String {
    hex::encode(frame.encode())
}
