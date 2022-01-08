use image::{imageops::FilterType, io::Reader as ImageReader, DynamicImage};
use std::fs::create_dir;

use crate::domain::Icon;

const SIZES: [u32; 8] = [72, 96, 128, 144, 152, 192, 384, 512];

pub struct IconGenerator;

use IconGenerationError::*;
impl IconGenerator {
    fn read_image(filename: &str) -> Result<DynamicImage, IconGenerationError> {
        ImageReader::open(filename)
            .or(Err(ImageReadError))?
            .decode()
            .or(Err(ImageDecodeError))
    }

    fn make_dirs() -> Result<(), IconGenerationError> {
        create_dir("manifest").or(Err(ManifestDirCreationError))?;
        create_dir("manifest/icons").or(Err(ManifestIconsDirCreationError))?;
        Ok(())
    }

    fn make_icons(img: DynamicImage) -> Result<Vec<Icon>, IconGenerationError> {
        let mut icons: Vec<Icon> = Vec::with_capacity(SIZES.len());

        for size in SIZES {
            let icon = Icon::new(
                format!("/icons/icon-{0}x{0}.png", size),
                format!("{0}x{0}", size),
            );

            let name = format!("manifest/icons/icon-{0}x{0}.png", size);
            let resized_image = img.resize(size, size, FilterType::Nearest);

            resized_image.save(&name).or(Err(ImageSaveError))?;
            icons.push(icon);
        }

        Ok(icons)
    }

    pub fn generate_icons(filename: &str) -> Result<Vec<Icon>, IconGenerationError> {
        let img = IconGenerator::read_image(filename)?;
        IconGenerator::make_dirs()?;
        let icons = IconGenerator::make_icons(img)?;
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
