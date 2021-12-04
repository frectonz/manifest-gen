use crate::icons::Icon;
use crate::prompts::{ask_for_select, ask_for_string};
use crate::{DisplayMode, Orientation};
use serde::Serialize;

const APP_NAME: &str = "app name";
const SHORT_APP_NAME: &str = "short name";

const THEME_COLOR: &str = "#fff";
const BACKGROUND_COLOR: &str = "#fff";

const DISPLAY_MODE: DisplayMode = DisplayMode::Browser;
const ORIENTATION: Orientation = Orientation::Any;

const SCOPE: &str = "/";
const START_URL: &str = "/";

#[derive(Default, Debug, Serialize)]
pub struct Manifest {
    name: String,
    short_name: String,

    theme_color: String,
    background_color: String,

    display: DisplayMode,
    orientation: Orientation,

    scope: String,
    start_url: String,

    icons: Vec<Icon>,
}

impl Manifest {
    pub fn add_icons(&mut self, icons: Vec<Icon>) {
        self.icons = icons;
    }
}

pub fn input_manifest() -> Manifest {
    let mut manifest = Manifest::default();

    let display_options = vec![
        DisplayMode::FullScreen,
        DisplayMode::Standalone,
        DisplayMode::MinimalUI,
        DisplayMode::Browser,
    ];

    let orientation_options = vec![
        Orientation::Any,
        Orientation::Natural,
        Orientation::Landscape,
        Orientation::LandscapePrimary,
        Orientation::LandscapeSecondary,
        Orientation::Portrait,
        Orientation::PortraitPrimary,
        Orientation::PortraitSecondary,
    ];

    manifest.name = ask_for_string("What's your app's name?", APP_NAME);
    manifest.short_name = ask_for_string("What's your app's short name?", SHORT_APP_NAME);
    manifest.theme_color = ask_for_string("What's your app's theme color?", THEME_COLOR);
    manifest.background_color =
        ask_for_string("What's your app's background color?", BACKGROUND_COLOR);
    manifest.display = ask_for_select(
        "What's your app's display mode?",
        display_options,
        DISPLAY_MODE,
    );
    manifest.orientation = ask_for_select(
        "What's your app's orientation mode?",
        orientation_options,
        ORIENTATION,
    );
    manifest.scope = ask_for_string("What's your app's url scope?", SCOPE);
    manifest.start_url = ask_for_string("What's your app's start URL?", START_URL);

    manifest
}
