//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::fmt;
use std::path::PathBuf;
use std::str;

use serde::ser::{ Serialize, Serializer };
use serde::de::{ Deserialize, Deserializer };


#[derive(Clone, Debug, Default)]
pub struct SshUrl {
    pub original: String,
    pub form: SshUrlType,
    pub user: String,
    pub host: String,
    pub port: Option<u16>,
    pub path: PathBuf,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum SshUrlType {
    #[default] Standard,
    Alternate,
}


impl str::FromStr for SshUrl {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.starts_with("ssh://") {
            Self::from_string_standard(s)
        } else {
            Self::from_string_alternate(s)
        }
    }
}


impl SshUrl {
    // ssh://git@github.com/hbons/Twinkle
    // ssh://git@github.com:22/hbons/Twinkle
    pub fn from_string_standard(s: &str) -> Result<Self, Box<dyn Error>> {
        let s = s.strip_prefix("ssh://").ok_or("No 'ssh://' found")?;
        let (user, host_and_path) = s.split_once('@').ok_or("No 'user@' found")?;
        let (host_and_port, path) = host_and_path.split_once('/').ok_or("No '/' found")?;

        if user.is_empty() {
            return Err("No user found".into());
        }

        let (host, port) = match host_and_port.split_once(':') {
            Some((host, port)) => (host, Some(port.parse::<u16>()?)),
            None => (host_and_port, None),
        };

        Ok(SshUrl {
            original: s.to_string(),
            form: SshUrlType::Standard,
            user: user.to_string(),
            host: host.to_string(),
            port,
            path: PathBuf::from(path),
        })
    }


    // git@github.com:hbons/Twinkle
    pub fn from_string_alternate(s: &str) -> Result<Self, Box<dyn Error>> {
        let (user, host_and_path) = s.split_once('@').ok_or("No 'user@' found")?;
        let (host, path) = host_and_path.split_once(':').ok_or("No ':' found")?;

        if user.is_empty() {
            return Err("No user found".into());
        }

        Ok(SshUrl {
            original: s.to_string(),
            form: SshUrlType::Alternate,
            user: user.to_string(),
            host: host.to_string(),
            port: None,
            path: PathBuf::from(path),
        })
    }
}


impl fmt::Display for SshUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self.form {
            SshUrlType::Standard  => self.to_string_standard(),
            SshUrlType::Alternate => self.to_string_alternate(),
        };

        write!(f, "{}", output)
    }
}

impl SshUrl {
    // ssh://git@github.com/hbons/Twinkle
    // ssh://git@github.com:22/hbons/Twinkle
    pub fn to_string_standard(&self) -> String {
        let port_str =
            match self.port {
                Some(port) => format!(":{}", port),
                None       => String::new(),
            };

        format!("ssh://{user}@{host}{port}/{path}",
            user = self.user,
            host = self.host,
            port = port_str,
            path = self.path.display())
    }


    // git@github.com:hbons/Twinkle
    pub fn to_string_alternate(&self) -> String {
        format!("{user}@{host}:{path}",
            user = self.user,
            host = self.host,
            path = self.path.display())
    }


    pub fn to_string_with_port(&self) -> String {
        if self.port.is_some() {
            self.to_string_standard()
        } else {
            self.to_string_alternate()
        }
    }
}


impl Serialize for SshUrl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        serializer.serialize_str(&self.original)
    }
}


impl<'de> Deserialize<'de> for SshUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let url =  s.parse::<SshUrl>().map_err(serde::de::Error::custom)?;

        Ok(url)
    }
}
