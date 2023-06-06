#!/bin/bash

# Enable immediate script termination if any command fails
set -e

# Function to print echo messages in yellow color with separators
function print_yellow {
  echo -e "\e[1;33m$1\e[0m"
}

print_yellow "Downloading latest binary..."
wget -O ~/near-lake-latest https://github.com/near/near-lake-indexer/releases/latest/download/near-lake
chmod +x ~/near-lake-latest

latest_version=$(~/near-lake-latest -V | awk '{print $2}')
prev_version=$(~/near-lake -V | awk '{print $2}')

if [[ "$latest_version" == "$prev_version" ]]; then
    print_yellow "Latest version of near-lake is already running. Exiting..."
    rm ~/near-lake-latest
    exit 1
fi

print_yellow "Backing up current near-lake binary to ~/near-lake-$prev_version..."
mv ~/near-lake ~/near-lake-$prev_version

print_yellow "Moving near-lake v$latest_version binary to ~/near-lake..."
mv ~/near-lake-latest ~/near-lake

print_yellow "Restarting near-lake systemd..."
sudo systemctl restart lake

print_yellow "Printing lake service logs..."
journalctl -u lake --since "5 min ago" -f
