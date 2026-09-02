#!/usr/bin/env bash

set -euo pipefail
source ./common/config.sh

! TWINKLE_ONCE=1 twinkle sync
