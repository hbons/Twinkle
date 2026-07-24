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
    let result = GitEnvironment::new(path)
        .config_get("remote.origin.url")
        .and_then(|o| o.stdout.parse::<SshUrl>());

    if let Ok(url) = result {
        let nc = Command::new("nc")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .arg("-zv")
            .arg(url.host)
            .arg(url.port.unwrap_or(22).to_string())
            .status();

        match nc {
            Ok(code) if  code.success() => return Ok(Check::Pass(None)),
            Ok(code) if !code.success() => return Ok(Check::Fail(None)),
            _ => return Err("".into()),
        }
    }

    Ok(Check::Fail(None))
}


pub fn is_ssh_host_known(path: &Path) -> Result<Check, Box<dyn Error>> {
    let result = GitEnvironment::new(path)
        .config_get("remote.origin.url")
        .and_then(|o| o.stdout.parse::<SshUrl>());

    if let Ok(url) = result {
        let nc = Command::new("ssh-keygen")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .arg("-F")
            .arg(url.host)
            .status();

        match nc {
            Ok(code) if  code.success() => return Ok(Check::Pass(None)),
            Ok(code) if !code.success() => return Ok(Check::Fail(None)),
            _ => return Err("".into()),
        }
    }

    Ok(Check::Fail(None))

}


pub fn is_ssh_host(path: &Path) -> Result<Check, Box<dyn Error>> {
    let result = GitEnvironment::new(path)
        .config_get("remote.origin.url")
        .and_then(|o| o.stdout.parse::<SshUrl>());

    if let Ok(url) = result {
        let nc = Command::new("nc")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .args(["-w", "3"])
            .arg(url.host)
            .arg(url.port.unwrap_or(22).to_string())
            .status();

        match nc {
            Ok(code) if  code.success() => return Ok(Check::Pass(None)),
            Ok(code) if !code.success() => return Ok(Check::Fail(None)),
            _ => return Err("".into()),
        }
    }

    Ok(Check::Fail(None))
}


fn is_ssh_host_supporting_key_type(path: &Path, key_type: KeyType) -> Result<Check, Box<dyn Error>> {
    let result = GitEnvironment::new(path)
        .config_get("remote.origin.url")
        .and_then(|o| o.stdout.parse::<SshUrl>());

    if let Ok(url) = result {
        return match ssh_keyscan(&url.host, Some(url.port.unwrap_or(22)), key_type) {
            Ok(_)  => Ok(Check::Pass(None)),
            Err(_) => Ok(Check::Missing),
        }
    }

    Ok(Check::Fail(None))
}

pub fn is_ssh_host_supporting_ed25519(path: &Path) -> Result<Check, Box<dyn Error>> {
    is_ssh_host_supporting_key_type(path, KeyType::ED25519)
}

pub fn is_ssh_host_supporting_ecdsa(path: &Path) -> Result<Check, Box<dyn Error>> {
    is_ssh_host_supporting_key_type(path, KeyType::ECDSA)
}

pub fn is_ssh_host_supporting_rsa(path: &Path) -> Result<Check, Box<dyn Error>> {
    is_ssh_host_supporting_key_type(path, KeyType::RSA)
}


// TODO: what if keys in Secrets?
pub fn is_ssh_client_key_known_to_host(path: &Path) -> Result<Check, Box<dyn Error>> {
    let result = GitEnvironment::new(path)
        .config_get("remote.origin.url")
        .and_then(|o| o.stdout.parse::<SshUrl>());

    if let Ok(url) = result {
        let nc = Command::new("ssh")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .arg("-T")
            .args(["-o", "BatchMode=yes"])
            .arg(format!("{}@{}", url.user, url.host))
            .arg(url.port.unwrap_or(22).to_string())
            .arg("exit")
            .status();

        match nc {
            Ok(code) if  code.success() => return Ok(Check::Pass(None)), // TODO: exits 1 on github.com!
            Ok(code) if !code.success() => return Ok(Check::Fail(None)),
            _ => return Err("".into()),
        }
    }

    Ok(Check::Fail(None))
}
