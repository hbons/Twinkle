#!/usr/bin/env bash

#   Twinkle, automatic syncing with Git
#   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
#
#   This program is free software: you can redistribute it and/or modify it
#   under the terms of the GNU General Public License v3 or any later version.


export ACCOUNT=sparkleshare-org
export KEY_FILE=~/.ssh/ed25519
export TAG="${GITHUB_REF_NAME}_${GITHUB_SHA}_${RUNNER_ARCH}"
export DEBUG=1
