#!/usr/bin/env bash

set -euo pipefail
source ./common/config.sh
source ./common/install_keys.sh

REPO_NAME=test_init_$TAG

gh repo create \
    $REPO_NAME \
    --private

touch README.md

twinkle init \
    git@github.com:$ACCOUNT/$REPO_NAME \
    .

TWINKLE_MAX_ATTEMPTS=4 twinkle sync

source ./common/test_synced.sh
twinkle check

source ./common/cleanup.sh
