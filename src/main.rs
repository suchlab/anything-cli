mod cli {
    pub mod cli;
    pub mod parse;
}

mod config {
    pub mod config;
    pub mod loader;
    pub mod saver;
}

mod utils {
    pub mod executable;
    pub mod git;
}

mod commands {
    pub mod set_base_url;
    pub mod set_header;
    pub mod uninstall;
}

mod instructions;
mod schema;

use clap::Parser;
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;

use crate::cli::cli::Cli;
use crate::cli::parse::parse_query_params;
use crate::config::loader::load_config;
use crate::instructions::process_instructions;
use crate::schema::parse_anything_schema;
use crate::utils::executable::get_executable_name;
use crate::utils::git::get_git_repo_info;

use crate::commands::set_base_url::handle_set_base_url;
use crate::commands::set_header::handle_set_header;
use crate::commands::uninstall::handle_uninstall;

fn main() {
    let cli = Cli::parse();
    let version = env!("CARGO_PKG_VERSION");
    let mut filtered_commands: Vec<String> = Vec::new();
    let mut filtered_args: Vec<String> = Vec::new();
    let mut args_iter = cli.commands.iter().peekable();

    while let Some(arg) = args_iter.next() {
        if arg.starts_with("--") {
            filtered_args.push(arg.clone());
            if let Some(next_arg) = args_iter.peek() {
                if !next_arg.starts_with('-') {
                    filtered_args.push(args_iter.next().unwrap().clone());
                }
            }
        } else if arg.starts_with('-') {
            filtered_args.push(arg.clone());
        } else {
            filtered_commands.push(arg.clone());
        }
    }

    let executable_name = get_executable_name();

    // Print version
    if filtered_commands.is_empty()
        && filtered_args
            .iter()
            .any(|arg| arg == "-v" || arg == "--version")
    {
        println!("anything-cli v{}", version);
        std::process::exit(0);
    }

    // Internal commands
    if let Some(cmd) = filtered_commands.get(0) {
        let must_exit = match cmd.as_str() {
            "self:set-header" => {
                handle_set_header(&executable_name, &filtered_commands);
                true
            }
            "self:set-base-url" => {
                handle_set_base_url(&executable_name, &filtered_commands);
                true
            }
            "self:uninstall" => {
                handle_uninstall(&executable_name);
                true
            }
            _ => false, // Not an internal command
        };

        if must_exit {
            std::process::exit(0);
        }
    }

    let (config_option, config_path) = load_config(&executable_name);
    let config = match config_option {
        Some(config) => config,
        None => {
            eprintln!(
                "Failed to load config. Ensure {:?} exists and has the correct format.",
                &config_path
            );
            std::process::exit(1);
        }
    };

    let endpoint = if filtered_commands.is_empty() {
        format!("{}/", config.base_url)
    } else {
        format!("{}/{}", config.base_url, filtered_commands.join("/"))
    };

    let query_params = parse_query_params(&filtered_args);
    let client = Client::new();
    let mut request = client.get(&endpoint).query(&query_params);

    if let Some(headers) = config.headers {
        for (key, value) in headers {
            if !key.trim().to_lowercase().starts_with("x-anything-cli-") {
                request = request.header(&key, &value);
            }
        }
    }

    // Headers
    request = request.header(
        USER_AGENT,
        format!(
            "anything-cli/v{version} ({executable_name}; repo: https://github.com/suchlab/anything-cli)"
        ),
    );

    // Add anything-cli headers
    request = request.header("x-anything-cli-version", version);
    request = request.header("x-anything-cli-executable-name", executable_name);

    // Add git context headers
    if let Some((remote_url, repo_name, branch_name)) = get_git_repo_info() {
        request = request.header("x-anything-cli-git", "true");
        request = request.header("x-anything-cli-git-repo-url", remote_url);
        request = request.header("x-anything-cli-git-repo-name", repo_name);
        request = request.header("x-anything-cli-git-branch", branch_name);
    }

    let response = match request.send() {
        Ok(resp) => resp,
        Err(err) => {
            eprintln!("Request failed: {}", err);
            std::process::exit(1);
        }
    };

    let response_status = response.status();

    // Handle non-stream response
    let text: String = match response.text() {
        Ok(t) => t,
        Err(_) => {
            eprintln!("Failed to read response.");
            std::process::exit(1);
        }
    };

    if let Some(parsed) = parse_anything_schema(&text) {
        if let Some(instructions) = parsed.instructions {
            process_instructions(&instructions);
        } else {
            println!("{}", text.trim());
        }
    } else {
        if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&text) {
            println!("{}", serde_json::to_string(&json_val).unwrap());
        } else {
            println!("{}", text.trim());
        }
    }

    // If HTTP status code was not success, exit with error
    if !response_status.is_success() {
        std::process::exit(1);
    }
}
