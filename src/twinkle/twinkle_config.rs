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
    config_path: Option<PathBuf>,
    loaded_repos: Vec<GitRepository>,
}

#[cfg(test)]
impl TwinkleConfig {
    pub fn config_path(&self) -> Option<PathBuf> {
        self.config_path.clone()
    }

    pub fn loaded_repos(&self) -> &Vec<GitRepository> {
        &self.loaded_repos
    }
}


impl TwinkleConfig {
    // New, Find, Add, Remove, List

    pub fn new(config_path: &Path) -> Self {
        TwinkleConfig {
            config_path: Some(config_path.to_path_buf()),
            loaded_repos: Vec::new()
        }
    }


    pub fn find(&mut self, path: &Path) -> Result<&mut GitRepository, Box<dyn Error>> {
        self.loaded_repos
            .iter_mut()
            .find(|repo| repo.path == path)
            .ok_or_else(|| format!("Path not in config: `{}`", path.to_string_lossy()).into())
    }


    pub fn add(&mut self, repo: &GitRepository) -> Result<(), Box<dyn Error>> {
        if self.loaded_repos.iter().any(|r| r.path == repo.path) {
            return Err(format!("Path already in config: `{}`", repo.path.display()).into());
        }

        self.loaded_repos.push(repo.clone());
        self.save()?;

        Ok(())
    }


    pub fn remove(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        if self.find(path).is_err() {
            return Err(format!("Path not in config: `{}`", path.to_string_lossy()).into());
        }

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
        let config_path = &self.config_path.clone().ok_or("Missing config path")?;
        self.loaded_repos = config_load(config_path)?;

        Ok(())
    }


    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        match &self.config_path {
            Some(path) => {
                if !path.exists() {
                    config_init(&path)?;
                }

                config_save(path, self.loaded_repos.clone())
            },
            None => Ok(()),
        }
    }
}
