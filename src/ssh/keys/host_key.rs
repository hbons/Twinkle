//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use super::hosts::bitbucket::*;
use super::hosts::codeberg::*;
use super::hosts::github::*;
use super::hosts::gitlab::*;
use super::hosts::gnome::*;
use super::hosts::sourcehut::*;

use super::fingerprint::Fingerprint;
use super::key_type::KeyType;
use super::super::keyscan::ssh_keyscan;
use super::super::objects::url::SshUrl;


#[derive(Clone, Debug, Default)]
pub struct HostKey {
    pub host: String,
    pub is_trusted: bool,
    pub key_type: KeyType,
    pub public_key: String,
    pub fingerprint: Option<Fingerprint>,
}


impl HostKey {
    pub fn for_host(url: &SshUrl, key_type: KeyType) -> Result<HostKey, Box<dyn Error>> {
        if key_type == KeyType::ED25519 {
            match url.host.as_str() {
                "bitbucket.org"    => Ok(ssh_hostkey_bitbucket()),
                "codeberg.org"     => Ok(ssh_hostkey_codeberg()),
                "github.com"       => Ok(ssh_hostkey_github()),
                "gitlab.com"       => Ok(ssh_hostkey_gitlab()),
                "gitlab.gnome.org" => Ok(ssh_hostkey_gnome()),
                "git.sr.ht"        => Ok(ssh_hostkey_sourcehut()),
                _ => ssh_keyscan(url.host.as_str(), url.port, KeyType::ED25519),
            }
         } else {
            ssh_keyscan(url.host.as_str(), url.port, key_type)
         }
    }


    /// "github.com ssh-ed25519 AAAAC3NzaC1lZDI1N…"
    pub fn from_file(path: &Path) -> Result<HostKey, Box<dyn Error>> {
        let content = fs::read_to_string(path)?;
        let mut parts = content.split_whitespace();

        let host        = parts.next().ok_or("Missing host")?.to_string();
        let key_type    = parts.next().ok_or("Missing key type")?.to_string();
        let key_type    = key_type.trim_start_matches("ssh-").parse::<KeyType>()?;
        let public_key  = parts.next().ok_or("Missing public key")?.trim().to_string();

        let host_key = HostKey {
            host,
            is_trusted: true, // If it's local we trust it
            key_type,
            public_key,
            fingerprint: None
        };

        Ok(host_key)
    }


    /// "github.com.ed25519.key.host"
    pub fn to_file_name(&self) -> PathBuf {
        let name = &format!("{}.{}.key.host", self.host, self.key_type);
        PathBuf::from(name)
    }
}


impl fmt::Display for HostKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.key_type {
            KeyType::ECDSA => write!(f, "{} ecdsa-sha2-nistp256 {}", self.host, self.public_key),
            _              => write!(f, "{} ssh-{} {}",              self.host, self.key_type, self.public_key),
        }
    }
}
