#!/usr/bin/env bash

set -euo pipefail
source ./common/config.sh
source ./common/install_keys.sh

REPO_NAME=test_sync_$TAG
REPO_NAME_1=test_sync_"$TAG"_1
REPO_NAME_2=test_sync_"$TAG"_2

gh repo create \
    $REPO_NAME \
    --private \
    --add-readme


twinkle clone \
    git@github.com:$ACCOUNT/$REPO_NAME \
    .

mv $REPO_NAME $REPO_NAME_1
cd $REPO_NAME_1
# timeout 15s twinkle sync || true  # --once
cd ..

DEBUG=1 twinkle clone \
    git@github.com:$ACCOUNT/$REPO_NAME \
    .

mv $REPO_NAME $REPO_NAME_2

cd $REPO_NAME_2
echo " ...a conflict!" >> README.md
timeout 15s twinkle sync || true  # --once
cd ..

cd $REPO_NAME_1
echo " Let's create..." >> README.md
timeout 45s twinkle sync || true  # --once

echo "--- README.md ---"
cat README.md
printf '\n'
echo "--- README (A).md ---"
cat "README (A).md"
echo "--- README (B).md ---"
cat "README (B).md"
echo "---"

test -f README.md
test -f "README (A).md"
test -f "README (B).md"


# TODO: Test all conflict paths: AA, UU, AU, UA, DU, UD, DD, XX, QQ


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
