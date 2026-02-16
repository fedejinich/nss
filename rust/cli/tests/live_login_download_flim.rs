use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn required_env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("missing required env var: {name}"))
}

fn optional_env(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.to_owned())
}

fn parse_env_usize(name: &str, default: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(default)
}

fn run_cli(args: &[String]) -> Output {
    if let Ok(bin) = std::env::var("CARGO_BIN_EXE_soul-cli") {
        return Command::new(bin)
            .args(args)
            .output()
            .unwrap_or_else(|err| panic!("failed to run soul-cli with args {args:?}: {err}"));
    }

    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("cli crate should have workspace parent")
        .to_path_buf();
    Command::new("cargo")
        .current_dir(workspace_root)
        .arg("run")
        .arg("-q")
        .arg("-p")
        .arg("soul-cli")
        .arg("--")
        .args(args)
        .output()
        .unwrap_or_else(|err| {
            panic!("failed to run cargo soul-cli fallback with args {args:?}: {err}")
        })
}

#[test]
#[ignore = "requires NSS_LIVE_TEST=1, network access, and valid Soulseek credentials"]
fn live_login_and_download_aphex_twin_flim() {
    let live_flag = optional_env("NSS_LIVE_TEST", "");
    assert_eq!(live_flag, "1", "set NSS_LIVE_TEST=1 to run this live test");

    let server = optional_env("NSS_TEST_SERVER", "server.slsknet.org:2416");
    let username = required_env("NSS_TEST_USERNAME");
    let password = required_env("NSS_TEST_PASSWORD");
    let query = optional_env("NSS_TEST_QUERY", "aphex twin flim");
    let search_mode = optional_env("NSS_TEST_SEARCH_MODE", "distributed");
    let strict_track = std::env::var("NSS_TEST_STRICT_TRACK").ok();
    let result_index = parse_env_usize("NSS_TEST_RESULT_INDEX", 0);
    let file_index = parse_env_usize("NSS_TEST_FILE_INDEX", 0);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_secs();
    let token = 900_000_u32.saturating_add((now % 10_000) as u32);
    let transfer_token = token.saturating_add(1);

    let output_path = if let Ok(raw) = std::env::var("NSS_TEST_OUTPUT_PATH") {
        PathBuf::from(raw)
    } else {
        std::env::temp_dir().join(format!("neosoulseek-flim-{now}.bin"))
    };
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).unwrap_or_else(|err| {
            panic!(
                "failed to create output directory {}: {err}",
                parent.display()
            )
        });
    }
    let output_path_str = output_path.to_string_lossy().to_string();

    let login_args = vec![
        "session".to_owned(),
        "login".to_owned(),
        "--server".to_owned(),
        server.clone(),
        "--username".to_owned(),
        username.clone(),
        "--password".to_owned(),
        password.clone(),
        "--client-version".to_owned(),
        "160".to_owned(),
        "--minor-version".to_owned(),
        "1".to_owned(),
    ];
    let login = run_cli(&login_args);
    assert!(
        login.status.success(),
        "login command failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&login.stdout),
        String::from_utf8_lossy(&login.stderr)
    );

    let mut download_args = vec![
        "session".to_owned(),
        "download-auto".to_owned(),
        "--server".to_owned(),
        server,
        "--username".to_owned(),
        username,
        "--password".to_owned(),
        password,
        "--token".to_owned(),
        token.to_string(),
        "--query".to_owned(),
        query,
        "--output".to_owned(),
        output_path_str.clone(),
        "--transfer-token".to_owned(),
        transfer_token.to_string(),
        "--result-index".to_owned(),
        result_index.to_string(),
        "--file-index".to_owned(),
        file_index.to_string(),
        "--search-timeout-secs".to_owned(),
        "12".to_owned(),
        "--max-messages".to_owned(),
        "64".to_owned(),
        "--search-mode".to_owned(),
        search_mode,
        "--peer-lookup-timeout-secs".to_owned(),
        "8".to_owned(),
        "--client-version".to_owned(),
        "160".to_owned(),
        "--minor-version".to_owned(),
        "1".to_owned(),
        "--verbose".to_owned(),
    ];
    if let Some(strict_track) = strict_track {
        download_args.push("--strict-track".to_owned());
        download_args.push(strict_track);
    }
    let download = run_cli(&download_args);
    assert!(
        download.status.success(),
        "download-auto command failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&download.stdout),
        String::from_utf8_lossy(&download.stderr)
    );

    let metadata = fs::metadata(&output_path).unwrap_or_else(|err| {
        panic!(
            "expected downloaded file at {}: {err}",
            output_path.display()
        )
    });
    assert!(
        metadata.len() > 0,
        "downloaded file is empty: {}",
        output_path.display()
    );

    println!(
        "live login+download ok output={} bytes={}",
        output_path.display(),
        metadata.len()
    );
}
