#!/usr/bin/env bash

#   Twinkle, automatic syncing with Git
#   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
#
#   This program is free software: you can redistribute it and/or modify it
#   under the terms of the GNU General Public License v3 or any later version.


set -euo pipefail
source ./common/config.sh
source ./common/install_keys.sh

REPO_NAME="test_sync_aa_$TAG"
REPO_NAME_1="${REPO_NAME}_1"
REPO_NAME_2="${REPO_NAME}_2"

gh repo create \
    $REPO_NAME \
    --private


DEBUG=1 twinkle clone \
    git@github.com:$ACCOUNT/$REPO_NAME

mv $REPO_NAME $REPO_NAME_1

DEBUG=1 twinkle clone \
    git@github.com:$ACCOUNT/$REPO_NAME

mv $REPO_NAME $REPO_NAME_2


cd $REPO_NAME_1
echo "Alice" >> README.md
git config user.name "Alice"
TWINKLE_ONCE=1 twinkle sync

cd ..

cd $REPO_NAME_2
echo "Bob" >> README.md
git config user.name "Bob"
TWINKLE_ONCE=1 twinkle sync


test -f README.md
test -f "README (Alice).md"
test "$(cat "README.md")" = "Bob"
test "$(cat "README (Alice).md")" = "Alice"


# TODO: Doesn't work...
# source ../common/test_synced.sh
LOCAL=$(git rev-parse HEAD)
REMOTE=$(git ls-remote origin HEAD | cut -f1)
[[ "$LOCAL" == "$REMOTE" ]]


rm -Rf "$REPO_NAME"
rm -Rf "$REPO_NAME_1"
rm -Rf "$REPO_NAME_2"

cd ..
source ./common/cleanup.sh
