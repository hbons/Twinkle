#!/usr/bin/env bash

#   Twinkle, automatic syncing with Git
#   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
#
#   This program is free software: you can redistribute it and/or modify it
#   under the terms of the GNU General Public License v3 or any later version.

#   ～ Lovingly Handcrafted Code ～


set -euo pipefail
source ../common/test_begin.sh


REPO_NAME="repo_readme_32"


git clone "$HOST/$REPO_NAME"
cd "$REPO_NAME"

LARGE_FILE=large_file.bin
dd if=/dev/zero of="$LARGE_FILE" bs=1M count=1
git lfs install
git lfs track "$LARGE_FILE"
git add "$LARGE_FILE"
git add .gitattributes
git config user.name "Alice"
git commit -m "Add large file"
git push origin main

cd ..
rm -Rf "$REPO_NAME"

twinkle clone "$HOST/$REPO_NAME"
cd "$REPO_NAME"

grep "$LARGE_FILE" .gitattributes
test -f "$LARGE_FILE"
[ $(wc -c < "$LARGE_FILE") -eq 1048576 ]
git config sync.lfs.enabled
git lfs ls-files | grep "$LARGE_FILE"
cd..


cd ..
source ./common/test_end.sh
