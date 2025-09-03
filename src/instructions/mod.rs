use crate::schema::Instruction;
use std::process::Command;

pub fn process_instructions(instructions: &[Instruction]) {
    for instr in instructions {
        match instr.action.as_str() {
            "ping" => {
                println!("pong");
            }
            "execute" => {
                if let Some(script) = &instr.content {
                    execute_script(script);
                    if instr.error.unwrap_or(false) {
                        std::process::exit(1);
                    }
                } else {
                    eprintln!("No content found for 'execute' action.");
                }
            }
            "print" => {
                if let Some(content) = &instr.content {
                    match instr.error {
                        Some(true) => eprintln!("{}", content),
                        _ => println!("{}", content),
                    }
                } else {
                    eprintln!("No content.");
                }
            }
            "none" => {
                if instr.error.unwrap_or(false) {
                    std::process::exit(1);
                }
                std::process::exit(0);
            }
            _ => {
                eprintln!("Unsupported action: {}", instr.action);
            }
        }
    }
}

fn execute_script(script: &str) {
    let status = Command::new("sh").arg("-c").arg(script).status();

    match status {
        Ok(exit_status) => {
            if !exit_status.success() {
                eprintln!("Error: {:?}", exit_status);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
