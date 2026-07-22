//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::env;
use std::path::Path;
use std::process::{ Command, Stdio };

use crate::ssh::keys::key_type::KeyType;
use crate::ssh::keyscan::ssh_keyscan;

use super::checklist::Check;


// Secure Shell

pub fn is_ssh_agent_running(_path: &Path) -> Result<Check, Box<dyn Error>> {
    match env::var("SSH_AUTH_SOCK") {
        Ok(_) => Ok(Check::Pass(None)),
        _ => Ok(Check::Fail(None)),
    }
}

pub fn is_key_added_to_agent(_path: &Path) -> Result<Check, Box<dyn Error>> {
    let ssh = Command::new("ssh-add")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg("-L")
        .status();

    match ssh {
        Ok(code) if  code.success() => Ok(Check::Pass(None)),
        Ok(code) if !code.success() => Ok(Check::Fail(None)),
        _ => Err("".into()),
    }
}


// Connectivity

pub fn is_host_reachable(_path: &Path) -> Result<Check, Box<dyn Error>> {
    let nc = Command::new("nc")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg("-zv")
        .arg("1.1.1.1") // TODO: use remote.origin.url
        .arg("80") // TODO
        .status();

    match nc {
        Ok(code) if  code.success() => Ok(Check::Pass(None)),
        Ok(code) if !code.success() => Ok(Check::Fail(None)),
        _ => Err("".into()),
    }
}


// TODO: is_host_known: ssh-keygen -F github.com


pub fn is_host_using_ssh(_path: &Path) -> Result<Check, Box<dyn Error>> {
    let nc = Command::new("nc")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg("-zv")
        .arg("-G 3") // timeout, TODO: Test on Linux
        .arg("notify.sparkleshare.org") // TODO: use remote.origin.url and get port
        .arg("22")
        .status();

    match nc {
        Ok(code) if  code.success() => Ok(Check::Pass(None)),
        Ok(code) if !code.success() => Ok(Check::Fail(None)),
        _ => Err("".into()),
    }
}


pub fn is_host_supporting_ed25519(_path: &Path) -> Result<Check, Box<dyn Error>> {
    // TODO: use remote.origin.url and get port
    match ssh_keyscan("notify.sparkleshare.org", Some(22), KeyType::ED25519) {
        Ok(_)  => Ok(Check::Pass(None)),
        Err(_) => Ok(Check::Missing),
    }
}

pub fn is_host_supporting_ecdsa(_path: &Path) -> Result<Check, Box<dyn Error>> {
    // TODO: use remote.origin.url and get port
    match ssh_keyscan("notify.sparkleshare.org", Some(22), KeyType::ECDSA) {
        Ok(_)  => Ok(Check::Pass(None)),
        Err(_) => Ok(Check::Missing),
    }
}

pub fn is_host_supporting_rsa(_path: &Path) -> Result<Check, Box<dyn Error>> {
    // TODO: use remote.origin.url and get port
    match ssh_keyscan("notify.sparkleshare.org", Some(22), KeyType::RSA) {
        Ok(_)  => Ok(Check::Pass(None)),
        Err(_) => Ok(Check::Missing),
    }
}



pub fn is_client_key_known_to_host(_path: &Path) -> Result<Check, Box<dyn Error>> {
    // let url = "ssh://debian@notify.sparkleshare.org/fsdfds".parse::<SshUrl>(); // TODO: strip /path from remote origin url

    let ssh = Command::new("ssh")
        .stdout(Stdio::null())
    .stderr(Stdio::null())
    .arg("-T")
    // .arg("-p")
    // .arg("22") // TODO
    .arg("-o BatchMode=yes")
    .arg("debian@notify.sparkleshare.org")
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
