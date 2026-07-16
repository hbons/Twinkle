#!/usr/bin/env bash

set -euo pipefail
source ./common/config.sh
source ./common/install_keys.sh

REPO_NAME=test_init_$TAG

gh repo create \
    $REPO_NAME \
    --private

touch README.md

DEBUG=1 twinkle init \
    git@github.com:$ACCOUNT/$REPO_NAME \
    .

DEBUG=1 timeout 20s twinkle sync || true  # --once

source ./common/cleanup.sh
