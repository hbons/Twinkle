#!/usr/bin/env bash

#   Twinkle, automatic syncing with Git
#   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
#
#   This program is free software: you can redistribute it and/or modify it
#   under the terms of the GNU General Public License v3 or any later version.

#   ～ Lovingly Handcrafted Code ～


export DEBUG=1
export HOST=ssh://git@localhost:2222
export KEY_FILE=~/.ssh/ed25519

eval "$(ssh-agent -k)" 2>/dev/null || true
eval "$(ssh-agent -s)"

sleep 3
ssh-add $KEY_FILE
