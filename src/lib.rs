pub mod cli {
    pub mod args;
    pub mod parse;
}

pub mod config {
    pub mod data;
    pub mod loader;
    pub mod saver;
}

pub mod utils {
    pub mod executable;
    pub mod git;
}

pub mod commands {
    pub mod set_base_url;
    pub mod set_header;
    pub mod uninstall;
}

pub mod instructions;
pub mod schema;
