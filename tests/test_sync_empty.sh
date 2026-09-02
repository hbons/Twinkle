#!/usr/bin/env bash

set -euo pipefail
source ./common/config.sh

! test -d .git
! TWINKLE_ONCE=1 twinkle sync
