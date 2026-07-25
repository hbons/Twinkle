//   Twinkle, automatic syncing with Git
//   Copyright (C) 2026  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::fmt;


pub enum Outcome {
    Error,
    Fail(Option<String>),
    Missing,
    Pass(Option<String>),
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error   => write!(f, "!"),
            Self::Fail(_) => write!(f, "!"),
            Self::Missing => write!(f, "?"),
            Self::Pass(_) => write!(f, "✓"),
        }
    }
}
