use anyhow::Result as AnyhowResult;
use clap::Parser;
use std::fs::File;

use manifest_gen::interactors::{IconGenerator, ManifestBuilder};

/// Generates a manifest.json for your PWA
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    image: String,
}

fn main() -> AnyhowResult<()> {
    let args = Args::parse();

    let icons = IconGenerator::generate_icons(&args.image)?;
    let manifest = ManifestBuilder::build_manifest(icons);

    let file = File::create("manifest/manifest.json")?;
    serde_json::to_writer_pretty(file, &manifest)?;

    println!("🎉 Successfully created the manifest");

    Ok(())
}
