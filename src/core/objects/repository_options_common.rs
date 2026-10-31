//   Twinkle, sync files with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.

//   ～ Lovingly Handcrafted Code ～


use std::error::Error;

use crate::git::objects::user::GitUser;
use crate::ssh::keys::key_pair::KeyPair;
use crate::ssh::objects::config::SshConfig;
use crate::ssh::objects::url::SshUrl;

use super::config_option::ConfigOption;
use super::repository::TwinkleRepository;


// RemoteOriginUrl
impl TwinkleRepository {
    pub fn remote_url(&self) -> Option<SshUrl> { // TODO: use .remote()
        self.get_option_as::<SshUrl>(ConfigOption::RemoteOriginUrl)
    }
}


// UserName, UserEmail, and UserSigningKey
impl TwinkleRepository {
    pub fn user(&self) -> Option<GitUser> {
        GitUser::from(&self.path)
    }


    pub fn set_user(&self, value: &GitUser) -> Result<(), Box<dyn Error>>{
        self.set_option(ConfigOption::UserName, value.name())?;
        self.set_option(ConfigOption::UserEmail, value.email())?;

        if let Some(keys) = &value.key_pair {
            self.set_user_signing_key(keys)?;
        }

        Ok(())
    }

    pub fn set_user_signing_key(&self, keys: &KeyPair) -> Result<(), Box<dyn Error>>{
        let value = &keys.private_key_path.to_string_lossy();
        self.set_option(ConfigOption::UserSigningKey, value)
    }
}


// CoreIgnoreCase
impl TwinkleRepository {
    pub fn core_ignore_case(&self) -> bool {
        self.get_option_as::<bool>(ConfigOption::CoreIgnoreCase)
            .unwrap_or_default()
    }

    pub fn set_core_ignore_case(&self, value: bool) -> Result<(), Box<dyn Error>>{
        self.set_option(ConfigOption::CoreIgnoreCase, &value.to_string())
    }
}


// CommitGpgSign and CommitGpgSign
impl TwinkleRepository {
    pub fn set_commit_gpg_sign(&self, value: bool) -> Result<(), Box<dyn Error>>{
        self.set_option(ConfigOption::CommitGpgSign, &value.to_string())
    }

    pub fn set_tag_gpg_sign(&self, value: bool) -> Result<(), Box<dyn Error>>{
        self.set_option(ConfigOption::CommitGpgSign, &value.to_string())
    }
}


// CoreSshCommand
impl TwinkleRepository {
    // Write a minimal SSH command to the .git/config for debugging purposes
    pub fn set_core_ssh_command(&self, key_pair: Option<&KeyPair>) -> Result<(), Box<dyn Error>>{
        let config = match key_pair {
            Some(key_pair) => {
                SshConfig {
                    IdentitiesOnly: true,
                    IdentityFile: Some(key_pair.private_key_path.clone()),
                    UserKnownHostsFile: Some(key_pair.private_key_path.with_extension("key.host")),
                    ..Default::default()
                }
            },
            None => SshConfig::default(),
        };

        self.set_option(
            ConfigOption::CoreSshCommand,
            &format!("ssh -F /dev/null {config}")
        )
    }
}
