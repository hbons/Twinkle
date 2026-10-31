//   Twinkle, sync files with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.

//   ～ Lovingly Handcrafted Code ～


use std::error::Error;
use std::fs::File;
use std::io::Write;

use crate::core::objects::repository::TwinkleRepository;
use crate::log;


impl TwinkleRepository {
    pub fn ignored_files(&self) -> Vec<String> {
        vec![]
    }


    pub fn hidden_files(&self) -> Vec<String> {
        vec![]
    }


    pub fn all_files(&self) -> Vec<String> {
        // _ = self.git.ls_files(); // ls-tree?
        vec![]
    }
}


impl TwinkleRepository {
    pub fn write_attribute_rules(&self, rules: &[String]) -> Result<(), Box<dyn Error>> {
        let attributes_path = self.git.working_dir.join(".git/info/attributes");
        let mut buffer = File::create(&attributes_path)?;
        buffer.write_all(rules.join("\n").as_bytes())?;

        log::debug(&format!("Repository | Created `{}`", attributes_path.to_string_lossy()));

        Ok(())
    }


    pub fn write_exclude_rules(&self, rules: &[&str]) -> Result<(), Box<dyn Error>> {
        let exclude_path = self.git.working_dir.join(".git/info/exclude");
        let mut buffer = File::create(&exclude_path)?;
        buffer.write_all(rules.join("\n").as_bytes())?;

        log::debug(&format!("Repository | Created `{}`", exclude_path.to_string_lossy()));

        Ok(())
    }
}
