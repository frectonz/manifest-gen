use clap::clap_app;
use std::fs::File;

mod display;
mod icons;
mod logger;
mod manifest;
mod orientation;
mod prompts;

use display::DisplayMode;
use icons::generate_icons;
use manifest::input_manifest;
use orientation::Orientation;

fn main() {
    let matches = clap_app!(app =>
        (version: "1.0")
        (author: "Fraol Lemecha <fraol0912@gmail.com>")
        (about: "Generates a manifest.json for your PWA")
        (@arg IMAGE: +required "the image for manifest icons")
    )
    .get_matches();

    let image_filename = matches.value_of("IMAGE").expect("No image provided.");

    let icons = generate_icons(image_filename);

    let mut manifest = input_manifest();

    manifest.add_icons(icons);

    let file = File::create("manifest/manifest.json").expect("Failed creating manifest.json");

    serde_json::to_writer_pretty(file, &manifest).expect("Failed serializing your inputs");
}
