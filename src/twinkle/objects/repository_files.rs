//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use super::repository::TwinkleRepository;


impl TwinkleRepository {
    pub fn unsynced_files(&self) -> Vec<String> {
        vec![] // TODO: git log origin/main..HEAD --name-status
    }


    pub fn ignored_files(&self) -> Vec<String> {
        vec![] // TODO: ls-files --ignored --exclude-standard --directory --others
    }


    pub fn deleted_files(&self) -> Vec<String> {
        vec![] // TODO: git log --diff-filter=D --summary
    }


    pub fn hidden_files(&self) -> Vec<String> {
        vec![] // TODO: git ls-tree HEAD
    }


    pub fn all_files(&self) -> Vec<String> {
        _ = self.git.ls_files();
        vec![] // TODO: ls-tree
    }
}
