#!/usr/bin/env bash

#   Twinkle, automatic syncing with Git
#   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
#
#   This program is free software: you can redistribute it and/or modify it
#   under the terms of the GNU General Public License v3 or any later version.


set -euo pipefail
source ./common/config.sh
source ./common/install_keys.sh

REPO_NAME=test_init_$TAG

gh repo create \
    $REPO_NAME \
    --private \
    --add-readme

mkdir $REPO_NAME
cd $REPO_NAME

touch README2.md

twinkle init \
    git@github.com:$ACCOUNT/$REPO_NAME \
    .

TWINKLE_ONCE=1 twinkle sync

test -f README.md
test -f README2.md

! twinkle init \
    git@github.com:$ACCOUNT/$REPO_NAME \
    .

source ../common/test_synced.sh
source ../common/cleanup.sh
