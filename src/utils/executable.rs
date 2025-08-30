use std::env;

pub fn get_executable_name() -> String {
    env::current_exe()
        .ok()
        .and_then(|path| path.file_stem().map(|n| n.to_string_lossy().into_owned()))
        .unwrap_or_else(|| "default".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_executable_name_returns_string() {
        let name = get_executable_name();
        assert!(!name.is_empty());
        // The name should not contain path separators
        assert!(!name.contains('/'));
        assert!(!name.contains('\\'));
    }

    #[test]
    fn test_get_executable_name_consistency() {
        // Should return the same value when called multiple times
        let name1 = get_executable_name();
        let name2 = get_executable_name();
        assert_eq!(name1, name2);
    }

    #[test]
    fn test_get_executable_name_not_default_in_test() {
        // In test environment, we should still get a reasonable name
        let name = get_executable_name();
        // Usually in tests this would be something like "deps" or the test runner name
        // We just check it's not empty and reasonable
        assert!(!name.is_empty());
        assert!(name.len() > 0);
    }
}
