#!/usr/bin/env bash

set -euo pipefail
source ./common/config.sh
source ./common/install_keys.sh

REPO_NAME=test_clone_$TAG

gh repo create \
    $REPO_NAME \
    --private \
    --add-readme

twinkle clone \
    git@github.com:$ACCOUNT/$REPO_NAME \
    .

cd $REPO_NAME
touch NEW_FILE
touch NEW_FILE1
touch NEW_FILE2
TWINKLE_MAX_ATTEMPTS=4 twinkle sync

source ../common/test_synced.sh
twinkle check

cd ..
source ./common/cleanup.sh
