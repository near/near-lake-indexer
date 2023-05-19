#!/bin/bash

set -e

# Back up current binary
mv ~/near-lake ~/near-lake-$(~/near-lake -V | awk '{print $2}')

# Download latest binary
wget -P ~ https://github.com/near/near-lake-indexer/releases/latest/download/near-lake
chmod +x ~/near-lake

# Restart service
# sudo systemctl restart lake

# Destroy
rm -- "$0"
