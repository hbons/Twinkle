#!/usr/bin/env bash

set -euo pipefail
source ./common/config.sh
source ./common/install_keys.sh

REPO_NAME=test_sync_$TAG
REPO_NAME_1=test_sync_"$TAG"_1
REPO_NAME_2=test_sync_"$TAG"_2

gh repo create \
    $REPO_NAME \
    --private


twinkle clone \
    git@github.com:$ACCOUNT/$REPO_NAME \
    $REPO_NAME_1

cd $REPO_NAME_1
touch README.md
TWINKLE_ONCE=1 twinkle sync
echo 1
ls

cd ..


twinkle clone \
    git@github.com:$ACCOUNT/$REPO_NAME \
    $REPO_NAME_2

cd $REPO_NAME_2
touch README2.md
TWINKLE_ONCE=1 twinkle sync
echo 2
ls
test -f README.md
test -f README2.md

cd ..


cd $REPO_NAME_1
TWINKLE_ONCE=1 twinkle sync
echo 3
ls
test -f README.md
test -f README2.md


# TODO: Doesn't work...
# source ../common/test_synced.sh
LOCAL=$(git rev-parse HEAD)
REMOTE=$(git ls-remote origin HEAD | cut -f1)
[[ "$LOCAL" == "$REMOTE" ]]


rm -Rf "$REPO_NAME"
rm -Rf "$REPO_NAME_1"
rm -Rf "$REPO_NAME_2"

cd ..
source ./common/cleanup.sh
