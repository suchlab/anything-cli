use crate::config::loader::load_config;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

pub fn handle_uninstall(executable_name: &String) {
    let (_, config_path) = load_config(&executable_name);

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
        if let Err(e) = fs::remove_file(config_path) {
            eprintln!("Failed to delete config file: {}", e);
        } else {
            println!("Config file deleted.");
        }
    }

    // Delete the executable - Checks for permissions
    if let Err(_) = fs::remove_file(&exe_path) {
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
