use clap::Parser;

#[derive(Parser, Debug)]
#[command(disable_help_flag = true, allow_hyphen_values = true)]
pub struct Cli {
    #[arg(num_args(0..))]
    pub commands: Vec<String>,
}
