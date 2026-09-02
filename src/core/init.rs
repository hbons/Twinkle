//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fs;
use std::path::Path;

use crate::ssh::keys::key_pair::KeyPair;
use crate::ssh::objects::url::SshUrl;
use crate::git::objects::environment::GitEnvironment;
use crate::git::objects::user::GitUser;

use super::defaults::common::COMMON_CONFIG_FILE;
use super::defaults::common::{ COMMON_FIRST_COMMIT_MESSAGE, COMMON_FIRST_FILE };
use super::defaults::config;
use super::defaults::config::{ K_ID, key };
use super::defaults::info;

use super::objects::repository::TwinkleRepository;
use super::util;


pub fn init_repo(
    path: &Path,
    remote_url: &SshUrl,
    key_pair: Option<&KeyPair>,
) -> Result<TwinkleRepository, Box<dyn Error>>
{
    let branch = {
        let git = GitEnvironment::new(path);

        if git.rev_parse_show_toplevel().is_ok() {
            return Err("Already inside a Git repository".into());
        }

        git.init()?
    };

    let repo = TwinkleRepository::new(path)?;
    let remote = repo.remote(&branch);
    repo.set_remote_url(&remote, remote_url)?;

    init_common(&repo, key_pair)?;
    init_id(&repo)?;
    init_first_commit(&repo)?;

    Ok(repo)
}


pub fn init_id( // TODO: Move to TwinkleRepository
    repo: &TwinkleRepository,
) -> Result<(), Box<dyn Error>>
{
    let config_file = Path::new(COMMON_CONFIG_FILE);
    let abs_config_file = repo.git.working_dir.join(config_file);

    let id = // TODO: validate type, twinkleID/SHA256
        if abs_config_file.exists() {
            repo.git.config_get_with_file(
                &key(K_ID),
                config_file,
            ).ok_or("Missing .id")?
        } else {
            if let Some(parent) = abs_config_file.parent() {
                fs::create_dir_all(parent)?;
            }

            let new_id = util::random_id()?;

            repo.git.config_set_with_file(
                &key(K_ID),
                &new_id,
                config_file,
            )?;

            repo.git.add(config_file)?;
            new_id
        };

    repo.set_id(&id)
}


pub fn init_first_commit(
    repo: &TwinkleRepository,
) -> Result<(), Box<dyn Error>>
{
    fs::write(
        repo.git.working_dir.join(COMMON_FIRST_FILE),
        init_welcome(&repo.remote_url().ok_or("Missing remote.url")?)
    )?;

    let config_file = Path::new(COMMON_CONFIG_FILE);

    if repo.abs_path(config_file).exists() {
        repo.git.add(config_file)?;
    }

    repo.git.add(Path::new(COMMON_FIRST_FILE))?;

    repo.git.commit(
        Some(GitUser::default()),
        COMMON_FIRST_COMMIT_MESSAGE
    )
}


pub fn init_common(
    repo: &TwinkleRepository,
    key_pair: Option<&KeyPair>,
) -> Result<(), Box<dyn Error>>
{
    repo.set_enabled(true)?;
    repo.set_user(
        &GitUser {
            key_pair: key_pair.cloned(),
            ..Default::default()
        }
    )?;

    if key_pair.is_some() {
        repo.set_core_ssh_command(key_pair)?;
    }

    init_config(repo)?;
    init_info_attributes(repo)?;
    init_info_exclude(repo)?;
    init_lfs(repo)?;

    Ok(())
}


/// .git/config
fn init_config(
    repo: &TwinkleRepository,
) -> Result<(), Box<dyn Error>>
{
    for (option, value) in config::default_git_settings() {
        repo.git.config_set(option, &value)?;
    }

    Ok(())
}


/// .git/info/attributes
fn init_info_attributes(
    repo: &TwinkleRepository,
) -> Result<(), Box<dyn Error>>
{
    let rules = info::default_info_attributes();
    repo.write_attribute_rules(&rules)
}


/// .git/info/exclude
fn init_info_exclude(
    repo: &TwinkleRepository,
) -> Result<(), Box<dyn Error>>
{
    let rules = info::default_info_exclude();
    repo.write_exclude_rules(&rules)
}


/// LFS
fn init_lfs(
    repo: &TwinkleRepository,
) -> Result<(), Box<dyn Error>>
{
    let git_ssh_command =
        if repo.user().is_some_and(|u| u.key_pair().is_some()) {
            Some(repo.git.GIT_SSH_COMMAND.clone())
        } else {
            None
        };

    if repo.lfs_enabled() {
        repo.git.lfs_config_set_filter(git_ssh_command.clone())?;
    }

    repo.git.lfs_install_pre_push_hook(git_ssh_command.clone())?;

    Ok(())
}


pub fn init_welcome(url: &SshUrl) -> String {
    format!(
        "# Hello!\nSync with `{}` was successfully set up.",
        url.original
    )
}
