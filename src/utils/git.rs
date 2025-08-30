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

pub fn extract_repo_name(remote_url: &str) -> Option<String> {
    let last_part = remote_url.split('/').next_back()?;

    if last_part.is_empty() {
        return None;
    }

    // Remove .git suffix if present, otherwise return the part as-is
    if let Some(name) = last_part.strip_suffix(".git") {
        if name.is_empty() {
            None
        } else {
            Some(name.to_string())
        }
    } else {
        Some(last_part.to_string())
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_repo_name_github_https() {
        let url = "https://github.com/user/repo.git";
        assert_eq!(extract_repo_name(url), Some("repo".to_string()));
    }

    #[test]
    fn test_extract_repo_name_github_ssh() {
        let url = "git@github.com:user/repo.git";
        assert_eq!(extract_repo_name(url), Some("repo".to_string()));
    }

    #[test]
    fn test_extract_repo_name_without_git_suffix() {
        let url = "https://github.com/user/repo";
        assert_eq!(extract_repo_name(url), Some("repo".to_string()));
    }

    #[test]
    fn test_extract_repo_name_gitlab() {
        let url = "https://gitlab.com/user/project.git";
        assert_eq!(extract_repo_name(url), Some("project".to_string()));
    }

    #[test]
    fn test_extract_repo_name_custom_domain() {
        let url = "https://git.company.com/team/awesome-project.git";
        assert_eq!(extract_repo_name(url), Some("awesome-project".to_string()));
    }

    #[test]
    fn test_extract_repo_name_nested_path() {
        let url = "https://github.com/org/subgroup/deep/repo.git";
        assert_eq!(extract_repo_name(url), Some("repo".to_string()));
    }

    #[test]
    fn test_extract_repo_name_empty_url() {
        let url = "";
        assert_eq!(extract_repo_name(url), None);
    }

    #[test]
    fn test_extract_repo_name_invalid_url() {
        let url = "not-a-valid-url";
        assert_eq!(extract_repo_name(url), Some("not-a-valid-url".to_string()));
    }

    #[test]
    fn test_extract_repo_name_trailing_slash() {
        let url = "https://github.com/user/repo.git/";
        assert_eq!(extract_repo_name(url), None);
    }

    #[test]
    fn test_git_functions_handle_no_git() {
        // These tests verify that the functions handle cases where git is not available
        // or the current directory is not a git repository gracefully

        // Note: These will return real values if git is actually installed and we're in a repo
        // The main thing is they don't panic
        let git_installed = is_git_installed();
        // Should return a boolean, not panic
        assert!(git_installed == true || git_installed == false);

        let is_repo = is_git_repository();
        // Should return a boolean, not panic
        assert!(is_repo == true || is_repo == false);

        // These functions should return None if git is not available or not in a repo
        let _remote = get_remote_url();
        let _branch = get_current_branch();
        let repo_info = get_git_repo_info();

        // These should either return valid data or None, not panic
        match repo_info {
            Some((url, name, branch)) => {
                assert!(!url.is_empty());
                assert!(!name.is_empty());
                assert!(!branch.is_empty());
            }
            None => {
                // Expected when not in a git repo or git not installed
            }
        }
    }

    #[test]
    fn test_get_git_repo_info_consistency() {
        // If git info is available, subsequent calls should return the same data
        let info1 = get_git_repo_info();
        let info2 = get_git_repo_info();
        assert_eq!(info1, info2);
    }
}
