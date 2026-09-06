#!/usr/bin/env bash

#   Twinkle, automatic syncing with Git
#   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
#
#   This program is free software: you can redistribute it and/or modify it
#   under the terms of the GNU General Public License v3 or any later version.


set -euo pipefail
source ./common/config.sh
source ./common/install_keys.sh

REPO_NAME=test_clone_$TAG

gh repo create \
    $REPO_NAME \
    --private \
    --add-readme


twinkle clone git@github.com:$ACCOUNT/$REPO_NAME

cd $REPO_NAME
touch NEW_FILE
touch NEW_FILE2
touch NEW_FILE3
TWINKLE_ONCE=1 twinkle sync


source ../common/test_synced.sh
cd ..
source ./common/cleanup.sh
