#!/usr/bin/env bash

#   SparkleShare, sync files with Git
#   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
#
#   This program is free software: you can redistribute it and/or modify it
#   under the terms of the GNU General Public License v3 or any later version.

#   ～ Lovingly Handcrafted Code ～


set -euo pipefail
source ./common/test_begin.sh


! test -d .git
! SPARKLESHARE_ONCE=1 sparkleshare sync


source ./common/test_end.sh
