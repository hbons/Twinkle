#!/usr/bin/env bash

#   Twinkle, automatic syncing with Git
#   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
#
#   This program is free software: you can redistribute it and/or modify it
#   under the terms of the GNU General Public License v3 or any later version.

#   ～ Lovingly Handcrafted Code ～


set -euo pipefail
source ./common/test_begin.sh


REPO_NAME=repo_empty_4

mkdir "$REPO_NAME"
cd "$REPO_NAME"

touch README.md

twinkle init \
    "$HOST/$REPO_NAME" \
    .

TWINKLE_ONCE=1 twinkle sync


cd ..
source ./common/test_end.sh
