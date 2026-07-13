//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use crate::twinkle::objects::repository::TwinkleRepository;


impl TwinkleRepository {
    pub fn ignored_files(&self) -> Vec<String> {
        vec![]
    }


    pub fn hidden_files(&self) -> Vec<String> {
        vec![]
    }


    pub fn all_files(&self) -> Vec<String> {
        _ = self.git.ls_files(); // ls-tree?
        vec![]
    }
}
