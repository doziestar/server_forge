#!/bin/bash

set -e

# Detect OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
else
    echo "Unsupported OS. Please install manually."
    exit 1
fi

# Detect architecture
ARCH=$(uname -m)
if [[ "$ARCH" != "x86_64" ]]; then
    echo "Unsupported architecture. Please install manually."
    exit 1
fi

# Set GitHub repo and latest release info
REPO="doziestar/server_forge"
LATEST_RELEASE=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

# Download the appropriate package
if [[ "$OS" == "linux" ]]; then
    curl -LO "https://github.com/$REPO/releases/download/$LATEST_RELEASE/serverforge-x86_64-unknown-linux-gnu.tar.gz"
    tar xzf serverforge-x86_64-unknown-linux-gnu.tar.gz
    sudo mv serverforge /usr/local/bin/
    rm serverforge-x86_64-unknown-linux-gnu.tar.gz
elif [[ "$OS" == "macos" ]]; then
    if command -v brew &> /dev/null; then
        brew install doziestar/tap/serverforge
    else
        curl -LO "https://github.com/$REPO/releases/download/$LATEST_RELEASE/serverforge-x86_64-apple-darwin.tar.gz"
        tar xzf serverforge-x86_64-apple-darwin.tar.gz
        sudo mv serverforge /usr/local/bin/
        rm serverforge-x86_64-apple-darwin.tar.gz
    fi
fi

echo "ServerForge has been installed successfully!"