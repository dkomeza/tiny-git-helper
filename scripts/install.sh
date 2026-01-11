#!/usr/bin/env bash
set -euo pipefail

# -------------------------------------------------------------------------
# Formatting & Colors
# -------------------------------------------------------------------------
if [[ -t 1 ]]; then
    Reset='\033[0m'
    Red='\033[31m'
    Green='\033[32m'
    Dim='\033[2m'
    Bold_White='\033[1m'
    Bold_Green='\033[1;32m'
else
    Reset=''
    Red=''
    Green=''
    Dim=''
    Bold_White=''
    Bold_Green=''
fi

error() {
    printf "${Red}error${Reset}: %b\n" "$*" >&2
    exit 1
}

info() {
    printf "${Dim}%b ${Reset}\n" "$*"
}

info_bold() {
    printf "${Bold_White}%b ${Reset}\n" "$*"
}

success() {
    printf "${Green}%b ${Reset}\n" "$*"
}

# -------------------------------------------------------------------------
# Argument Parsing (Positional Tag)
# -------------------------------------------------------------------------
VERSION="latest"

# Check if first argument exists and doesn't start with '-'
if [[ $# -gt 0 && ! "$1" =~ ^- ]]; then
    VERSION="$1"
fi

# -------------------------------------------------------------------------
# Platform & Architecture Detection
# -------------------------------------------------------------------------
os=$(uname -s)
arch=$(uname -m)
target=""
ext=""

case $os in
    Darwin)
        ext="zip"
        case $arch in
            x86_64)
                if [[ $(sysctl -n sysctl.proc_translated 2>/dev/null) == "1" ]]; then
                    target="aarch64-apple-darwin"
                    info "Running in Rosetta 2. Selecting native Apple Silicon binary."
                else
                    target="x86_64-apple-darwin"
                fi
                ;;
            arm64)
                target="aarch64-apple-darwin"
                ;;
            *)
                error "Unsupported macOS architecture: $arch"
                ;;
        esac
        ;;
    Linux)
        ext="tar.gz"
        case $arch in
            x86_64)
                target="x86_64-unknown-linux-gnu"
                ;;
            aarch64|arm64)
                target="aarch64-unknown-linux-gnu"
                ;;
            *)
                error "Unsupported Linux architecture: $arch"
                ;;
        esac
        ;;
    *)
        error "Unsupported OS: $os"
        ;;
esac

# -------------------------------------------------------------------------
# Configuration
# -------------------------------------------------------------------------
REPO_URL="https://github.com/dkomeza/tiny-git-helper"

if [[ "$VERSION" == "latest" ]]; then
    DOWNLOAD_URL="$REPO_URL/releases/latest/download/tgh-$target.$ext"
else
    DOWNLOAD_URL="$REPO_URL/releases/download/$VERSION/tgh-$target.$ext"
fi

install_env=TGH_INSTALL
bin_env=\$$install_env/bin

install_dir=${TGH_INSTALL:-"$HOME/.tgh"}
bin_dir="$install_dir/bin"
exe="$bin_dir/tgh"

# -------------------------------------------------------------------------
# Installation
# -------------------------------------------------------------------------
info_bold "Installing tgh ($VERSION) for $target..."

if [[ ! -d $bin_dir ]]; then
    mkdir -p "$bin_dir"
fi

# Download
echo
info "Downloading from $DOWNLOAD_URL"
if ! curl --fail --location --progress-bar --output "tgh_download.$ext" "$DOWNLOAD_URL"; then
    echo
    error "Download failed. Please check version '$VERSION' exists or your internet connection."
fi

# Extract based on extension
echo
info "Extracting..."
if [[ "$ext" == "zip" ]]; then
    unzip -q -o -j "tgh_download.$ext" -d "$bin_dir" || error "Failed to unzip archive"
else
    tar -xzf "tgh_download.$ext" -C "$bin_dir" || error "Failed to extract tar archive"
fi

# Cleanup
rm -f "tgh_download.$ext"

