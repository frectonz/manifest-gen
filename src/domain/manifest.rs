use serde::Serialize;

use super::{DisplayMode, Icon, Orientation};

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
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn short_name(mut self, short_name: String) -> Self {
        self.short_name = short_name;
        self
    }

    pub fn theme_color(mut self, theme_color: String) -> Self {
        self.theme_color = theme_color;
        self
    }

    pub fn background_color(mut self, background_color: String) -> Self {
        self.background_color = background_color;
        self
    }

    pub fn display(mut self, display: DisplayMode) -> Self {
        self.display = display;
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn scope(mut self, scope: String) -> Self {
        self.scope = scope;
        self
    }

    pub fn start_url(mut self, start_url: String) -> Self {
        self.start_url = start_url;
        self
    }

    pub fn icons(mut self, icons: Vec<Icon>) -> Self {
        self.icons = icons;
        self
    }
}
