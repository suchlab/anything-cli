use std::process::Command;

fn is_git_installed() -> bool {
    Command::new("git")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn is_git_repository() -> bool {
    Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()
        .map(|output| {
            output.status.success() && String::from_utf8_lossy(&output.stdout).trim() == "true"
        })
        .unwrap_or(false)
}

fn get_remote_url() -> Option<String> {
    Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("remote.origin.url")
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                None
            }
        })
}

fn extract_repo_name(remote_url: &str) -> Option<String> {
    remote_url
        .split('/')
        .last()
        .and_then(|part| part.strip_suffix(".git").map(String::from))
}

fn get_current_branch() -> Option<String> {
    Command::new("git")
        .arg("symbolic-ref")
        .arg("--short")
        .arg("HEAD")
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                None
            }
        })
}

pub fn get_git_repo_info() -> Option<(String, String, String)> {
    if !is_git_installed() {
        return None;
    }

    if !is_git_repository() {
        return None;
    }

    let remote_url = get_remote_url()?;
    let repo_name = extract_repo_name(&remote_url)?;
    let branch_name = get_current_branch()?;

    Some((remote_url, repo_name, branch_name))
}
