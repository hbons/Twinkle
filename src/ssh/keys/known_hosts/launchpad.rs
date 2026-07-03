//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use super::super::fingerprint::Fingerprint;
use super::super::host_key::HostKey;
use super::super::key_type::KeyType;


pub fn ssh_hostkey_launchpad() -> HostKey {
    // Last updated: July 3, 2026
    // Source: https://ubuntu.com/docs/launchpad/user/reference/ssh-fingerprints

    HostKey {
        host: "git.launchpad.net".into(),
        is_trusted: true,
        key_type: KeyType::RSA,
        public_key: "AAAAB3NzaC1yc2EAAAADAQABAAABAQDEFREwBD2ye2Xrc2SVcUmmJ44MF1BCB3W11NTaiqzVj7XZnQmgWZk9UadHVY2wBXvelcDO51MPN5ozJjFAknw09rP7XMRJMlAOLSIVoU6DRF1u1j8kJVY+dfiDHheS7+siADnrmb8HGn2xQQ6EJDjAXrw1x58x5eZjQ0PFWdI+pRTdYGvWkpHdXKFO6a9/lDx4uo9MCnePEGi/QnkCmKqLCBUlYNZYRiB8nVee2tMF0mjV8xk1rJ+/UP+897+FXFR9w/B1EPRjiQ35ZNQZKPP4isxPtyMuCQkZY7ckWr5YsylNfvNcyGDnO1XazZhJ71rzDpi1RmnFXBW5i+2dm2y7".into(),
        fingerprint: Some(Fingerprint::SHA256("UNOzlP66WpDuEo34Wgs8mewypV0UzqHLsIFoqwe8dYo".into())),
    }
}
