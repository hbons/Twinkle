#!/usr/bin/env bash

set -euo pipefail

gh api user/keys --jq '.[].id' |
while read -r id; do
    gh api -X DELETE "user/keys/$id"
done

gh api user/ssh_signing_keys --jq '.[].id' |
while read -r id; do
    gh api -X DELETE "user/ssh_signing_keys/$id"
done

rm -f "$KEY_FILE"
rm -f "$KEY_FILE.pub"
rm -Rf .git/

# TODO
# gh repo delete \
#     $REPO_NAME \
#     --yes
