use argh::FromArgs;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Clone)]
pub enum Quality {
    Both,
    Major,
    Minor,
}

impl Quality {
    pub fn random() -> Self {
        if rand::random() {
            Self::Major
        } else {
            Self::Minor
        }
    }
}

impl FromStr for Quality {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "M" | "major" | "maj" => Ok(Quality::Major),
            "m" | "minor" | "min" => Ok(Quality::Minor),
            _ => Ok(Quality::Both),
        }
    }
}

impl Display for Quality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Quality::Both => write!(f, "Both"),
            Quality::Major => write!(f, "Major"),
            Quality::Minor => write!(f, "Minor"),
        }
    }
}

fn default_quality() -> Quality {
    Quality::Both
}

#[derive(FromArgs, Debug)]
/// Music key options
pub struct Opts {
    /// whether accidentals should be used
    #[argh(switch, short = 'a')]
    pub accidentals: bool,

    /// the key quality to use: major, minor, both (default)
    #[argh(option, short = 'q', default = "default_quality()")]
    pub quality: Quality,
}
