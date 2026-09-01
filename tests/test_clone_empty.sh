#!/usr/bin/env bash

set -euo pipefail
source ./common/config.sh
source ./common/install_keys.sh

REPO_NAME=test_clone_empty_$TAG

gh repo create \
    $REPO_NAME \
    --private

twinkle clone \
    git@github.com:$ACCOUNT/$REPO_NAME \
    .

cd $REPO_NAME
TWINKLE_MAX_ATTEMPTS=4 twinkle sync

source ../common/test_synced.sh
test -f TWINKLE.md

twinkle check

cd ..
source ./common/cleanup.sh
