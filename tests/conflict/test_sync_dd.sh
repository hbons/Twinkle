#!/usr/bin/env bash

set -euo pipefail
source ./common/config.sh
source ./common/install_keys.sh

REPO_NAME=test_sync_dd_$TAG
REPO_NAME_1=test_sync_dd_"$TAG"_1
REPO_NAME_2=test_sync_dd_"$TAG"_2

gh repo create \
    $REPO_NAME \
    --private \
    --add-readme


DEBUG=1 twinkle clone \
    git@github.com:$ACCOUNT/$REPO_NAME

mv $REPO_NAME $REPO_NAME_1

DEBUG=1 twinkle clone \
    git@github.com:$ACCOUNT/$REPO_NAME

mv $REPO_NAME $REPO_NAME_2


cd $REPO_NAME_2
rm README.md
TWINKLE_ONCE=1 twinkle sync
cd ..

cd $REPO_NAME_1
rm README.md
TWINKLE_ONCE=1 twinkle sync

! test -f README.md


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
