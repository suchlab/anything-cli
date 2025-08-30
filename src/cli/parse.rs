use std::collections::HashMap;

pub fn parse_query_params(args: &[String]) -> HashMap<String, String> {
    let mut params = HashMap::new();
    let mut iter = args.iter().peekable();

    while let Some(arg) = iter.next() {
        if let Some((key, value)) = arg.strip_prefix("--").and_then(|s| s.split_once('=')) {
            params.insert(key.to_string(), value.to_string());
        } else if let Some(key) = arg.strip_prefix("--") {
            if let Some(next_arg) = iter.peek() {
                if !next_arg.starts_with('-') {
                    params.insert(key.to_string(), iter.next().unwrap().clone());
                } else {
                    params.insert(key.to_string(), "true".to_string());
                }
            } else {
                params.insert(key.to_string(), "true".to_string());
            }
        } else if let Some(flags) = arg.strip_prefix('-') {
            for flag in flags.chars() {
                params.insert(flag.to_string(), "true".to_string());
            }
        }
    }
    params
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_query_params_empty() {
        let args: Vec<String> = vec![];
        let params = parse_query_params(&args);
        assert!(params.is_empty());
    }

    #[test]
    fn test_parse_query_params_single() {
        let args = vec!["--name".to_string(), "value".to_string()];
        let params = parse_query_params(&args);
        assert_eq!(params.get("name").unwrap(), "value");
    }

    #[test]
    fn test_parse_query_params_single_equal() {
        let args = vec!["--name=value".to_string()];
        let params = parse_query_params(&args);
        assert_eq!(params.get("name").unwrap(), "value");
    }

    #[test]
    fn test_parse_query_params_multiple() {
        let args = vec![
            "--name".to_string(),
            "value".to_string(),
            "--age".to_string(),
            "25".to_string(),
            "--sort=ASC".to_string(),
        ];
        let params = parse_query_params(&args);
        assert_eq!(params.get("name").unwrap(), "value");
        assert_eq!(params.get("age").unwrap(), "25");
        assert_eq!(params.get("sort").unwrap(), "ASC");
    }

    #[test]
    fn test_parse_query_params_flag_only() {
        let args = vec!["--verbose".to_string()];
        let params = parse_query_params(&args);
        assert_eq!(params.get("verbose").unwrap(), "true");
    }

    #[test]
    fn test_parse_query_params_flag_only_short() {
        let args = vec!["-v".to_string()];
        let params = parse_query_params(&args);
        assert_eq!(params.get("v").unwrap(), "true");
    }

    #[test]
    fn test_parse_query_params_mixed() {
        let args = vec![
            "--name".to_string(),
            "value".to_string(),
            "-t".to_string(),
            "--verbose".to_string(),
            "--count".to_string(),
            "10".to_string(),
            "--kind=new".to_string(),
            "-p".to_string(),
        ];
        let params = parse_query_params(&args);
        assert_eq!(params.get("name").unwrap(), "value");
        assert_eq!(params.get("verbose").unwrap(), "true");
        assert_eq!(params.get("t").unwrap(), "true");
        assert_eq!(params.get("count").unwrap(), "10");
        assert_eq!(params.get("kind").unwrap(), "new");
        assert_eq!(params.get("p").unwrap(), "true");
    }
}
