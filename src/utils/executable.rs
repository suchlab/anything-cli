use std::env;

pub fn get_executable_name() -> String {
    env::current_exe()
        .ok()
        .and_then(|path| path.file_stem().map(|n| n.to_string_lossy().into_owned()))
        .unwrap_or_else(|| "default".to_string())
}
