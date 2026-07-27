//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::env;
use std::io::{ BufRead, BufReader };
use std::net::{ TcpStream, ToSocketAddrs };
use std::path::Path;
use std::process::{ Command, Stdio };
use std::time::Duration;

use crate::git::objects::environment::GitEnvironment;
use crate::ssh::keys::key_type::KeyType;
use crate::ssh::keyscan::ssh_keyscan;
use crate::ssh::objects::url::SshUrl;

use super::outcome::Outcome;


// Secure Shell

pub fn is_ssh_agent_running(_path: &Path) -> Outcome {
    match env::var("SSH_AUTH_SOCK") {
        Ok(v)  => Outcome::Pass(Some(v.to_string())),
        Err(_) => Outcome::Fail(None),
    }
}

pub fn is_ssh_agent_has_keys(_path: &Path) -> Outcome {
    let ssh = Command::new("ssh-add")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .arg("-l")
        .output();

    match ssh {
        Ok(o) if o.status.success() => {
            let line_count = BufReader::new(&o.stdout[..])
                .lines()
                .count()
                .to_string();

            Outcome::Pass(Some(line_count))
        },
        Ok(o) if !o.status.success() => Outcome::Missing,
        _ => Outcome::Error,
    }
}


// Connectivity

pub fn is_ssh_host_reachable(path: &Path) -> Outcome {
    let result = GitEnvironment::new(path)
        .config_get("remote.origin.url")
        .and_then(|o| o.stdout.parse::<SshUrl>());

    if let Ok(url) = result {
        if check_host_port(&url.host, url.port.unwrap_or(22), 3) {
            return Outcome::Pass(Some(url.host));
        }
    }

    Outcome::Fail(None)
}


pub fn is_ssh_host_known(path: &Path) -> Outcome {
    let result = GitEnvironment::new(path)
        .config_get("remote.origin.url")
        .and_then(|o| o.stdout.parse::<SshUrl>());

    if let Ok(url) = result {
        let ssh = Command::new("ssh-keygen")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .arg("-F")
            .arg(url.host)
            .status();

        match ssh {
            Ok(status) if  status.success() => return Outcome::Pass(None),
            Ok(status) if !status.success() => return Outcome::Fail(None),
            _ => return Outcome::Error,
        }
    }

    Outcome::Fail(None)

}


pub fn is_ssh_host(path: &Path) -> Outcome {
    let result = GitEnvironment::new(path)
        .config_get("remote.origin.url")
        .and_then(|o| o.stdout.parse::<SshUrl>());

    if let Ok(url) = result {
        if check_host_port(&url.host, url.port.unwrap_or(22), 3) {
            return Outcome::Pass(Some(format!("{}", url.port.unwrap_or(22))));
        }
    }

    Outcome::Fail(None)
}


pub fn is_ssh_host_supporting_ed25519(path: &Path) -> Outcome {
    is_ssh_host_supporting_key_type(path, KeyType::ED25519)
}

pub fn is_ssh_host_supporting_ecdsa(path: &Path) -> Outcome {
    is_ssh_host_supporting_key_type(path, KeyType::ECDSA)
}

pub fn is_ssh_host_supporting_rsa(path: &Path) -> Outcome {
    is_ssh_host_supporting_key_type(path, KeyType::RSA)
}

fn is_ssh_host_supporting_key_type(path: &Path, key_type: KeyType) -> Outcome {
    let result = GitEnvironment::new(path)
        .config_get("remote.origin.url")
        .and_then(|o| o.stdout.parse::<SshUrl>());

    if let Ok(url) = result {
        return match ssh_keyscan(&url.host, Some(url.port.unwrap_or(22)), key_type) {
            Ok(_)  => Outcome::Pass(Some(format!("{:?}", key_type))),
            Err(_) => Outcome::Missing,
        }
    }

    Outcome::Fail(None)
}


pub fn is_ssh_client_key_known_to_host(path: &Path) -> Outcome {
    // TODO: What if keys stored in Secrets when running in Flatpak?

    let result = GitEnvironment::new(path)
        .config_get("remote.origin.url")
        .and_then(|o| o.stdout.parse::<SshUrl>());

    if let Ok(url) = result {
        let ssh = Command::new("ssh")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .arg("-T")
            .args(["-o", "BatchMode=yes"])
            .arg(format!("{}@{}", url.user, url.host))
            .arg(url.port.unwrap_or(22).to_string())
            .arg("exit")
            .status();

        match ssh {
            Ok(status) => {
                if status.success() {
                    return Outcome::Pass(None);
                }

                match status.code() {
                    Some(n) => {
                        if  url.host == "github.com" && n == 1 {
                            return Outcome::Pass(None)
                        } else {
                            return Outcome::Fail(None)
                        }
                    },
                    None => return Outcome::Pass(None),
                };
            },
            Err(_) => return Outcome::Error,
        }
    }

    Outcome::Fail(None)
}


fn check_host_port(host: &str, port: u16, timeout_secs: u64) -> bool {
    format!("{}:{}", host.trim(), port)
        .to_socket_addrs()
        .ok()
        .into_iter()
        .flatten()
        .any(|address|
            TcpStream::connect_timeout(
                &address,
                Duration::from_secs(timeout_secs)
            ).is_ok()
        )
}
