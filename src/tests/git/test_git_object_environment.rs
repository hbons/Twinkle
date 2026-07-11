//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::path::Path;
use crate::git::objects::environment::GitEnvironment;


#[test]
fn test_git_object_environment_new() {
    let path = Path::new("/test/path");
    let git = GitEnvironment::new(path);

    assert_eq!(git.working_dir, path);
}


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


#[test]
fn test_git_object_environment_run_with_env() {
    let git = GitEnvironment::default();
    let env: Vec<(String, String)> = vec![
        ("GIT_CONFIG_PARAMETERS".into(), "'user.name=Test Bot'".into()),
    ];

    let result = git.run_with_env("config", &["user.name"], env);
    assert_eq!(result.unwrap().stdout.trim(), "Test Bot");
}


#[test]
fn test_git_object_environment_get_environment() {
    let git = GitEnvironment::default();
    let env = git.get_environment();

    assert_eq!(env.len(), 8);
}
