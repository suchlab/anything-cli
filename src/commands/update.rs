use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const GITHUB_REPO: &str = "suchlab/anything-cli";
const BINARY_NAME: &str = "anything-cli";

#[derive(Debug)]
pub struct PlatformInfo {
    pub os: String,
    pub arch: String,
    pub download_url: String,
}

fn get_platform_info() -> Result<PlatformInfo, String> {
    let os = env::consts::OS;
    let arch = env::consts::ARCH;

    let (platform_os, platform_arch, url_suffix) = match (os, arch) {
        ("macos", "aarch64") => ("darwin", "arm64", "anything-cli-darwin-arm64.tar.gz"),
        ("macos", "x86_64") => ("darwin", "amd64", "anything-cli-darwin-amd64.tar.gz"),
        ("linux", "x86_64") => ("linux", "amd64", "anything-cli-linux-amd64.tar.gz"),
        ("linux", "aarch64") => return Err("Linux ARM64 is not currently supported".to_string()),
        ("windows", _) => return Err("Windows is not currently supported".to_string()),
        _ => return Err(format!("Unsupported platform: {} {}", os, arch)),
    };

    Ok(PlatformInfo {
        os: platform_os.to_string(),
        arch: platform_arch.to_string(),
        download_url: format!(
            "https://github.com/{}/releases/latest/download/{}",
            GITHUB_REPO, url_suffix
        ),
    })
}

fn get_current_executable_path() -> Result<PathBuf, String> {
    env::current_exe().map_err(|e| format!("Failed to get current executable path: {}", e))
}

fn get_latest_version() -> Result<String, String> {
    // Try to get the latest version from GitHub API
    let client = reqwest::blocking::Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/releases/latest",
        GITHUB_REPO
    );

    let response = client
        .get(&url)
        .header(
            "User-Agent",
            format!("anything-cli/{}", env!("CARGO_PKG_VERSION")),
        )
        .send()
        .map_err(|e| format!("Failed to fetch release info: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("GitHub API returned status: {}", response.status()));
    }

    let json: serde_json::Value = response
        .json()
        .map_err(|e| format!("Failed to parse GitHub API response: {}", e))?;

    let tag_name = json["tag_name"]
        .as_str()
        .ok_or("No tag_name found in release")?;

    // Remove 'v' prefix if present
    let version = tag_name.strip_prefix('v').unwrap_or(tag_name);

    Ok(version.to_string())
}

fn download_and_extract_binary(
    url: &str,
    temp_dir: &Path,
    platform_info: &PlatformInfo,
) -> Result<PathBuf, String> {
    println!("Downloading latest binary from: {}", url);

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header(
            "User-Agent",
            format!("anything-cli/{}", env!("CARGO_PKG_VERSION")),
        )
        .send()
        .map_err(|e| format!("Failed to download binary: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Download failed with status: {}",
            response.status()
        ));
    }

    let bytes = response
        .bytes()
        .map_err(|e| format!("Failed to read download content: {}", e))?;

    // Write tar.gz file to temp directory
    let tar_path = temp_dir.join("binary.tar.gz");
    fs::write(&tar_path, &bytes).map_err(|e| format!("Failed to write temporary file: {}", e))?;

    // Extract the tar.gz file
    let output = Command::new("tar")
        .args(["-xzf", tar_path.to_str().unwrap()])
        .current_dir(temp_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to execute tar command: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to extract archive: {}", stderr));
    }

    // The binary name includes the platform suffix in the archive
    let binary_name = format!(
        "{}-{}-{}",
        BINARY_NAME, platform_info.os, platform_info.arch
    );
    let binary_path = temp_dir.join(&binary_name);

    if !binary_path.exists() {
        // Fallback: try just the binary name without suffix
        let fallback_path = temp_dir.join(BINARY_NAME);
        if fallback_path.exists() {
            return Ok(fallback_path);
        }

        // List directory contents for debugging
        let entries: Vec<String> = fs::read_dir(temp_dir)
            .map_err(|e| format!("Failed to read temp directory: {}", e))?
            .filter_map(|entry| entry.ok().and_then(|e| e.file_name().into_string().ok()))
            .collect();

        return Err(format!(
            "Binary not found in extracted archive. Expected: '{}' or '{}'. Found files: {:?}",
            binary_name, BINARY_NAME, entries
        ));
    }

    Ok(binary_path)
}

