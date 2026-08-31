#!/usr/bin/env bash

set -euo pipefail
source ../common/config.sh
source ../common/install_keys.sh

REPO_NAME=test_lfs_sync_$TAG

gh repo create \
    $REPO_NAME \
    --private \
    --add-readme

twinkle clone \
    git@github.com:$ACCOUNT/$REPO_NAME \
    .

cd $REPO_NAME

SMALL_FILE=small_file.txt
LARGE_FILE=large_file.bin
touch $SMALL_FILE
dd if=/dev/zero of=$LARGE_FILE bs=3M count=1

git config twinkle.lfs.enabled true
git config twinkle.lfs.sizeThreshold 3m
timeout 20s twinkle sync || true  # --once

test -f $SMALL_FILE
test -f $LARGE_FILE
test -f .gitattributes
! git lfs ls-files | grep $SMALL_FILE
git lfs ls-files | grep $LARGE_FILE

source ../../common/test_synced.sh
twinkle check

cd ..
source ../common/cleanup.sh
