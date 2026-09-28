//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use crate::git::objects::id::GitId;


#[test]
fn test_object_id_from_str() {
    let hash = "527e638c780bb4af2dda5244f2f657bf5029e596";
    let result = hash.parse::<GitId>();

    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str(), hash);


    let hash = "0bfc1eaa3e39f6ea40ef7c2c6ed8f4605df0019d5cb36129dd2dd667db1277fc";
    let result = hash.parse::<GitId>();

    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str(), hash);


    let hash = "";
    let result = hash.parse::<GitId>();

    assert!(result.is_err());


    let hash = "527e638c780bb4af2dda5244f2f657bf5029e59"; // Invalid length
    let result = hash.parse::<GitId>();

    assert!(result.is_err());


    let hash = "0bfc1eaa3e39f6ea40ef7c2c6ed8f4605df0019d5cb36129dd2dd667db1277fc1"; // Invalid length
    let result = hash.parse::<GitId>();

    assert!(result.is_err());


    let hash = "😊😊😊😊😊😊😊😊😊😊"; // Invalid characters
    let result = hash.parse::<GitId>();

    assert!(result.is_err());
}
