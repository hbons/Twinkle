#!/usr/bin/env bash

#   SparkleShare, sync files with Git
#   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
#
#   This program is free software: you can redistribute it and/or modify it
#   under the terms of the GNU General Public License v3 or any later version.

#   ～ Lovingly Handcrafted Code ～


set -euo pipefail
source ./common/test_begin.sh


REPO_NAME="repo_empty_5"
REPO_NAME_1="${REPO_NAME}_1"
REPO_NAME_2="${REPO_NAME}_2"

sparkleshare clone \
    "$HOST/$REPO_NAME" \
    "$REPO_NAME_1"

sparkleshare clone \
    "$HOST/$REPO_NAME" \
    "$REPO_NAME_2"

cd "$REPO_NAME_1"
touch README.md
SPARKLESHARE_ONCE=1 sparkleshare sync
cd ..

cd "$REPO_NAME_2"
touch script.sh
chmod +x script.sh
SPARKLESHARE_ONCE=1 sparkleshare sync
cd ..

cd "$REPO_NAME_1"
SPARKLESHARE_ONCE=1 sparkleshare sync

test -f README.md
test -x script.sh


cd ..
source ./common/test_end.sh
