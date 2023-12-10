#!/bin/sh

OS=""
ARCH=$(uname -m)
APP_NAME="tgh-$OS-$ARCH-$VERSION"
REPO="dkomeza/tiny-git-helper"
INSTALL_DIR="/usr/local/bin"

get_latest_version() {
    VERSION=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    if [ -z "$VERSION" ]; then
        echo "Failed to retrieve the latest version."
        exit 1
    fi
}

check_supported_os() {
    if [ "$(uname -s)" = "Linux" ]; then
        OS="linux"
    elif [ "$(uname -s)" = "Darwin" ]; then
        OS="macos"
    else
        echo "Unsupported operating system: $(uname -s)"
        exit 1
    fi
}

check_supported_arch() {
    if [ "$ARCH" != "x86_64" ] && [ "$ARCH" != "arm64" ]; then
        echo "Unsupported architecture: $ARCH"
        exit 1
    fi
}

update_app_name() {
    APP_NAME="tgh-$OS-$ARCH-$VERSION"
}

download_and_install() {
    DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/$APP_NAME"

    echo "Downloading $DOWNLOAD_URL"
    # HTTP_STATUS_CODE=$(curl -s -L -o /dev/null -w "%{http_code}" "$DOWNLOAD_URL")
    # if [ "$HTTP_STATUS_CODE" -eq 200 ]; then
    #     curl -L --output tgh "$DOWNLOAD_URL"
    #     chmod +x tgh
    #     mv tgh "$INSTALL_DIR"
    #     echo "Installation complete."
    # else
    #     echo "No suitable version found for your system."
    #     exit 1
    # fi
}

check_supported_os
check_supported_arch
get_latest_version
update_app_name
download_and_install