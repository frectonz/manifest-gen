use image::{imageops::FilterType, io::Reader as ImageReader, DynamicImage, ImageError};
use std::{error::Error, fmt::Display, fs::create_dir, io::Error as IoError};

use crate::domain::Icon;

const SIZES: [u32; 8] = [72, 96, 128, 144, 152, 192, 384, 512];

pub struct IconGenerator;

impl IconGenerator {
    fn read_image(filename: &str) -> Result<DynamicImage, IconGenerationError> {
        Ok(ImageReader::open(filename)?.decode()?)
    }

    fn make_dirs() -> Result<(), IconGenerationError> {
        create_dir("manifest")?;
        create_dir("manifest/icons")?;
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

            resized_image.save(&name)?;
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
    IoError(IoError),
    ImageError(ImageError),
}

impl From<IoError> for IconGenerationError {
    fn from(e: IoError) -> Self {
        Self::IoError(e)
    }
}

impl From<ImageError> for IconGenerationError {
    fn from(e: ImageError) -> Self {
        Self::ImageError(e)
    }
}

impl Display for IconGenerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => writeln!(f, "{}", e),
            Self::ImageError(e) => writeln!(f, "{}", e),
        }
    }
}

impl Error for IconGenerationError {}
