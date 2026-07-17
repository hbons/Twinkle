#!/usr/bin/env bash

set -euo pipefail

echo "--- INSTALL KEYS ---"

KEY_TITLE="Integration Test $TAG"

echo "KEY_FILE: $KEY_FILE"
echo "KEY_TITLE: $KEY_TITLE"


ssh-keygen \
    -f $KEY_FILE \
    -t ed25519 \
    -P ""

eval "$(ssh-agent -s)"
ssh-add $KEY_FILE
ssh-add -l


gh ssh-key add \
    --type authentication \
    --title "$KEY_TITLE" \
    $KEY_FILE.pub

gh ssh-key add \
    --type signing \
    --title "$KEY_TITLE" \
    $KEY_FILE.pub


export KEY_ID=$(
    gh api user/keys \
        --jq ".[] | select(.title == \"$KEY_TITLE\") | .id"
)

export SIGNING_KEY_ID=$(
    gh api user/ssh_signing_keys \
        --jq ".[] | select(.title == \"$KEY_TITLE\") | .id"
)

echo "--- END INSTALL KEYS ---"
