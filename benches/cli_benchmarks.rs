use std::collections::HashMap;
use std::hint::black_box;

/// Benchmark for argument parsing
#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::time::Instant;

    #[test]
    fn benchmark_parse_query_params() {
        // Import the function from the CLI module
        use anything_cli::cli::parse::parse_query_params;

        let args = vec![
            "--name".to_string(),
            "value".to_string(),
            "--count".to_string(),
            "100".to_string(),
            "--verbose".to_string(),
            "--output=json".to_string(),
            "-f".to_string(),
            "--timeout".to_string(),
            "30".to_string(),
        ];

        let start = Instant::now();
        let iterations = 10000;

        for _ in 0..iterations {
            let result = parse_query_params(black_box(&args));
            black_box(result);
        }

        let duration = start.elapsed();
        let per_iteration = duration / iterations;

        println!("parse_query_params: {:?} per iteration", per_iteration);

        // Should be very fast (under 1ms for this simple case)
        assert!(
            per_iteration.as_millis() < 1,
            "parse_query_params took too long: {:?}",
            per_iteration
        );
    }

    #[test]
    fn benchmark_config_serialization() {
        use anything_cli::config::data::Config;

        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer token123".to_string());
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("User-Agent".to_string(), "anything-cli/1.0".to_string());

        let config = Config {
            base_url: "https://api.example.com/v1".to_string(),
            headers: Some(headers),
        };

        let start = Instant::now();
        let iterations = 1000;

        for _ in 0..iterations {
            let serialized = serde_json::to_string(black_box(&config)).unwrap();
            let _deserialized: Config = serde_json::from_str(black_box(&serialized)).unwrap();
        }

        let duration = start.elapsed();
        let per_iteration = duration / iterations;

        println!(
            "config serialization/deserialization: {:?} per iteration",
            per_iteration
        );

        // Should be reasonably fast (under 10ms)
        assert!(
            per_iteration.as_millis() < 10,
            "Config serialization took too long: {:?}",
            per_iteration
        );
    }

    #[test]
    fn benchmark_schema_parsing() {
        use anything_cli::schema::parse_anything_schema;

        let json_str = r#"
        {
            "schema": "anything-cli/v0.1.0",
            "instructions": [
                {
                    "action": "execute",
                    "content": "npm install",
                    "error": false
                },
                {
                    "action": "print",
                    "content": "Installation complete!"
                },
                {
                    "action": "execute",
                    "content": "npm test",
                    "error": true
                },
                {
                    "action": "none"
                }
            ]
        }"#;

        let start = Instant::now();
        let iterations = 1000;

        for _ in 0..iterations {
            let result = parse_anything_schema(black_box(json_str));
            black_box(result);
        }

        let duration = start.elapsed();
        let per_iteration = duration / iterations;

        println!("schema parsing: {:?} per iteration", per_iteration);

        // Should be reasonably fast (under 5ms)
        assert!(
            per_iteration.as_millis() < 5,
            "Schema parsing took too long: {:?}",
            per_iteration
        );
    }

    #[test]
    fn benchmark_git_url_parsing() {
        use anything_cli::utils::git::extract_repo_name;

        let urls = vec![
            "https://github.com/user/repo.git",
            "git@github.com:user/repo.git",
            "https://gitlab.com/user/project.git",
            "https://git.company.com/team/awesome-project.git",
            "https://github.com/org/subgroup/deep/repo.git",
        ];

        let start = Instant::now();
        let iterations = 10000;

        for _ in 0..iterations {
            for url in &urls {
                let result = extract_repo_name(black_box(url));
                black_box(result);
            }
        }

        let duration = start.elapsed();
        let per_iteration = duration / iterations;

        println!("git URL parsing: {:?} per iteration", per_iteration);

        // Should be very fast (under 1ms)
        assert!(
            per_iteration.as_millis() < 1,
            "Git URL parsing took too long: {:?}",
            per_iteration
        );
    }

    #[test]
    fn benchmark_executable_name_retrieval() {
        use anything_cli::utils::executable::get_executable_name;

        let start = Instant::now();
        let iterations = 1000;

        for _ in 0..iterations {
            let result = get_executable_name();
            black_box(result);
        }

        let duration = start.elapsed();
        let per_iteration = duration / iterations;

        println!("get_executable_name: {:?} per iteration", per_iteration);

        // Should be reasonably fast (under 10ms)
        assert!(
            per_iteration.as_millis() < 10,
            "get_executable_name took too long: {:?}",
            per_iteration
        );
    }

    #[test]
    fn benchmark_instruction_creation() {
        use anything_cli::schema::Instruction;

        let start = Instant::now();
        let iterations = 10000;

        for _ in 0..iterations {
            let instruction = Instruction {
                action: black_box("execute".to_string()),
                content: black_box(Some("echo 'test'".to_string())),
                error: black_box(Some(false)),
            };
            black_box(instruction);
        }

        let duration = start.elapsed();
        let per_iteration = duration / iterations;

        println!("instruction creation: {:?} per iteration", per_iteration);

        // Should be very fast (under 1ms)
        assert!(
            per_iteration.as_millis() < 1,
            "Instruction creation took too long: {:?}",
            per_iteration
        );
    }
}