fn replace_executable(
    new_binary_path: &Path,
    current_executable_path: &Path,
) -> Result<(), String> {
    // Check if we need sudo
    let parent_dir = current_executable_path
        .parent()
        .ok_or("Failed to get parent directory of executable")?;

    let needs_sudo = !is_writable(parent_dir);

    if needs_sudo {
        println!("Root access required to update the binary. Please enter your password.");

        // Use sudo to replace the binary
        let output = Command::new("sudo")
            .args([
                "cp",
                new_binary_path.to_str().unwrap(),
                current_executable_path.to_str().unwrap(),
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| format!("Failed to execute sudo cp: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to copy binary with sudo: {}", stderr));
        }

        // Make sure it's executable
        let output = Command::new("sudo")
            .args(["chmod", "+x", current_executable_path.to_str().unwrap()])
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| format!("Failed to execute sudo chmod: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to make binary executable: {}", stderr));
        }
    } else {
        // Copy without sudo
        fs::copy(new_binary_path, current_executable_path)
            .map_err(|e| format!("Failed to copy binary: {}", e))?;

        // Make it executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(current_executable_path)
                .map_err(|e| format!("Failed to get file metadata: {}", e))?
                .permissions();
            perms.set_mode(0o755);
            fs::set_permissions(current_executable_path, perms)
                .map_err(|e| format!("Failed to set executable permissions: {}", e))?;
        }
    }

    Ok(())
}

fn is_writable(path: &Path) -> bool {
    // Try to create a temporary file in the directory
    let temp_file = path.join(".write_test");
    let result = fs::File::create(&temp_file).is_ok();
    if result {
        let _ = fs::remove_file(&temp_file);
    }
    result
}

pub fn handle_update(executable_name: &str) {
    println!("Checking for updates...");

    // Get current version
    let current_version = env!("CARGO_PKG_VERSION");
    println!("Current version: {}", current_version);

    // Get platform information
    let platform_info = match get_platform_info() {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // Get latest version
    let latest_version = match get_latest_version() {
        Ok(version) => version,
        Err(e) => {
            eprintln!("Failed to check latest version: {}", e);
            std::process::exit(1);
        }
    };

    println!("Latest version: {}", latest_version);

    // Compare versions
    if current_version == latest_version {
        println!("You already have the latest version!");
        return;
    }

    // Get current executable path
    let current_executable_path = match get_current_executable_path() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // Create temporary directory
    let temp_dir = match tempfile::tempdir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Failed to create temporary directory: {}", e);
            std::process::exit(1);
        }
    };

    // Download and extract new binary
    let new_binary_path = match download_and_extract_binary(
        &platform_info.download_url,
        temp_dir.path(),
        &platform_info,
    ) {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // Replace the current executable
    if let Err(e) = replace_executable(&new_binary_path, &current_executable_path) {
        eprintln!("Failed to update binary: {}", e);
        std::process::exit(1);
    }

    println!("Successfully updated to version {}!", latest_version);
    println!(
        "You can now use the updated version of {}.",
        executable_name
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_platform_info() {
        let result = get_platform_info();
        assert!(result.is_ok());

        let platform = result.unwrap();
        assert!(!platform.os.is_empty());
        assert!(!platform.arch.is_empty());
        assert!(platform.download_url.contains("github.com"));
        assert!(platform.download_url.contains("releases/latest/download"));
    }

    #[test]
    fn test_get_current_executable_path() {
        let result = get_current_executable_path();
        assert!(result.is_ok());

        let path = result.unwrap();
        assert!(path.exists());
    }

    #[test]
    fn test_github_url_format() {
        let platform_info = PlatformInfo {
            os: "darwin".to_string(),
            arch: "arm64".to_string(),
            download_url: format!(
                "https://github.com/{}/releases/latest/download/anything-cli-darwin-arm64.tar.gz",
                GITHUB_REPO
            ),
        };

        assert!(platform_info.download_url.contains("suchlab/anything-cli"));
        assert!(platform_info
            .download_url
            .contains("releases/latest/download"));
        assert!(platform_info.download_url.ends_with(".tar.gz"));
    }
}
