#!/bin/bash

# Enable immediate script termination if any command fails
set -e

# Function to print echo messages in yellow color with separators
function print_yellow {
  echo -e "\e[1;33m$1\e[0m"
}

# Function to check if a command exists
function command_exists {
  command -v "$1" >/dev/null 2>&1
}

# Function to check if a package is installed
function package_is_installed {
  dpkg-query -Wf'${db:Status-abbrev}' "$1" 2>/dev/null | grep -q '^i'
}

if ! command_exists rustc; then
  print_yellow "Rust is not installed. Installing now..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source "$HOME/.cargo/env"
fi

if ! package_is_installed build-essential; then
  print_yellow "build-essential is not installed. Installing now..."
  sudo apt-get install -y build-essential
fi

if ! package_is_installed libclang-dev; then
  print_yellow "libclang-dev is not installed. Installing now..."
  sudo apt-get install -y libclang-dev
fi

cd ~/near-lake-indexer

print_yellow "Pulling latest near-lake-indexer main..."
git checkout main
git pull origin

version=$(grep ^version Cargo.toml | awk '{print $3}' | tr -d \")
prev_version=$(~/near-lake -V | awk '{print $2}')

if [[ "$version" == "$prev_version" ]]; then
    print_yellow "Latest version of near-lake is already running. Exiting..."
    exit 1
fi

print_yellow "Building release version of near-lake v$(version)..."
cargo build --release

cd ~

print_yellow "Backing up current near-lake binary to ~/near-lake-$(prev_version)..."
mv ~/near-lake ~/near-lake-$(prev_version)

print_yellow "Moving near-lake v$(version) binary to ~/near-lake..."
cp ~/near-lake-indexer/target/release/near-lake ~/near-lake
chmod +x ~/near-lake

print_yellow "Restarting near-lake systemd..."
sudo systemctl restart lake

print_yellow "Printing lake service logs..."
journalctl -u lake --since "5 min ago" -f
