use serde::Serialize;
use std::{
    default::Default,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Debug, Serialize)]
pub enum DisplayMode {
    FullScreen,
    Standalone,
    MinimalUI,
    Browser,
}

impl DisplayMode {
    pub fn all() -> Vec<Self> {
        vec![
            DisplayMode::FullScreen,
            DisplayMode::Standalone,
            DisplayMode::MinimalUI,
            DisplayMode::Browser,
        ]
    }
}

impl Display for DisplayMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use DisplayMode::*;

        match self {
            FullScreen => write!(f, "fullscreen"),
            Standalone => write!(f, "standalone"),
            MinimalUI => write!(f, "minimal-ui"),
            Browser => write!(f, "browser"),
        }
    }
}

impl Default for DisplayMode {
    fn default() -> Self {
        Self::Browser
    }
}
