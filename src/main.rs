use clap::clap_app;
use std::fs::File;

use manifest_gen::interactors::{IconGenerator, ManifestBuilder};

fn main() {
    let matches = clap_app!(app =>
        (version: "1.0")
        (author: "Fraol Lemecha <fraol0912@gmail.com>")
        (about: "Generates a manifest.json for your PWA")
        (@arg IMAGE: +required "the image for manifest icons")
    )
    .get_matches();

    let filename = matches.value_of("IMAGE").expect("No image provided");

    let icons = IconGenerator::generate_icons(filename).expect("Failed to generate icon");
    let manifest = ManifestBuilder::build_manifest(icons);

    let file = File::create("manifest/manifest.json").expect("Failed creating manifest.json");
    serde_json::to_writer_pretty(file, &manifest).expect("Failed serializing your inputs");
}
