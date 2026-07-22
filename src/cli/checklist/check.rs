//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::fmt;


pub enum Check {
    Pass(Option<String>),
    Fail(Option<String>),
    Missing, //
    // Invalid, //
    // Error, TODO: instead of Result
}

impl fmt::Display for Check {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pass(_) => write!(f, "✓"),
            Self::Missing => write!(f, "?"),
            Self::Fail(_) => write!(f, "!"),
        }
    }
}
