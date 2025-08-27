//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fs;
use std::path::{ Path, PathBuf };

use crate::git::objects::change::GitChange;
use crate::git::objects::merge_status::GitMergeStatus;
use crate::git::objects::user::GitUser;

use crate::log;

use super::twinkle_lfs::twinkle_lfs_track;
use super::objects::twinkle_repository::TwinkleRepository;


pub fn twinkle_resolve_changes(repo: &TwinkleRepository) -> Result<(), Box<dyn Error>> {
    log::info("Resolving conflicts…");

    for change in repo.git.status()? {
        twinkle_resolve(repo, &change)?;
    }

    repo.git.commit(&repo.user, "Twinkle: Resolve conflicts")?;
    log::info("Conflicts resolved");

    Ok(())
}


pub fn twinkle_resolve(repo: &TwinkleRepository, change: &GitChange) -> Result<(), Box<dyn Error>> {
    // Docs: https://git-scm.com/docs/git-merge#_how_to_resolve_conflicts

    let merge_status = change.as_merge_status();
    let path = &change.path;

    if let Some(status) = &merge_status {
        log::info(&format!("Resolve | {status} | {}", path.display()));
    }

    match merge_status {
        Some(status) => match status {
            GitMergeStatus::AA | GitMergeStatus::AU |
            GitMergeStatus::UA | GitMergeStatus::UU => {
                let their_user = repo.git.merge_blame(path)?;
                let (ours, theirs) = twinkle_resolve_path_names(path, &repo.user, &their_user)?;

                repo.git.checkout_ours(path)?;
                fs::rename(repo.path(path), repo.path(&ours))?;

                repo.git.checkout_theirs(path)?;
                fs::rename(repo.path(path), repo.path(&theirs))?;

                repo.git.checkout_original(path)?;

                repo.git.add(&ours)?;
                repo.git.add(&theirs)?;
                repo.git.add(path)?;
            },
            GitMergeStatus::UD => {
                repo.git.checkout_ours(&change.path)?;
                repo.git.add(&change.path)?
            },
            GitMergeStatus::DU => repo.git.add(&change.path)?, // Our version is checked out
            GitMergeStatus::DD => ( /* Nothing to do */ ),
            GitMergeStatus::QQ => ( /* Nothing to do */ ),
            GitMergeStatus::XX => ( /* Nothing to do */ ),
        },
        None => log::debug(&format!("Resolve | No conflict at {}", change.path.display())),
    }

    if repo.large_file_storage {
        for change in repo.git.status()? {
            _ = twinkle_lfs_track(repo, &change);
        }
    }

    Ok(())
}


/// Generates unique names for ours/theirs names of a path
pub fn twinkle_resolve_path_names(path: &Path, our_user: &GitUser, their_user: &GitUser) -> Result<(PathBuf, PathBuf), Box<dyn Error>> {
    let mut clue_a = our_user.name().to_string();
    let mut clue_b = their_user.name().to_string();

    if our_user.name() == their_user.name() {
        clue_a = "A".to_string();
        clue_b = "B".to_string();
    }

    let file_name = path.file_stem().ok_or("Could not find file stem")?;
    let file_name = file_name.to_str().ok_or("File name is not valid UTF-8")?;

    let mut file_name_a = format!("{file_name} ({clue_a})");
    let mut file_name_b = format!("{file_name} ({clue_b})");

    if let Some(extension) = path.extension() {
        let ext = extension.to_str().ok_or("Could not parse extension")?;
        file_name_a.push_str(&format!(".{}", ext));
        file_name_b.push_str(&format!(".{}", ext));
    }

    if let Some(parent) = path.parent() {
        return Ok((parent.join(file_name_a),
                   parent.join(file_name_b)))
    }

    Ok((PathBuf::from(file_name_a),
        PathBuf::from(file_name_b)))
}
