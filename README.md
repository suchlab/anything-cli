# Anything CLI

**Transform any REST API into a powerful command-line interface**

`anything-cli` is a flexible CLI tool that allows you to interact with any REST API through the command line. It dynamically creates custom commands, handles authentication headers, and provides extensible instruction processing for automated workflows.

## 🚀 Features

- **Your own command**: Install this CLI with any custom command name
- **REST API Integration**: Seamlessly interact with any HTTP REST API
- **Flexible Parameter Handling**: Support for query parameters, flags, and nested endpoints
- **Custom Headers**: Configure authentication and custom headers
- **Git Context Awareness**: Automatically includes git repository information in requests
- **Instruction Processing**: Execute server-defined instructions for automation
- **Cross-Platform**: Supports macOS (Intel/Apple Silicon) and Linux
- **Zero Configuration**: Works out of the box with minimal setup

## 📦 Installation

### End Users

Install the CLI with a custom command name and base URL:

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/suchlab/anything-cli/HEAD/install/install.sh)" -- --cmd "my-api" --base-url "https://api.example.com"
```

**Parameters:**
- `--cmd`: Your custom command name (e.g., `github-cli`, `slack-bot`, `company-api`)
- `--base-url`: The base URL of the API you want to interact with

**Interactive Installation:**
```bash
curl -fsSL https://raw.githubusercontent.com/suchlab/anything-cli/HEAD/install/install.sh | bash
```

The installer will prompt you for the command name and base URL.

### Supported Platforms

- **macOS**: Intel (x64) and Apple Silicon (ARM64)
- **Linux**: x64

## 🛠 Usage

### Basic Commands

Once installed with your custom command name (e.g., `my-api`), you can use it to interact with your API:

```bash
# Get the root endpoint
my-api -> `/`

# Access specific endpoints
my-api users -> `/users`
my-api users 123 -> `/users/123`
my-api posts recent -> `/posts/recent`

# With query parameters
my-api users --limit 10 --sort name -> `/users?limit=10&sort=name`
my-api search --q "rust programming" --type repositories -> `/search?q=rust%20programing&type=repositories`

# Using flags
my-api posts --published --format json -> `/posts?published=true&format=json`

# Mixed parameters and flags
my-api users/123/posts --limit 5 --draft --since 2023-01-01 -> `/users/123/posts?limit=5&draft=true&since=2022-04-28`
```

### Command Structure

```
<command-name> [endpoint/path] [--param value] [--flag] [-f]
```

- **Endpoint/Path**: Path appended to base URL
- **Parameters**: `--key value` or `--key=value` format
- **Flags**: `--flag` (sets flag to "true")
- **Short Flags**: `-f` (single character flags)

### Internal Commands

Each installation includes built-in management commands:

```bash
# Set or update headers (for authentication, etc.)
my-api self:set-header "Authorization" "Bearer your-token-here"
my-api self:set-header "X-API-Key" "your-api-key"

# Remove a header
my-api self:set-header "Authorization"

# Update the base URL
my-api self:set-base-url "https://new-api.example.com"

# Uninstall the command
my-api self:uninstall

# Check version
my-api --version
my-api -v
```

## ⚙️ Configuration

The CLI creates a configuration directory at `~/.{command-name}/` with a `config.json` file:

```json
{
  "base_url": "https://api.example.com",
  "headers": {
    "Authorization": "Bearer your-token",
    "X-API-Key": "your-key"
  }
}
```

### Configuration Management

```bash
# View current configuration
cat ~/.my-api/config.json

# Set headers programmatically
my-api self:set-header "Authorization" "Bearer $(get-fresh-token)"

# Update base URL
my-api self:set-base-url "https://staging-api.example.com"
```

## 🎯 Anything-CLI Schema

The CLI supports a special response schema that enables server-controlled instruction execution:

### Schema Format

```json
{
  "schema": "anything-cli/v0.1",
  "instructions": [
    {
      "action": "print",
      "content": "Hello, World!",
      "error": false
    },
    {
      "action": "execute",
      "content": "echo 'Running command'",
    }
  ]
}
```

### Supported Actions

- **`ping`**: Responds with "pong" (useful for testing)
- **`print`**: Prints content to stdout (or stderr if `error: true`)
- **`execute`**: Executes shell commands
- **`none`**: Exits silently (with optional error if `error: true`)

### Example Server Response

```json
{
  "schema": "anything-cli/v0.1",
  "instructions": [
    {
      "action": "print",
      "content": "🚀 Deployment started..."
    },
    {
      "action": "execute",
      "content": "git pull origin main && npm install && npm run build"
    },
    {
      "action": "print",
      "content": "✅ Deployment completed successfully!"
    }
  ]
}
```

## 🔧 Development

### Prerequisites

- [Rust](https://rustup.rs/) 1.70 or later
- Cargo (comes with Rust)

### Setup

```bash
# Clone the repository
git clone https://github.com/suchlab/anything-cli.git
cd anything-cli

