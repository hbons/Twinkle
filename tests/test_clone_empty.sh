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
TWINKLE_ONCE=1 twinkle sync

! twinkle clone \
    git@github.com:$ACCOUNT/$REPO_NAME \
    .

source ../common/test_synced.sh
cd ..
source ./common/cleanup.sh
