//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use crate::git::objects::environment::GitEnvironment;


#[test]
fn test_git_object_environment_run() {
    let git = GitEnvironment::default();

    let result = git.run("version", &["--build-options"]);
    assert!(result.is_ok());

    let output = result.unwrap();
    assert_eq!(output.exit_code, 0);
    assert!(output.stdout.starts_with("git version"));

    let result = git.run("versiasdfgasdfg", &["--build-options"]);
    assert!(result.is_err());
}
