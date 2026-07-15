#!/usr/bin/env bash

set -euo pipefail

ssh-keygen \
    -f $KEY_FILE \
    -t ed25519 \
    -P ""

gh ssh-key add \
    --type authentication \
    --title "Twinkle Integration Test" \
    $KEY_FILE.pub

gh ssh-key add \
    --type signing \
    --title "Twinkle Integration Test" \
    $KEY_FILE.pub

eval "$(ssh-agent -s)"
ssh-add $KEY_FILE
ssh-add -l
