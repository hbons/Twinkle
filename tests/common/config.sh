#!/usr/bin/env bash

export ACCOUNT=sparkleshare-org
export KEY_FILE=~/.ssh/ed25519
export TAG="${GITHUB_REF_NAME}_${GITHUB_SHA}_${RUNNER_ARCH}"
export DEBUG=1
