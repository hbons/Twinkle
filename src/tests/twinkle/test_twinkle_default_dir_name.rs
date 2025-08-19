//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::path::Path;

use crate::ssh::objects::url::SshUrl;
use crate::twinkle::twinkle_util::twinkle_default_dir_name;


#[test]
fn test_twinkle_default_dir_name_with_repo() {
    let url = "ssh://git@github.com/hbons/Twinkle".parse::<SshUrl>().unwrap();
    let result = twinkle_default_dir_name(&url);

    assert_eq!(result.unwrap(), Path::new("Twinkle").to_path_buf());


    let url = "ssh://git@github.com/hbons".parse::<SshUrl>().unwrap();
    let result = twinkle_default_dir_name(&url);

    assert_eq!(result.unwrap(), Path::new("hbons").to_path_buf());
}
