use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand, ValueEnum};
use protocol::{
    Frame, TransferDirection, build_file_search_request, build_login_request,
    build_transfer_request, build_transfer_response,
};
use soul_core::{
    Credentials, DownloadPlan, ManualUploadDecision, SessionClient, UploadAgent,
    UploadDecisionKind, download_single_file, probe_login_versions,
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
    let path = PathBuf::from(".env.local");
    if !path.exists() {
        return;
    }
    let Ok(raw) = fs::read_to_string(path) else {
        return;
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

async fn run_login(
    server: &str,
    username: &str,
    password: &str,
    client_version: u32,
    minor_version: u32,
) -> Result<()> {
    let mut client = SessionClient::connect(server).await?;
    client
        .login(&Credentials {
            username: username.to_owned(),
            password: password.to_owned(),
            client_version,
            minor_version,
        })
        .await?;
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
    let mut client = SessionClient::connect(server).await?;
    client
        .login(&Credentials {
            username: username.to_owned(),
            password: password.to_owned(),
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
