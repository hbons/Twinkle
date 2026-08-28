//   Twinkle, automatic syncing with Git
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use super::common::COMMON_APP_NAME;


pub fn twinkle_default_info_attributes() -> Vec<String> {
    // Docs: https://git-scm.com/docs/gitattributes

    vec![
        format!("# For {COMMON_APP_NAME} conflict resolution"),
        "* merge=binary".into(),
    ]
}


pub fn twinkle_default_info_exclude() -> Vec<&'static str>
{
    // Docs: https://git-scm.com/docs/gitignore

    vec![
        "*.autosave", // Various autosaving apps
        ".*.sw[a-z]", "*.un~", "*.swp", "*.swo", // vi(m)
        "*~", // Emacs
        ".~lock.*", // LibreOffice
        "*.part", "*.crdownload", // Firefox and Chromium
        "*.kate-swp", // Kate
        ".directory", // Dolphin
        ".DS_Store", "Icon\r", "._*", ".Spotlight-V100", ".Trashes", // macOS
    ]
}
