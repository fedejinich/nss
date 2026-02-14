use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use protocol::{
    build_file_search_request, build_login_request, build_transfer_request, build_transfer_response, Frame,
    TransferDirection,
};
use soul_core::{download_single_file, Credentials, DownloadPlan, SoulClient};
use std::path::PathBuf;
use verify::{compare_fixture_to_frame, write_report, FrameComparison};

#[derive(Debug, Parser)]
#[command(name = "soul-cli")]
#[command(about = "Soulseek protocol reconstruction CLI", version)]
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
        } => {
            let mut client = SoulClient::connect(&server).await?;
            client
                .login(&Credentials {
                    username,
                    password_md5,
                    client_version,
                    minor_version,
                })
                .await?;
            println!("login frame sent to {}", server);
        }
        Commands::RunSearch {
            server,
            username,
            password_md5,
            token,
            query,
            client_version,
            minor_version,
        } => {
            let mut client = SoulClient::connect(&server).await?;
            client
                .login(&Credentials {
                    username,
                    password_md5,
                    client_version,
                    minor_version,
                })
                .await?;
            client.search(token, &query).await?;
            println!("search frame sent to {} token={} query={}", server, token, query);
        }
        Commands::Download {
            peer,
            token,
            path,
            size,
            output,
        } => {
            let result = download_single_file(&DownloadPlan {
                peer_addr: peer,
                token,
                virtual_path: path,
                file_size: size,
                output_path: output,
            })
            .await?;
            println!("downloaded {} bytes into {}", result.bytes_written, result.output_path.display());
        }
        Commands::VerifyFixtures { fixtures_dir, report } => {
            let comparisons = verify_fixtures(&fixtures_dir)?;
            write_report(&report, &comparisons).with_context(|| format!("write report: {}", report.display()))?;

            for cmp in &comparisons {
                println!(
                    "{} matches={} expected_len={} actual_len={} first_diff={:?}",
                    cmp.fixture, cmp.matches, cmp.expected_len, cmp.actual_len, cmp.first_diff_offset
                );
            }

            if comparisons.iter().any(|c| !c.matches) {
                bail!("fixture verification failed");
            }
        }
    }

    Ok(())
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
    let bytes = frame.encode();
    hex::encode(bytes)
}
