//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;

use crate::ssh::keys::key_pair::KeyPair;
use crate::ssh::objects::config::SshConfig;

use super::objects::environment::GitEnvironment;
use super::objects::output::GitOutput;
use super::objects::user::GitUser;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-config

    pub fn config_get(&self, name: &str) -> Result<GitOutput, Box<dyn Error>> {
        self.run("config", &[name]) // Deprecated in Git 2.44+, later use:
        // self.run("config", &["get", name])
    }


    pub fn config_set(&self, name: &str, value: &str) -> Result<GitOutput, Box<dyn Error>> {
        self.run("config", &[name, value]) // Deprecated in Git 2.44+, later use:
        // self.run("config", &["set", name, value])
    }
}


impl GitEnvironment {
    pub fn config_set_user(&self, user: &GitUser) -> Result<(), Box<dyn Error>>{
        self.config_set("user.name", user.name())?;
        self.config_set("user.email", user.email())?;

        Ok(())
    }


    pub fn config_set_user_signing_key(&self, key_pair: &KeyPair) -> Result<(), Box<dyn Error>>{
        let key_path = &key_pair.private_key_path.to_string_lossy();
        self.config_set("user.signingKey", key_path)?;

        Ok(())
    }


    // Write a minimal SSH command to the .git/config for debugging purposes
    pub fn config_set_core_ssh_command(&self, key_pair: &KeyPair) -> Result<(), Box<dyn Error>>{
        let config = SshConfig {
            IdentityFile: key_pair.private_key_path.clone(),
            UserKnownHostsFile: key_pair.private_key_path.with_extension("key.host"),
            ..Default::default()
        };

        self.config_set("core.sshCommand",
            &format!("ssh -F /dev/null {config}"))?;

        Ok(())
    }
}
