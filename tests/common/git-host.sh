#!/usr/bin/env bash

#   Twinkle, automatic syncing with Git
#   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
#
#   This program is free software: you can redistribute it and/or modify it
#   under the terms of the GNU General Public License v3 or any later version.

#   ～ Lovingly Handcrafted Code ～


set -euo pipefail

GIT_USER=git

useradd -m -s /bin/bash "$GIT_USER"

mkdir -p "/home/$GIT_USER/.ssh"
chmod 700 "/home/$GIT_USER/.ssh"
chown -R "$GIT_USER:$GIT_USER" "/home/$GIT_USER/.ssh"

touch "/home/$GIT_USER/.ssh/authorized_keys"
chmod 600 "/home/$GIT_USER/.ssh/authorized_keys"
chown "$GIT_USER:$GIT_USER" "/home/$GIT_USER/.ssh/authorized_keys"

mkdir -p /var/run/sshd

/usr/sbin/sshd \
    -p 2222 \
    -o PermitRootLogin=no \
    -o PasswordAuthentication=no

sleep 3

ssh-keyscan \
    -p 2222 \
    -t ed25519 \
    localhost