# Make executable
chmod +x "$exe" || error "Failed to make binary executable"

# -------------------------------------------------------------------------
# Shell Configuration
# -------------------------------------------------------------------------
tildify() {
    if [[ $1 = $HOME/* ]]; then
        local replacement=\~/
        echo "${1/$HOME\//$replacement}"
    else
        echo "$1"
    fi
}

success "tgh ($VERSION) was installed successfully to $Bold_Green$(tildify "$exe")"

refresh_command=''
tilde_bin_dir=$(tildify "$bin_dir")
quoted_install_dir=\"${install_dir//\"/\\\"}\"

echo

shell_name=$(basename "$SHELL")

case $shell_name in
fish)
    commands=(
        "set --export $install_env $quoted_install_dir"
        "set --export PATH $bin_env \$PATH"
    )

    fish_config=$HOME/.config/fish/config.fish
    tilde_fish_config=$(tildify "$fish_config")

    if [[ -w $fish_config ]]; then
        if ! grep -q "$install_env" "$fish_config"; then
            {
                echo ''
                echo "# tgh"
                for command in "${commands[@]}"; do
                    echo "$command"
                done
            } >>"$fish_config"
            info "Added \"$tilde_bin_dir\" to \$PATH in \"$tilde_fish_config\""
        else
            info "Path already configured in \"$tilde_fish_config\""
        fi
        refresh_command="source $tilde_fish_config"
    else
        echo "Manually add the directory to $tilde_fish_config:"
        for command in "${commands[@]}"; do
            info_bold "  $command"
        done
    fi
    ;;
zsh)
    commands=(
        "export $install_env=$quoted_install_dir"
        "export PATH=\"$bin_env:\$PATH\""
    )

    zsh_config=$HOME/.zshrc
    tilde_zsh_config=$(tildify "$zsh_config")

    if [[ -w $zsh_config ]]; then
        if ! grep -q "$install_env" "$zsh_config"; then
            {
                echo ''
                echo '# tgh'
                for command in "${commands[@]}"; do
                    echo "$command"
                done
            } >>"$zsh_config"
            info "Added \"$tilde_bin_dir\" to \$PATH in \"$tilde_zsh_config\""
        else
            info "Path already configured in \"$tilde_zsh_config\""
        fi
        refresh_command="exec $SHELL"
    else
        echo "Manually add the directory to $tilde_zsh_config:"
        for command in "${commands[@]}"; do
            info_bold "  $command"
        done
    fi
    ;;
bash)
    commands=(
        "export $install_env=$quoted_install_dir"
        "export PATH=$bin_env:\$PATH"
    )
    
    bash_configs=(
        "$HOME/.bashrc"
        "$HOME/.bash_profile"
    )

    set_manually=true
    for bash_config in "${bash_configs[@]}"; do
        if [[ -w $bash_config ]]; then
            tilde_bash_config=$(tildify "$bash_config")
            
            if ! grep -q "$install_env" "$bash_config"; then
                {
                    echo ''
                    echo '# tgh'
                    for command in "${commands[@]}"; do
                        echo "$command"
                    done
                } >>"$bash_config"
                info "Added \"$tilde_bin_dir\" to \$PATH in \"$tilde_bash_config\""
            else
                info "Path already configured in \"$tilde_bash_config\""
            fi
            
            refresh_command="source $bash_config"
            set_manually=false
            break
        fi
    done

    if [[ $set_manually = true ]]; then
        echo "Manually add the directory to your bash config:"
        for command in "${commands[@]}"; do
            info_bold "  $command"
        done
    fi
    ;;
*)
    echo 'Manually add the directory to your shell config:'
    info_bold "  export $install_env=$quoted_install_dir"
    info_bold "  export PATH=\"$bin_env:\$PATH\""
    ;;
esac

echo
info "To get started, run:"
echo

if [[ $refresh_command ]]; then
    info_bold "  $refresh_command"
fi

info_bold "  tgh --help"
echo
