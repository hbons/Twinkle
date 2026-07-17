#!/usr/bin/env bash

set -euo pipefail

echo "--- CLEANUP ---"

echo "KEY_FILE: $KEY_FILE"
echo "KEY_ID: $KEY_ID"
echo "SIGNING_KEY_ID: $SIGNING_KEY_ID"
echo "ACCOUNT: $ACCOUNT"
echo "REPO_NAME: $REPO_NAME"

gh repo delete \
    $ACCOUNT/$REPO_NAME \
    --yes

gh api -X DELETE "user/keys/$KEY_ID"
gh api -X DELETE "user/ssh_signing_keys/$SIGNING_KEY_ID"

rm -f "$KEY_FILE"
rm -f "$KEY_FILE.pub"

rm -Rf .git/

echo "--- END CLEANUP ---"
