//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::env;
use std::path::Path;
use std::process::{ Command, Stdio };

use crate::git::objects::environment::GitEnvironment;
use crate::ssh::keys::key_type::KeyType;
use crate::ssh::keyscan::ssh_keyscan;
use crate::ssh::objects::url::SshUrl;

use super::check::Check;


// Secure Shell

pub fn is_ssh_agent_running(_path: &Path) -> Result<Check, Box<dyn Error>> {
    match env::var("SSH_AUTH_SOCK") {
        Ok(v)  => Ok(Check::Pass(Some(v.to_string()))),
        Err(_) => Ok(Check::Fail(None)),
    }
}

pub fn is_ssh_agent_has_keys(_path: &Path) -> Result<Check, Box<dyn Error>> {
    let ssh = Command::new("ssh-add")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg("-l")
        .status();

    match ssh {
        Ok(code) if  code.success() => Ok(Check::Pass(None)), // TODO: Pass number of keys
        Ok(code) if !code.success() => Ok(Check::Fail(None)),
        _ => Err("".into()),
    }
}


// Connectivity

pub fn is_ssh_host_reachable(path: &Path) -> Result<Check, Box<dyn Error>> {
    let url = GitEnvironment::new(path).config_get("remote.origin.url");

    // dbg!(&url.unwrap());
    let nc = Command::new("nc")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg("-zv")
        .arg(&url.unwrap().stdout) // TODO
        .arg("80") // TODO
        .status();

    match nc {
        Ok(code) if  code.success() => Ok(Check::Pass(None)),
        Ok(code) if !code.success() => Ok(Check::Fail(None)),
        _ => Err("".into()),
    }
}


pub fn is_ssh_host_known(_path: &Path) -> Result<Check, Box<dyn Error>> {
    Ok(Check::Pass(None)) // TODO: ssh-keygen -F codeberg.org
}


pub fn is_ssh_host(_path: &Path) -> Result<Check, Box<dyn Error>> {
    let nc = Command::new("nc")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .args(["-w",  "3"]) // timeout, TODO: Test on Linux
        .arg("notify.sparkleshare.org") // TODO: use remote.origin.url and get port
        .arg("22")
        .status();

    match nc {
        Ok(code) if  code.success() => Ok(Check::Pass(None)), // TODO: Pass remote SSH version string?
        Ok(code) if !code.success() => Ok(Check::Fail(None)),
        _ => Err("".into()),
    }
}


// TODO: Condense these 3 into 1
pub fn is_ssh_host_supporting_ed25519(path: &Path) -> Result<Check, Box<dyn Error>> {
    let result = GitEnvironment::new(path)
        .config_get("remote.origin.url")
        .and_then(|o| o.stdout.parse::<SshUrl>());

    if let Ok(url) = result {
        return match ssh_keyscan(&url.host, Some(url.port.unwrap_or(22)), KeyType::ED25519) {
            Ok(_)  => Ok(Check::Pass(None)),
            Err(_) => Ok(Check::Missing),
        }
    }

    Ok(Check::Fail(None))
}

pub fn is_ssh_host_supporting_ecdsa(path: &Path) -> Result<Check, Box<dyn Error>> {
    let result = GitEnvironment::new(path)
        .config_get("remote.origin.url")
        .and_then(|o| o.stdout.parse::<SshUrl>());

    if let Ok(url) = result {
        return match ssh_keyscan(&url.host, Some(url.port.unwrap_or(22)), KeyType::ECDSA) {
            Ok(_)  => Ok(Check::Pass(None)),
            Err(_) => Ok(Check::Missing),
        }
    }

    Ok(Check::Fail(None))
}

pub fn is_ssh_host_supporting_rsa(path: &Path) -> Result<Check, Box<dyn Error>> {
    let result = GitEnvironment::new(path)
        .config_get("remote.origin.url")
        .and_then(|o| o.stdout.parse::<SshUrl>());

    if let Ok(url) = result {
        return match ssh_keyscan(&url.host, Some(url.port.unwrap_or(22)), KeyType::RSA) {
            Ok(_)  => Ok(Check::Pass(None)),
            Err(_) => Ok(Check::Missing),
        }
    }

    Ok(Check::Fail(None))
}


pub fn is_ssh_client_key_known_to_host(_path: &Path) -> Result<Check, Box<dyn Error>> {
    // let url = "ssh://debian@notify.sparkleshare.org/fsdfds".parse::<SshUrl>(); // TODO: strip /path from remote origin url

    let ssh = Command::new("ssh")
        .stdout(Stdio::null())
    .stderr(Stdio::null())
    .arg("-T")
    // .arg("-p")
    // .arg("22") // TODO
    .args(["-o", "BatchMode=yes"])
    .arg("debian@notify.sparkleshare.org") // TODO
    .arg("exit")
    .status();

    // dbg!("ssh://debian@notify.sparkleshare.org/fsdfds".parse::<SshUrl>().unwrap().to_string_alternate());

    match ssh {
        Ok(code) if  code.success() => Ok(Check::Pass(None)),
        Ok(code) if !code.success() => Ok(Check::Fail(None)),
        _ => Err("".into()),
    }

    // TODO: what if keys in Secrets?
}
