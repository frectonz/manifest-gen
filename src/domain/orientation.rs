use serde::Serialize;
use std::{
    default::Default,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Debug, Serialize)]
pub enum Orientation {
    Any,
    Natural,
    Landscape,
    LandscapePrimary,
    LandscapeSecondary,
    Portrait,
    PortraitPrimary,
    PortraitSecondary,
}

impl Orientation {
    pub fn all() -> Vec<Self> {
        vec![
            Orientation::Any,
            Orientation::Natural,
            Orientation::Landscape,
            Orientation::LandscapePrimary,
            Orientation::LandscapeSecondary,
            Orientation::Portrait,
            Orientation::PortraitPrimary,
            Orientation::PortraitSecondary,
        ]
    }
}

impl Display for Orientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use Orientation::*;

        match self {
            Any => write!(f, "any"),
            Natural => write!(f, "natural"),
            Landscape => write!(f, "landscape"),
            LandscapePrimary => write!(f, "landscape-primary"),
            LandscapeSecondary => write!(f, "landscape-secondary"),
            Portrait => write!(f, "portrait"),
            PortraitPrimary => write!(f, "portrait-primary"),
            PortraitSecondary => write!(f, "portrait-secondary"),
        }
    }
}

impl Default for Orientation {
    fn default() -> Self {
        Self::Any
    }
}
