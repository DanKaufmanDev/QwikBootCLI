#!/usr/bin/env bash
set -e 

REPO="DanKaufmanDev/QwikBootCLI"
BINARY="qwikboot"
INSTALL_DIR="/usr/local/bin"

echo "Installing $BINARY..."

OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

if [[ $OS == "darwin" ]]; then
    TARGET="x86_64-apple-darwin"
elif [[ $OS == "linux" ]]; then
    TARGET="x86_64-unknown-linux-gnu"
else
    echo "Unsupported OS: $OS"
    exit 1
fi

URL=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" \
    | grep "browser_download_url.*$TARGET" \
    | cut -d '"' -f 4)

echo "Downloading $BINARY from $URL"
curl -L "$URL" -o /tmp/$BINARY.tar.gz
tar -xzf /tmp/$BINARY.tar.gz -C /tmp

sudo cp /tmp/$BINARY $INSTALL_DIR/$BINARY
sudo chmod +x $INSTALL_DIR/$BINARY

echo "QwikBoot CLI installed successfully!"