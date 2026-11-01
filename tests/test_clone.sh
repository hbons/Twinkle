#!/usr/bin/env bash

#   Twinkle, automatic syncing with Git
#   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
#
#   This program is free software: you can redistribute it and/or modify it
#   under the terms of the GNU General Public License v3 or any later version.

#   ～ Lovingly Handcrafted Code ～


set -euo pipefail
source ./common/test_begin.sh


REPO_NAME=repo_readme_1

twinkle clone "$HOST/$REPO_NAME"

cd $REPO_NAME
touch NEW_FILE
touch NEW_FILE2
touch NEW_FILE3

TWINKLE_ONCE=1 twinkle sync


cd ..
source ./common/test_end.sh
