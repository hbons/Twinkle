//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::path::PathBuf;
use crate::ssh::objects::url::{ SshUrl, SshUrlType };


#[test]
fn test_ssh_url_to_string() {
    let url_standard  = "ssh://git@github.com/hbons/Twinkle".parse::<SshUrl>().unwrap();
    let url_alternate = "git@github.com:hbons/Twinkle".parse::<SshUrl>().unwrap();

    assert_eq!(url_standard.form,  SshUrlType::Standard);
    assert_eq!(url_alternate.form, SshUrlType::Alternate);

    let no_user   = "ssh://github.com/hbons/Twinkle".parse::<SshUrl>();
    let no_scheme = "github.com/hbons/Twinkle".parse::<SshUrl>();
    let no_colon  = "git@github.com/hbons/Twinkle".parse::<SshUrl>();
    let no_path   = "ssh://git@github.com".parse::<SshUrl>();

    assert!(no_user.is_err());
    assert!(no_scheme.is_err());
    assert!(no_colon.is_err());
    assert!(no_path.is_err());
}


#[test]
fn test_ssh_url_from_str_standard() {
    let url = "ssh://git@github.com/hbons/Twinkle".to_string();
    let url = url.parse::<SshUrl>().unwrap();

    assert_eq!(url.form, SshUrlType::Standard);
    assert_eq!(url.user, "git");
    assert_eq!(url.host, "github.com");
    assert_eq!(url.port, None);
    assert_eq!(url.path, PathBuf::from("hbons/Twinkle"));


    let url = "ssh://git@github.com:22/hbons/Twinkle".to_string();
    let url = url.parse::<SshUrl>().unwrap();

    assert_eq!(url.form, SshUrlType::Standard);
    assert_eq!(url.user, "git");
    assert_eq!(url.host, "github.com");
    assert_eq!(url.port, Some(22));
    assert_eq!(url.path, PathBuf::from("hbons/Twinkle"));
}


#[test]
fn test_ssh_url_to_string_standard() {
    let url = SshUrl {
        form: SshUrlType::Standard,
        user: "git".to_string(),
        host: "github.com".to_string(),
        port: Some(22),
        path: PathBuf::from("hbons/Twinkle"),
        original: "ssh://git@github.com:22/hbons/Twinkle".to_string()
    };

    assert_eq!(url.to_string(), url.original);


    let url = SshUrl {
        form: SshUrlType::Standard,
        user: "git".to_string(),
        host: "github.com".to_string(),
        port: None,
        path: PathBuf::from("hbons/Twinkle"),
        original: "ssh://git@github.com/hbons/Twinkle".to_string()
    };

    assert_eq!(url.to_string(), url.original);
}


#[test]
fn test_ssh_url_to_string_with_port() {
    let url = "ssh://git@github.com:22/hbons/Twinkle".to_string();
    let url = url.parse::<SshUrl>().unwrap();

    assert_eq!(url.to_string_with_port(), url.to_string_standard());


    let url = "ssh://git@github.com/hbons/Twinkle".to_string();
    let url = url.parse::<SshUrl>().unwrap();

    assert_eq!(url.to_string_with_port(), url.to_string_alternate());
}


#[test]
fn test_ssh_url_from_str_alternate() {
    let url = "git@github.com:hbons/Twinkle".to_string();
    let url = url.parse::<SshUrl>().unwrap();

    assert_eq!(url.form, SshUrlType::Alternate);
    assert_eq!(url.user, "git");
    assert_eq!(url.host, "github.com");
    assert_eq!(url.port, None);
    assert_eq!(url.path, PathBuf::from("hbons/Twinkle"));
}


#[test]
fn test_ssh_url_to_string_alternate() {
    let url = SshUrl {
        form: SshUrlType::Alternate,
        user: "git".to_string(),
        host: "github.com".to_string(),
        port: None,
        path: PathBuf::from("hbons/Twinkle"),
        original: "git@github.com:hbons/Twinkle".to_string()
    };

    assert_eq!(url.to_string(), url.original);


    let url = SshUrl {
        form: SshUrlType::Alternate,
        user: "git".to_string(),
        host: "github.com".to_string(),
        port: Some(22),
        path: PathBuf::from("hbons/Twinkle"),
        original: "git@github.com:hbons/Twinkle".to_string()
    };

    assert_eq!(url.to_string(), url.original);
}


// #[test] // TODO
// fn test_ssh_url_from_str_standard_ipv6() {
//     let url = "ssh://git@[2001:adb8:85a4:0000:0000:8a1e:0e70:7034]/hbons/Twinkle".to_string();
//     let url = url.parse::<SshUrl>().unwrap();

//     assert_eq!(url.form, SshUrlType::Standard);
//     assert_eq!(url.user, "git");
//     assert_eq!(url.host, "[2001:0db8:85a3:0000:0000:8a2e:0370:7334]");
//     assert_eq!(url.port, None);
//     assert_eq!(url.path, PathBuf::from("hbons/Twinkle"));
// }
