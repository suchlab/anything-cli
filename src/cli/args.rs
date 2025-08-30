use clap::Parser;

#[derive(Parser, Debug)]
#[command(disable_help_flag = true, allow_hyphen_values = true)]
pub struct Cli {
    #[arg(num_args(0..))]
    pub commands: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing_empty() {
        let cli = Cli::try_parse_from(["test"]).unwrap();
        assert!(cli.commands.is_empty());
    }

    #[test]
    fn test_cli_parsing_single_command() {
        let cli = Cli::try_parse_from(["test", "hello"]).unwrap();
        assert_eq!(cli.commands, vec!["hello"]);
    }

    #[test]
    fn test_cli_parsing_multiple_commands() {
        let cli = Cli::try_parse_from(["test", "hello", "world", "test"]).unwrap();
        assert_eq!(cli.commands, vec!["hello", "world", "test"]);
    }

    #[test]
    fn test_cli_parsing_with_flags() {
        let cli = Cli::try_parse_from(["test", "command", "--flag", "value", "-v"]).unwrap();
        assert_eq!(cli.commands, vec!["command", "--flag", "value", "-v"]);
    }

    #[test]
    fn test_cli_parsing_hyphen_values() {
        let cli = Cli::try_parse_from(["test", "command", "--negative-value", "-123"]).unwrap();
        assert_eq!(cli.commands, vec!["command", "--negative-value", "-123"]);
    }
}
