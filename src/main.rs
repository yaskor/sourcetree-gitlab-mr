#![windows_subsystem = "windows"]

use ini::Ini;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::process::Command;
use urlencoding::encode;

fn log_message(message: &str) {
    let log_file_path = "sourcetree-gitlab-mr.log";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)
        .unwrap();
    writeln!(file, "{}", message).unwrap();
}

fn fatal(message: &str) -> ! {
    log_message(message);
    eprintln!("Error: {}", message);
    std::process::exit(1);
}

fn get_repo_url_from_ini(repo_path: &str) -> String {
    let config_path = format!("{}/.git/config", repo_path);

    let config_contents =
        fs::read_to_string(&config_path).unwrap_or_else(|_| fatal("Failed to read config file"));

    let conf =
        Ini::load_from_str(&config_contents).unwrap_or_else(|_| fatal("Failed to load INI file"));

    let section = conf
        .section(Some("remote \"origin\""))
        .unwrap_or_else(|| fatal("Failed to find remote \"origin\" section"));

    section
        .get("url")
        .unwrap_or_else(|| fatal("Failed to find repository URL"))
        .to_string()
}

fn convert_to_https_url(repo_url: &str) -> String {
    let https_url = if repo_url.starts_with("git@") {
        repo_url.replace("git@", "https://").replace(":", "/")
    } else {
        repo_url.to_string()
    };

    if https_url.ends_with(".git") {
        https_url[..https_url.len() - 4].to_string()
    } else {
        https_url
    }
}

fn main() {
    let repo_path = env::args()
        .nth(1)
        .unwrap_or_else(|| fatal("Please provide $REPO as first argument"));

    let source_branch = env::args()
        .nth(2)
        .unwrap_or_else(|| fatal("Please provide $BRANCH as second argument"));

    let repo_url = get_repo_url_from_ini(&repo_path);

    let https_url = convert_to_https_url(&repo_url);

    let title = source_branch
        .rsplitn(2, '/')
        .next()
        .unwrap_or(&source_branch);

    let open_command = format!(
        "{}/-/merge_requests/new?merge_request[source_branch]={}&merge_request[title]={}",
        https_url,
        encode(&source_branch),
        encode(&title)
    );

    Command::new("powershell")
        .arg(format!("Start-Process \"{}\"", open_command))
        .spawn()
        .unwrap_or_else(|_| fatal("Failed to execute command"));
}
