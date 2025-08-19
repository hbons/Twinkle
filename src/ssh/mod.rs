//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


pub mod keys {
    pub mod fingerprint;
    pub mod host_key;
    pub mod key_pair;
    pub mod key_size;
    pub mod key_type;

    pub mod hosts {
        pub mod bitbucket;
        pub mod codeberg;
        pub mod github;
        pub mod gitlab;
        pub mod gnome;
        pub mod sourcehut;
    }
}

pub mod objects {
    pub mod url;
}

pub mod keygen;
pub mod keyscan;
pub mod util;
pub mod version;
