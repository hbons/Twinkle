#!/usr/bin/env bash

#   Twinkle, automatic syncing with Git
#   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
#
#   This program is free software: you can redistribute it and/or modify it
#   under the terms of the GNU General Public License v3 or any later version.

#   ～ Lovingly Handcrafted Code ～


set -euo pipefail

KEY_FILE=~/.ssh/ed25519
USER=git

ssh-keygen \
    -f $KEY_FILE \
    -t ed25519 \
    -P ""

cat $KEY_FILE.pub | sudo tee -a /home/$USER/.ssh/authorized_keys
ssh-keyscan -p 2222 -t ed25519 localhost >> ~/.ssh/known_hosts
