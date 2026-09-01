#!/usr/bin/env bash

set -euo pipefail
source ../common/config.sh
source ../common/install_keys.sh

REPO_NAME=test_lfs_clone_$TAG

gh repo create \
    $REPO_NAME \
    --private \
    --add-readme

git clone git@github.com:$ACCOUNT/$REPO_NAME

cd $REPO_NAME

LARGE_FILE=large_file.bin
dd if=/dev/zero of=$LARGE_FILE bs=1M count=1
git lfs install
git lfs track $LARGE_FILE
git add $LARGE_FILE
git add .gitattributes
git config user.email "Test Bot"
git config user.name "ci@localhost"
git commit -m "Add large file"
git push origin main

cd ..
rm -Rf $REPO_NAME

twinkle clone \
    git@github.com:$ACCOUNT/$REPO_NAME \
    .

cd $REPO_NAME

test -f .gitattributes
test -f $LARGE_FILE
[ $(wc -c < $LARGE_FILE) -eq 1048576 ]
git config twinkle.lfs.enabled
git lfs ls-files | grep $LARGE_FILE

source ../../common/test_synced.sh
twinkle check

cd ..
source ../common/cleanup.sh
