//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::ffi::{ OsStr, OsString };
use std::os::unix::ffi::OsStrExt;

use crate::log;

use super::objects::change::GitChange;
use super::objects::environment::GitEnvironment;


impl GitEnvironment {
    // Docs: https://git-scm.com/docs/git-status

    pub fn status(&self) -> Result<Vec<GitChange>, Box<dyn Error>> {
        let changes = self.get_changes(
            Some(OsStr::new("--untracked-files=normal")),
        )?;

        Ok(changes)
    }


    fn get_changes(
        &self,
        extra_arg: Option<&OsStr>,
    ) -> Result<Vec<GitChange>, Box<dyn Error>>
    {
        let output = self.run("status", &[
            OsStr::new("--porcelain=v1"),
            OsStr::new("-z"), // Single line, NUL-separated
            extra_arg.unwrap_or_default(),
        ])?;

        let line: Vec<&OsStr> = output.stdout
            .split(|&b| b == 0)
            .filter(|chunk| !chunk.is_empty())
            .map(OsStr::from_bytes)
            .collect();

        let mut changes = vec![];
        let mut chunk_iter = line.iter();
        let mut buf = OsString::new();

        while let Some(&chunk) = chunk_iter.next() {
            let mut byte_iter = chunk.as_bytes().iter();
            let x = byte_iter.next().ok_or("Missing status code X")?;
            let y = byte_iter.next().ok_or("Missing status code Y")?;

            // For renames/copies, join the next chunk (ORIG_PATH)
            let chunk =
                if *x == b'R' || *y == b'R' ||
                   *x == b'C' || *y == b'C' {
                    if let Some(orig_path) = chunk_iter.next() {
                        buf.clear();
                        buf.push(chunk);
                        buf.push(OsStr::new("\0"));
                        buf.push(orig_path);
                        &buf
                    } else {
                        chunk
                    }
                } else {
                    chunk
                };

            match GitChange::from_status_line(chunk) {
                Ok(change) => changes.push(change),
                Err(e) => log::error(
                    &format!("{e}: `{}`", chunk.to_string_lossy())
                ),
            }
        }

        Ok(changes)
    }
}
