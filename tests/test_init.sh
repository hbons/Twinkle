#!/usr/bin/env bash

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
