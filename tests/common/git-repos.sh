#!/usr/bin/env bash

#   Twinkle, automatic syncing with Git
#   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
#
#   This program is free software: you can redistribute it and/or modify it
#   under the terms of the GNU General Public License v3 or any later version.

#   ～ Lovingly Handcrafted Code ～


set -euo pipefail

GIT_USER=git
REPO_COUNT=128
TMP_REPO=tmp_repo
README=README.md

git config --global user.name "$GIT_USER"
git config --global user.email "$GIT_USER@localhost"

cd /home/$GIT_USER

for i in $(seq 1 "$REPO_COUNT"); do
    REPO_NAME="repo_empty_$i"
    git init --quiet --bare --initial-branch=main "$REPO_NAME"
    ln -s "/home/$GIT_USER/$REPO_NAME" "/$REPO_NAME"

    REPO_NAME="repo_readme_$i"
    git init --quiet --bare --initial-branch=main "$REPO_NAME"
    ln -s "/home/$GIT_USER/$REPO_NAME" "/$REPO_NAME"


    git clone --quiet "$REPO_NAME" "$TMP_REPO"
    cd "$TMP_REPO"

    touch "$README"
    git add "$README"
    git commit --message "Add $README"
    git push --quiet origin main

    cd ..
    rm -rf "$TMP_REPO"
done

setfacl -R -d -b "/home/$GIT_USER"
chmod -R 755 "/home/$GIT_USER"
chown -R "$GIT_USER:$GIT_USER" "/home/$GIT_USER"
