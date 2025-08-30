use crate::config::loader::load_config;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

pub fn handle_uninstall(executable_name: &str) {
    let (_, config_path) = load_config(executable_name);

    let exe_path = match std::env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to get current executable path: {}", e);
            return;
        }
    };

    println!("This will permanently delete:");

    if config_path.exists() {
        println!("  - Config file: {:?}", config_path);
    }

    println!("  - Executable: {:?}", exe_path);
    print!("Are you sure? (y/N): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_lowercase();

    if input != "y" {
        println!("Exited.");
        return;
    }

    if config_path.exists() {
        if let Err(e) = fs::remove_file(&config_path) {
            eprintln!("Failed to delete config file: {}", e);
        } else {
            println!("Config file deleted.");

            if let Some(parent_dir) = config_path.parent() {
                if fs::remove_dir(parent_dir).is_err() {
                    // Directory not empty or other error - this is fine, we only want to remove if empty
                } else {
                    println!("Config directory deleted.");
                }
            }
        }
    }

    // Delete the executable - Checks for permissions
    if fs::remove_file(&exe_path).is_err() {
        println!("Root access required to delete executable. Asking for sudo...");
        let status = Command::new("sudo")
            .arg("rm")
            .arg("-f")
            .arg(&exe_path)
            .status();

        match status {
            Ok(exit_status) if exit_status.success() => {
                println!("Executable deleted.");
            }
            _ => {
                eprintln!("Failed to delete executable. Please run the command manually:");
                eprintln!("  sudo rm -f {:?}", exe_path);
            }
        }
    } else {
        println!("Executable deleted.");
    }

    std::process::exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::TempDir;

    #[test]
    fn test_get_executable_path() {
        // Test that we can get the current executable path
        let exe_path = std::env::current_exe();
        assert!(exe_path.is_ok());

        let path = exe_path.unwrap();
        assert!(path.exists());
        assert!(path.is_file());
    }

    #[test]
    fn test_input_processing() {
        // Test the logic for processing user confirmation input
        let test_cases = vec![
            ("y", true),
            ("Y", true),
            ("yes", false),
            ("n", false),
            ("N", false),
            ("no", false),
            ("", false),
            ("maybe", false),
            ("anything", false),
        ];

        for (input, expected) in test_cases {
            let processed = input.trim().to_lowercase();
            let result = processed == "y";
            assert_eq!(
                result, expected,
                "Input '{}' should result in {}",
                input, expected
            );
        }
    }

    #[test]
    fn test_config_path_logic() {
        let (_temp_dir, fake_home) = setup_test_environment();

        // Test with a non-existent config
        let original_home = env::var("HOME").ok();
        env::set_var("HOME", &fake_home);

        let (config, config_path) = load_config("test-app");

        // Restore HOME
        match original_home {
            Some(home) => env::set_var("HOME", home),
            None => env::remove_var("HOME"),
        }

        assert!(config.is_none());
        assert!(!config_path.exists());
        assert!(config_path.to_string_lossy().contains("test-app"));
        assert!(config_path.to_string_lossy().ends_with("config.json"));
    }

    #[test]
    fn test_config_file_deletion_logic() {
        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join("config.json");

        // Create a test config file
        fs::write(&config_file, "test content").unwrap();
        assert!(config_file.exists());

        // Test deletion logic
        let deletion_result = fs::remove_file(&config_file);
        assert!(deletion_result.is_ok());
        assert!(!config_file.exists());
    }

    #[test]
    fn test_config_directory_cleanup_logic() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("app-config");
        let config_file = config_dir.join("config.json");

        // Create directory and file
        fs::create_dir_all(&config_dir).unwrap();
        fs::write(&config_file, "test content").unwrap();

        assert!(config_dir.exists());
        assert!(config_file.exists());

        // Remove file first
        fs::remove_file(&config_file).unwrap();
        assert!(!config_file.exists());

        // Try to remove directory (should succeed if empty)
        let dir_removal = fs::remove_dir(&config_dir);

        // The directory should be removable since it's empty
        if dir_removal.is_ok() {
            assert!(!config_dir.exists());
        }
        // If it fails, that's also acceptable (the real function handles this gracefully)
    }

    #[test]
    fn test_executable_deletion_permission_check() {
        // We can't actually test deleting executables in unit tests
        // but we can test the logic around file operations

        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test-executable");

        // Create a test file
        fs::write(&test_file, "test executable").unwrap();
        assert!(test_file.exists());

        // Test successful deletion
        let result = fs::remove_file(&test_file);
        assert!(result.is_ok());
        assert!(!test_file.exists());
    }

    #[test]
    fn test_parent_directory_handling() {
        let temp_dir = TempDir::new().unwrap();
        let nested_dir = temp_dir.path().join("nested").join("config");
        let config_file = nested_dir.join("config.json");

        // Create nested directory structure
        fs::create_dir_all(&nested_dir).unwrap();
        fs::write(&config_file, "test").unwrap();

        // Test parent directory access
        if let Some(parent) = config_file.parent() {
            assert_eq!(parent, nested_dir);
            assert!(parent.exists());
        }
    }

    fn setup_test_environment() -> (TempDir, String) {
        let temp_dir = TempDir::new().unwrap();
        let fake_home = temp_dir.path().to_str().unwrap().to_string();
        (temp_dir, fake_home)
    }

    // Note: The main handle_uninstall function is difficult to test directly because:
    // 1. It reads from stdin (user input)
    // 2. It calls std::process::exit(0)
    // 3. It modifies the filesystem and potentially requires sudo
    //
    // In production code, consider refactoring to:
    // 1. Accept an input reader trait instead of using stdin directly
    // 2. Return a Result instead of calling exit
    // 3. Take filesystem operations as dependencies for easier mocking
}
