//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hi@planetpeanut.uk)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fs;
use std::path::Path;

use crate::log;

use crate::ssh::keygen::ssh_keygen;
use crate::ssh::keys::host_key::HostKey;
use crate::ssh::keys::key_pair::KeyPair;
use crate::ssh::keys::key_type::KeyType;
use crate::ssh::objects::url::SshUrl;


pub fn twinkle_keypair_new(host: &String, key_type: KeyType, keys_dir: &Path) -> Result<KeyPair, Box<dyn Error>> {
    let key_name = format!("{}.{}.key", host, key_type);
    let key_path = keys_dir.join(&key_name);

    let key_pair = ssh_keygen(&key_path, KeyType::default(), 256)?;
    log::info(&format!("Keys | Created key `{}`", key_pair.private_key_path.to_string_lossy()));

    Ok(key_pair)
}


pub fn twinkle_keypair_for(host: &String, key_type: KeyType, keys_dir: &Path) -> Result<KeyPair, Box<dyn Error>> {
    let key_name = format!("{}.{}.key", host, key_type);
    let key_path = keys_dir.join(&key_name);

    if !key_path.exists() {
        let key_pair = twinkle_keypair_new(host, key_type, keys_dir)?;
        Ok(key_pair)
    } else {
        let key_pair = KeyPair::from_file(&key_path)?;
        log::info(&format!("Keys | Found key `{}`", key_pair.private_key_path.to_string_lossy()));
        Ok(key_pair)
    }
}


pub fn twinkle_keypair_renew(host: &String, key_type: KeyType, keys_dir: &Path) -> Result<KeyPair, Box<dyn Error>> {
    let key_pair = twinkle_keypair_for(host, key_type, keys_dir)?;
    twinkle_keypair_delete(&key_pair)?;
    twinkle_keypair_new(host, key_type, keys_dir)
}


pub fn twinkle_keypair_delete(key_pair: &KeyPair) -> Result<(), Box<dyn Error>>{
    fs::remove_file(&key_pair.private_key_path)?;
    fs::remove_file(&key_pair.public_key_path)?;
    log::info(&format!("Keys | Deleted key `{}`", key_pair.private_key_path.to_string_lossy()));
    Ok(())
}


pub fn twinkle_hostkey_for(ssh_url: &SshUrl, key_type: KeyType, keys_dir: &Path) -> Result<HostKey, Box<dyn Error>> {
    let mut host_key = HostKey {
        host: ssh_url.host.clone(),
        ..Default::default()
    };

    let key_path = keys_dir.join(host_key.to_file_name());

    if key_path.exists() {
        host_key = HostKey::from_file(&key_path)?;
        log::info(&format!("Keys | Found trusted host key `{}`", key_path.to_string_lossy()));
    } else {
        host_key = HostKey::for_host(ssh_url, key_type)?;
    }

    // log::debug_struct(&host_key);
    Ok(host_key)
}


pub fn twinkle_hostkey_trust(host_key: &HostKey, keys_dir: &Path) -> Result<(), Box<dyn Error>> {
    let key_name = host_key.to_file_name();
    let key_path = keys_dir.join(key_name);

    fs::write(&key_path, host_key.to_string())?;
    log::info(&format!("Keys | Trusted host key `{}`", key_path.to_string_lossy()));

    Ok(())
}
