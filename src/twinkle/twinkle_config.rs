//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::path::{ Path, PathBuf };
use std::str;

use crate::config::{ config_init, config_load, config_load_string, config_save };

use crate::git::objects::repository::GitRepository;
use crate::git::objects::user::{ GitUserName, GitUserEmail };


#[derive(Debug)]
pub struct TwinkleConfig {
    config_path: PathBuf,
    loaded_repos: Vec<GitRepository>,
}


impl TwinkleConfig {
    // New, Find, Add, Remove, List

    pub fn new(config_path: &Path) -> Self {
        let mut config = TwinkleConfig {
            config_path: config_path.to_path_buf(),
            loaded_repos: Vec::new()
        };

        _ = config.load();
        config
    }


    pub fn find(&mut self, path: &Path) -> Result<&mut GitRepository, Box<dyn Error>> {
        self.loaded_repos
            .iter_mut()
            .find(|repo| repo.path == path)
            .ok_or_else(|| "Path not found in config".into())
    }


    pub fn add(&mut self, repo: &GitRepository) -> Result<(), Box<dyn Error>> {
        if self.loaded_repos.iter().any(|r| r.path == repo.path) {
            return Err(format!("Path `{}` already in config", repo.path.display()).into());
        }

        self.loaded_repos.push(repo.clone());
        self.save()?;
        Ok(())
    }


    pub fn remove(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        self.loaded_repos.retain(|repo| repo.path != path);
        self.save()
    }


    pub fn list(&self) -> Result<Vec<GitRepository>, Box<dyn Error>> {
        Ok(self.loaded_repos.clone())
    }
}


impl TwinkleConfig {
    // Properties

    pub fn set_user(&mut self, path: &Path, name: Option<&str>, email: Option<&str>) -> Result<(), Box<dyn Error>> {
        let repo = self.find(path)?;

        if let Some(name) = name {
            repo.user.name = GitUserName::new(name.into())?;
        }

        if let Some(email) = email {
            repo.user.email = GitUserEmail::new(email.into())?;
        }

        self.save().map_err(|_| "Could not set user")?;
        Ok(())
    }


    pub fn set_interval(&mut self, path: &Path, interval: u64) -> Result<(), Box<dyn Error>>{
        self.find(path)?.polling_interval = Some(interval);
        self.save().map_err(|_| "Could not set interval")?;
        Ok(())
    }
}


impl str::FromStr for TwinkleConfig {
    type Err = Box<dyn Error>;

    fn from_str(lines: &str) -> Result<Self, Self::Err> {
        Ok(TwinkleConfig {
            config_path: None,
            loaded_repos: config_load_string(&lines)?,
        })
    }
}


impl TwinkleConfig {
    // Load, Save

    pub fn load(&mut self) -> Result<(), Box<dyn Error>> {
        self.loaded_repos = config_load(&self.config_path)?;
        Ok(())
    }


    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        if !self.config_path.exists() {
            config_init(&self.config_path)?;
        }

        config_save(&self.config_path, self.loaded_repos.clone())?;
        Ok(())
    }
}