# Build the project
cargo build

# Run tests
cargo test

# Build for release
cargo build --release
```

### Cargo Commands

```bash
# Format code
cargo fmt

# Check for errors without building
cargo check

# Run clippy for linting
cargo clippy

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Build debug version
cargo build

# Build optimized release version
cargo build --release

# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Generate documentation
cargo doc --open
```

### Running During Development
```bash
# Run directly with cargo
cargo run -- --help

# Test with sample commands (Make sure ~/.anything-cli/config.json exists)
cargo run -- users --limit 10
cargo run -- self:set-header "Authorization" "Bearer test-token"

# Build and test the binary
cargo build
./target/debug/anything-cli --version
```

### Testing the Installation Script

```bash
# Test locally (requires setting up a local binary)
./install/install.sh --cmd "test-cli" --base-url "https://httpbin.org"

# Test the installed command
test-cli get --format json
test-cli self:set-header "X-Test" "value"
test-cli self:uninstall
```

## 🏗 Architecture

### Project Structure

```
src/
├── main.rs              # Application entry point and request handling
├── cli/
│   ├── cli.rs           # Command-line interface definition
│   └── parse.rs         # Parameter and flag parsing logic
├── commands/
│   ├── set_base_url.rs  # Base URL management command
│   ├── set_header.rs    # Header management command
│   └── uninstall.rs     # Uninstallation command
├── config/
│   ├── config.rs        # Configuration data structures
│   ├── loader.rs        # Configuration loading logic
│   └── saver.rs         # Configuration saving logic
├── instructions/
│   └── mod.rs           # Instruction processing engine
├── schema/
│   └── mod.rs           # Anything-CLI schema parsing
└── utils/
    ├── executable.rs    # Executable name detection
    └── git.rs           # Git repository context detection
```

### Key Modules

- **`cli`**: Command-line parsing and argument handling
- **`config`**: Configuration file management
- **`commands`**: Built-in command implementations
- **`instructions`**: Server instruction processing
- **`schema`**: Response schema parsing
- **`utils`**: Utility functions for git context and executable detection

### Request Flow

1. **Parse CLI arguments** into commands, parameters, and flags
2. **Load configuration** from `~/.{command-name}/config.json`
3. **Build HTTP request** with endpoint, query parameters, and headers
4. **Add context headers** including git repository information
5. **Send request** to the configured API
6. **Process response** through schema parser
7. **Execute instructions** or display response content

### HTTP Headers

The CLI automatically adds several headers to requests:

- `User-Agent`: `anything-cli/v{version} ({command-name}; repo: https://github.com/suchlab/anything-cli)`
- `x-anything-cli-version`: CLI version
- `x-anything-cli-executable-name`: Custom command name
- `x-anything-cli-git`: "true" (if in git repository)
- `x-anything-cli-git-repo-url`: Git remote URL
- `x-anything-cli-git-repo-name`: Repository name
- `x-anything-cli-git-branch`: Current branch name

## 🔄 Release Process

1. **Prepare main branch** with all changes
2. **Create tag**: Use GitHub Actions workflow "create tag"
3. **Create release**: Use GitHub Actions workflow "create release"

This automatically builds binaries for all supported platforms and creates a GitHub release.

## 📝 Examples

### GitHub API Integration

```bash
# Install for GitHub API
curl -fsSL https://raw.githubusercontent.com/suchlab/anything-cli/HEAD/install/install.sh | bash
# Enter: gh-api
# Enter: https://api.github.com

# Set authentication
gh-api self:set-header "Authorization" "token your-github-token"

# Use the API
gh-api user
gh-api repos octocat Hello-World
gh-api search repositories --q "rust cli" --sort stars
```

### Slack API Integration

```bash
# Install for Slack API
./install.sh --cmd "slack" --base-url "https://slack.com/api"

# Set authentication
slack self:set-header "Authorization" "Bearer xoxb-your-slack-token"

# Use the API
slack auth.test
slack channels.list --types public_channel
slack chat.postMessage --channel "#general" --text "Hello from CLI!"
```

### Custom API with Automation

```bash
# Install for your API
./install.sh --cmd "deploy" --base-url "https://deploy.yourcompany.com"

# Set authentication
deploy self:set-header "X-API-Key" "your-deploy-key"

# Trigger deployment (server returns anything-cli schema with instructions)
deploy environments production deploy --branch main
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Format code (`cargo fmt`)
6. Run clippy (`cargo clippy`)
7. Commit your changes (`git commit -m 'Add amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://rust-lang.org/) and [Clap](https://github.com/clap-rs/clap)
- HTTP client powered by [reqwest](https://github.com/seanmonstar/reqwest)
- JSON handling with [serde](https://github.com/serde-rs/serde)

## 👥 People
- [Iñigo Taibo](https://github.com/itaibo)
