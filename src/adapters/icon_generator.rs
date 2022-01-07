use image::{imageops::FilterType, io::Reader as ImageReader};
use std::fs::create_dir;

use crate::domain::Icon;

const SIZES: [u32; 8] = [72, 96, 128, 144, 152, 192, 384, 512];

pub struct IconGenerator;

impl IconGenerator {
    pub fn generate_icons(filename: &str) -> Result<Vec<Icon>, IconGenerationError> {
        use IconGenerationError::*;
        let mut icons: Vec<Icon> = Vec::with_capacity(SIZES.len());

        let img = ImageReader::open(filename)
            .or(Err(ImageReadError))?
            .decode()
            .or(Err(ImageDecodeError))?;

        create_dir("manifest").or(Err(ManifestDirCreationError))?;
        create_dir("manifest/icons").or(Err(ManifestIconsDirCreationError))?;

        for size in SIZES {
            let mut icon = Icon::default();

            icon.set_src(format!("/icons/icon-{0}x{0}.png", size));
            icon.set_sizes(format!("{0}x{0}", size));

            let name = format!("manifest/icons/icon-{0}x{0}.png", size);
            let resized_image = img.resize(size, size, FilterType::Nearest);

            resized_image.save(&name).or(Err(ImageSaveError))?;
            icons.push(icon);
        }

        Ok(icons)
    }
}

#[derive(Debug)]
pub enum IconGenerationError {
    ImageReadError,
    ImageSaveError,
    ImageDecodeError,
    ManifestDirCreationError,
    ManifestIconsDirCreationError,
}
