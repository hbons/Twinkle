#!/usr/bin/env bash

LOCAL=$(git rev-parse HEAD)
REMOTE=$(git ls-remote origin HEAD | cut -f1)
[[ "$LOCAL" == "$REMOTE" ]]
