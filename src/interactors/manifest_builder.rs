use crate::{
    constants::{
        APP_NAME, BACKGROUND_COLOR, DISPLAY_MODE, ORIENTATION, SCOPE, SHORT_APP_NAME, START_URL,
        THEME_COLOR,
    },
    domain::{DisplayMode, Icon, Manifest, Orientation},
};

use super::helpers::{ask_for_select, ask_for_string};

pub struct ManifestBuilder;

impl ManifestBuilder {
    pub fn build_manifest(icons: Vec<Icon>) -> Manifest {
        let display_options = DisplayMode::all();
        let orientation_options = Orientation::all();

        Manifest::default()
            .name(ask_for_string("What's your app's name?", APP_NAME))
            .short_name(ask_for_string(
                "What's your app's short name?",
                SHORT_APP_NAME,
            ))
            .theme_color(ask_for_string(
                "What's your app's theme color?",
                THEME_COLOR,
            ))
            .background_color(ask_for_string(
                "What's your app's background color?",
                BACKGROUND_COLOR,
            ))
            .display(ask_for_select(
                "What's your app's display mode?",
                display_options,
                DISPLAY_MODE,
            ))
            .orientation(ask_for_select(
                "What's your app's orientation mode?",
                orientation_options,
                ORIENTATION,
            ))
            .scope(ask_for_string("What's your app's url scope?", SCOPE))
            .start_url(ask_for_string("What's your app's start URL?", START_URL))
            .icons(icons)
    }
}
