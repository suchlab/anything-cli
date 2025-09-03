#!/bin/bash

set -e

# Parse arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --cmd) CMD_NAME="$2"; shift ;;
        --base-url) BASE_URL="$2"; shift ;;
        *) echo "Unknown parameter: $1"; exit 1 ;;
    esac
    shift
done

# Prompt for command name if not provided
if [ -z "$CMD_NAME" ]; then
    read -p "Enter the command name you want to install (e.g., my-command): " CMD_NAME
fi

# Prompt for base URL if not provided
if [ -z "$BASE_URL" ]; then
    read -p "Enter the base URL (e.g., https://api.example.com): " BASE_URL
fi

# Ensure values are provided
if [ -z "$CMD_NAME" ] || [ -z "$BASE_URL" ]; then
    echo "Error: Both command name and base URL are required."
    exit 1
fi

# Determine OS and set binary URL
OS="$(uname -s)"
ARCH="$(uname -m)"

if [[ "$OS" == "Darwin" ]]; then
    if [[ "$ARCH" == "arm64" ]]; then
        URL="https://github.com/suchlab/anything-cli/releases/latest/download/anything-cli-darwin-arm64.tar.gz"
    else
        URL="https://github.com/suchlab/anything-cli/releases/latest/download/anything-cli-darwin-amd64.tar.gz"
    fi
elif [[ "$OS" == "Linux" ]]; then
    URL="https://github.com/suchlab/anything-cli/releases/latest/download/anything-cli-linux-amd64.tar.gz"
else
    echo "Unsupported OS: $OS"
    exit 1
fi

# Create temp directory and download the binary
TMP_DIR=$(mktemp -d)
curl -sSL "$URL" | tar -xz -C "$TMP_DIR"

# Set installation path
INSTALL_PATH="/usr/local/bin/$CMD_NAME"

# Ask for sudo password upfront if necessary
if [ ! -w "/usr/local/bin" ]; then
    echo "Root access required to install '$CMD_NAME'. Please enter your password."
    sudo -v # Ask for sudo password upfront
fi

# Move binary to /usr/local/bin with custom name
sudo mv "$TMP_DIR/"* "$INSTALL_PATH"
sudo chmod +x "$INSTALL_PATH"

# Create config directory and file
CONFIG_DIR="$HOME/.$CMD_NAME"
mkdir -p "$CONFIG_DIR"

# Create the config.json file with base_url
cat <<-EOF > "$CONFIG_DIR/config.json"
{
  "base_url": "$BASE_URL"
}
EOF

echo "Installed '$CMD_NAME' successfully!"
