#!/usr/bin/env bash

#   Twinkle, automatic syncing with Git
#   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
#
#   This program is free software: you can redistribute it and/or modify it
#   under the terms of the GNU General Public License v3 or any later version.

#   ～ Lovingly Handcrafted Code ～


set -euo pipefail
source ../common/test_begin.sh


REPO_NAME="repo_readme_69"
REPO_NAME_1="${REPO_NAME}_1"
REPO_NAME_2="${REPO_NAME}_2"


twinkle clone "$HOST/$REPO_NAME"
mv "$REPO_NAME" "$REPO_NAME_1"

twinkle clone "$HOST/$REPO_NAME"
mv "$REPO_NAME" "$REPO_NAME_2"


cd "$REPO_NAME_1"
echo "Alice" > README.md
git config user.name "Alice"
TWINKLE_ONCE=1 twinkle sync
cd ..

cd "$REPO_NAME_2"
rm README.md
git config user.name "Bob"
TWINKLE_ONCE=1 twinkle sync


! test -f .git/MERGE_HEAD
test -f README.md
test "$(cat "README.md")" = "Alice"


cd ..
source ../common/test_end.sh
