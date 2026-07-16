#!/usr/bin/env bash

set -euo pipefail
source ./common/config.sh
source ./common/install_keys.sh

REPO_NAME=test_sync_$TAG
REPO_NAME_1=test_sync_{$TAG}1
REPO_NAME_2=test_sync_{$TAG}2

gh repo create \
    $REPO_NAME \
    --private \
    --add-readme


twinkle clone \
    git@github.com:$ACCOUNT/$REPO_NAME \
    .

mv $REPO_NAME $REPO_NAME_1
cd $REPO_NAME_1
timeout 15s twinkle sync || true  # --once

cd ..

DEBUG=1 twinkle clone \
    git@github.com:$ACCOUNT/$REPO_NAME \
    .

mv $REPO_NAME $REPO_NAME_2


cd $REPO_NAME_1
echo "Let's create..." >> README.md
timeout 15s twinkle sync || true  # --once

cd ..

cd $REPO_NAME_2
echo "...a conflict!" >> README.md
timeout 15s twinkle sync || true  # --once

ls

test -f README.md
test -f "README (A).md"
test -f "README (B).md"

# source ../common/test_synced.sh

cd ..
source ./common/cleanup.sh
