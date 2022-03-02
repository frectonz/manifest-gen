use anyhow::Result as AnyhowResult;
use clap::clap_app;
use std::fs::File;

use manifest_gen::interactors::{IconGenerator, ManifestBuilder};

fn main() -> AnyhowResult<()> {
    let matches = clap_app!(app =>
        (name: "manifest-gen")
        (version: "1.0")
        (author: "Fraol Lemecha <fraol0912@gmail.com>")
        (about: "Generates a manifest.json for your PWA")
        (@arg IMAGE: +required "the image for manifest icons")
    )
    .get_matches();

    // we can safely unwrap because the argument is required
    let filename = matches.value_of("IMAGE").unwrap();

    let icons = IconGenerator::generate_icons(filename)?;
    let manifest = ManifestBuilder::build_manifest(icons);

    let file = File::create("manifest/manifest.json")?;
    serde_json::to_writer_pretty(file, &manifest)?;

    println!("🎉 Successfully created the manifest");

    Ok(())
}
