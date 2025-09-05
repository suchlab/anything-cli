use crate::schema::Instruction;
use std::process::Command;

pub fn process_instructions(instructions: &[Instruction]) -> Result<(), i32> {
    let mut has_error = false;

    for instr in instructions {
        match instr.action.as_str() {
            "ping" => {
                println!("pong");
            }
            "execute" => {
                if let Some(script) = &instr.content {
                    execute_script(script);
                    if instr.error.unwrap_or(false) {
                        has_error = true;
                    }
                }
            }
            "print" => {
                if let Some(content) = &instr.content {
                    match instr.error {
                        Some(true) => {
                            eprintln!("{}", content);
                            has_error = true;
                        }
                        _ => println!("{}", content),
                    }
                } else {
                    println!();
                }
            }
            "none" => {
                if instr.error.unwrap_or(false) {
                    has_error = true;
                }
            }
            _ => {
                eprintln!("Unsupported action: {}", instr.action);
            }
        }
    }

    if has_error {
        Err(1)
    } else {
        Ok(())
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

    #[test]
    fn test_process_instructions_with_errors() {
        // Test that instructions continue processing even when errors occur
        let instructions = vec![
            Instruction {
                action: "ping".to_string(),
                content: None,
                error: None,
            },
            Instruction {
                action: "print".to_string(),
                content: Some("This should print".to_string()),
                error: Some(false),
            },
            Instruction {
                action: "execute".to_string(),
                content: Some("echo 'test'".to_string()),
                error: Some(true), // This should cause an error
            },
            Instruction {
                action: "print".to_string(),
                content: Some("This should still print despite previous error".to_string()),
                error: Some(false),
            },
        ];

        // Should return error because one instruction had error: true
        let result = process_instructions(&instructions);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), 1);
    }

    #[test]
    fn test_process_instructions_print_with_error_flag() {
        // Test that print instruction with error: true causes failure
        let instructions = vec![
            Instruction {
                action: "print".to_string(),
                content: Some("This is an error message".to_string()),
                error: Some(true), // This should cause an error
            },
            Instruction {
                action: "print".to_string(),
                content: Some("This should still print".to_string()),
                error: Some(false),
            },
        ];

        // Should return error because print instruction had error: true
        let result = process_instructions(&instructions);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), 1);
    }

    #[test]
    fn test_process_instructions_no_errors() {
        // Test that instructions return Ok when no errors occur
        let instructions = vec![
            Instruction {
                action: "ping".to_string(),
                content: None,
                error: None,
            },
            Instruction {
                action: "print".to_string(),
                content: Some("Hello".to_string()),
                error: Some(false),
            },
            Instruction {
                action: "execute".to_string(),
                content: Some("echo 'test'".to_string()),
                error: Some(false),
            },
        ];

        let result = process_instructions(&instructions);
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_instructions_warnings_dont_fail() {
        // Test that warnings (like empty print, unsupported actions, empty execute) don't cause failure
        let instructions = vec![
            Instruction {
                action: "ping".to_string(),
                content: None,
                error: None,
            },
            Instruction {
                action: "print".to_string(),
                content: None, // No content - should print empty line, not fail
                error: Some(false),
            },
            Instruction {
                action: "execute".to_string(),
                content: None, // No content - should be allowed (no-op), not fail
                error: Some(false),
            },
            Instruction {
                action: "unknown_action".to_string(), // Unsupported action - should warn, not fail
                content: Some("test".to_string()),
                error: Some(false),
            },
            Instruction {
                action: "print".to_string(),
                content: Some("This should still work".to_string()),
                error: Some(false),
            },
        ];

        // Should return Ok because warnings don't cause failure
        let result = process_instructions(&instructions);
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_instructions_missing_execute_content_succeeds() {
        // Test that missing content for execute action is allowed (might be intentional no-op)
        let instructions = vec![Instruction {
            action: "execute".to_string(),
            content: None, // Missing content for execute is now allowed
            error: Some(false),
        }];

        let result = process_instructions(&instructions);
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_instructions_none_with_error() {
        // Test that "none" action with error: true causes failure
        let instructions = vec![Instruction {
            action: "none".to_string(),
            content: None,
            error: Some(true), // Should cause error
        }];

        let result = process_instructions(&instructions);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), 1);
    }
}
