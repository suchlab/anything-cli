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
                    std::process::exit(1);
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
                    std::process::exit(1);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_script_echo() {
        // This test executes a simple echo command
        // Since execute_script doesn't return a value, we're mainly testing it doesn't panic
        execute_script("echo 'test'");
    }

    #[test]
    fn test_execute_script_true() {
        execute_script("true");
    }

    #[test]
    fn test_execute_script_false() {
        // This should print an error but not panic
        execute_script("false");
    }

    #[test]
    fn test_execute_script_invalid_command() {
        // Test with an invalid command
        // This should print an error but not panic
        execute_script("nonexistent_command_12345");
    }

    #[test]
    fn test_execute_script_complex() {
        // Test with a more complex script
        execute_script("echo 'hello' && echo 'world'");
    }

    #[test]
    fn test_instruction_creation() {
        // Test that we can create instructions manually for testing
        let ping_instruction = Instruction {
            action: "ping".to_string(),
            content: None,
            error: None,
        };

        let print_instruction = Instruction {
            action: "print".to_string(),
            content: Some("Hello, World!".to_string()),
            error: Some(false),
        };

        let execute_instruction = Instruction {
            action: "execute".to_string(),
            content: Some("echo 'test'".to_string()),
            error: Some(true),
        };

        assert_eq!(ping_instruction.action, "ping");
        assert!(ping_instruction.content.is_none());

        assert_eq!(print_instruction.action, "print");
        assert_eq!(print_instruction.content, Some("Hello, World!".to_string()));
        assert_eq!(print_instruction.error, Some(false));

        assert_eq!(execute_instruction.action, "execute");
        assert_eq!(execute_instruction.content, Some("echo 'test'".to_string()));
        assert_eq!(execute_instruction.error, Some(true));
    }
}
