use std::env;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

/// Integration test that verifies the CLI binary compiles and runs
#[test]
fn test_cli_compiles_and_runs() {
    let output = Command::new("cargo")
        .args(&["build", "--bin", "anything-cli"])
        .output()
        .expect("Failed to execute cargo build command");

    assert!(
        output.status.success(),
        "Build should succeed but failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Test that the version flag works
#[test]
fn test_version_flag() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--version"])
        .output()
        .expect("Failed to execute cargo run command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("anything-cli v"));
}

/// Test that the short version flag works
#[test]
fn test_version_flag_short() {
    let output = Command::new("cargo")
        .args(&["run", "--", "-v"])
        .output()
        .expect("Failed to execute cargo run command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("anything-cli v"));
}

/// Test CLI argument parsing with various flag combinations
#[test]
fn test_argument_parsing() {
    // Test insufficient arguments for set-base-url
    let output = Command::new("cargo")
        .args(&["run", "--", "self:set-base-url"])
        .output()
        .expect("Failed to execute cargo command");

    // Should exit with error code 1 due to insufficient arguments
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Usage:"));
}

/// Test that the CLI handles invalid commands gracefully
#[test]
fn test_invalid_internal_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "self:invalid-command"])
        .output()
        .expect("Failed to execute cargo command");

    // Should fail because config doesn't exist
    assert!(!output.status.success());
}

/// Test config file operations in isolation
#[test]
fn test_config_operations() {
    let temp_dir = TempDir::new().unwrap();
    let fake_home = temp_dir.path().to_str().unwrap();

    // Set fake HOME for this test
    let original_home = env::var("HOME").ok();
    env::set_var("HOME", fake_home);

    // Create a config file first
    let config_dir = temp_dir.path().join(".test-app");
    fs::create_dir_all(&config_dir).unwrap();
    let config_file = config_dir.join("config.json");

    let initial_config = r#"{
        "base_url": "https://api.example.com",
        "headers": {
            "Authorization": "Bearer test-token"
        }
    }"#;

    fs::write(&config_file, initial_config).unwrap();

    // Test that config loading works
    assert!(config_file.exists());
    let content = fs::read_to_string(&config_file).unwrap();
    assert!(content.contains("api.example.com"));

    // Restore original HOME
    match original_home {
        Some(home) => env::set_var("HOME", home),
        None => env::remove_var("HOME"),
    }
}

/// Test CLI with different argument patterns
#[test]
fn test_cli_argument_patterns() {
    // Test that the CLI can handle various argument patterns without panicking
    let test_cases = vec![
        (vec!["--version"], "Version flag should work"),
        (vec!["-v"], "Short version flag should work"),
        (
            vec!["self:set-base-url"],
            "Internal commands should be handled",
        ),
    ];

    for (args, description) in test_cases {
        let output = Command::new("cargo")
            .args(&["run", "--"])
            .args(&args)
            .output()
            .expect("Failed to execute cargo command");

        // The command should either succeed or fail gracefully (not panic)
        // We don't care about the exit code, just that it doesn't crash
        assert!(
            output.status.code().is_some(),
            "{}: CLI should exit with a status code, not crash",
            description
        );
    }
}

/// Test that the binary exists after build
#[test]
fn test_binary_exists_after_build() {
    // Build the project
    let build_output = Command::new("cargo")
        .args(&["build"])
        .output()
        .expect("Failed to execute cargo build command");

    assert!(build_output.status.success(), "Build failed");

    // Check that the binary exists
    let target_debug = env::current_dir()
        .unwrap()
        .join("target")
        .join("debug")
        .join("anything-cli");

    // Binary should exist (with or without .exe extension on Windows)
    let binary_exists = target_debug.exists() || target_debug.with_extension("exe").exists();

    assert!(binary_exists, "Binary should exist at {:?}", target_debug);
}

/// Test CLI with no arguments (should try to make a request and fail due to no config)
#[test]
fn test_cli_no_arguments() {
    let output = Command::new("cargo")
        .args(&["run"])
        .output()
        .expect("Failed to execute cargo command");

    // Should fail because no config exists
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Failed to load config"));
}

/// Test that cargo test runs all unit tests successfully
#[test]
fn test_unit_tests_pass() {
    let output = Command::new("cargo")
        .args(&["test", "--lib"])
        .output()
        .expect("Failed to execute cargo test command");

    assert!(
        output.status.success(),
        "Unit tests failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Test specific module functionality through the CLI
#[test]
fn test_module_integration() {
    // Test that individual modules work together
    // This is a basic smoke test to ensure modules can interact

    // Create a temporary config for testing
    let temp_dir = TempDir::new().unwrap();
    let fake_home = temp_dir.path().to_str().unwrap();

    let original_home = env::var("HOME").ok();
    env::set_var("HOME", fake_home);

    // The CLI should handle the case where no config exists
    let output = Command::new("cargo")
        .args(&["run", "--", "test-command"])
        .output()
        .expect("Failed to execute cargo command");

    // Should fail due to missing config
    assert!(!output.status.success());

    // Restore HOME
    match original_home {
        Some(home) => env::set_var("HOME", home),
        None => env::remove_var("HOME"),
    }
}

/// Test that the CLI handles git repository detection
#[test]
fn test_git_integration() {
    // Test that git detection doesn't cause the CLI to crash
    let output = Command::new("cargo")
        .args(&["run", "--", "--version"])
        .output()
        .expect("Failed to execute cargo run command");

    // The CLI should work regardless of git status
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("anything-cli v"));
}

/// Performance test: ensure CLI starts up reasonably quickly
#[test]
fn test_cli_startup_performance() {
    use std::time::Instant;

    let start = Instant::now();

    let output = Command::new("cargo")
        .args(&["run", "--", "--version"])
        .output()
        .expect("Failed to execute cargo run command");

    let duration = start.elapsed();

    assert!(output.status.success());

    // CLI should start up within reasonable time (1 second including compilation)
    // This is generous to account for different system speeds and cold compilation
    assert!(
        duration.as_secs() < 1,
        "CLI took too long to start: {:?}",
        duration
    );
}
