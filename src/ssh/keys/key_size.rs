//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::fmt;
use super::key_type::KeyType;


pub enum KeySize {
    Bits256,
    Bits384,
    Bits512,
    Bits1024,
    Bits2048,
    Bits4096,
    Bits8192,
}


impl KeySize {
    pub fn default(key_type: KeyType) -> KeySize {
        match key_type {
            KeyType::ED25519 => KeySize::Bits256,
            KeyType::RSA     => KeySize::Bits4096,
            KeyType::ECDSA   => KeySize::Bits256,
        }
    }
}


impl fmt::Display for KeySize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Bits256  => 256.to_string(),
            Self::Bits384  => 384.to_string(),
            Self::Bits512  => 512.to_string(),
            Self::Bits1024 => 1024.to_string(),
            Self::Bits2048 => 2048.to_string(),
            Self::Bits4096 => 4096.to_string(),
            Self::Bits8192 => 8192.to_string(),
        })
    }
}
