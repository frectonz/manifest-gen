use image::{imageops::FilterType, io::Reader as ImageReader};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::fs::create_dir;

use crate::logger::Logger;

const SIZES: [u32; 8] = [72, 96, 128, 144, 152, 192, 384, 512];

#[derive(Debug)]
pub struct Icon {
    pub src: String,
    pub sizes: String,
}

impl Serialize for Icon {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Icon", 3)?;
        state.serialize_field("src", &self.src)?;
        state.serialize_field("sizes", &self.sizes)?;
        state.serialize_field("type", "image/png")?;
        state.end()
    }
}

pub fn generate_icons(image_filename: &str) -> Vec<Icon> {
    let mut icons: Vec<Icon> = Vec::with_capacity(SIZES.len());
    Logger::reading_file(&image_filename);

    let img = ImageReader::open(&image_filename)
        .expect("Failed to read the image")
        .decode()
        .expect("Failed to decode the image");

    create_dir("manifest").expect("Failed to create manifest directory");
    create_dir("manifest/icons").expect("Failed to create icons directory");

    for size in SIZES {
        let mut icon = Icon {
            src: String::new(),
            sizes: String::new(),
        };

        Logger::resizing_image(&image_filename, size);
        let resized_image = img.resize(size, size, FilterType::Nearest);

        let name = format!("manifest/icons/icon-{0}x{0}.png", size);
        icon.src = format!("/icons/icon-{0}x{0}.png", size);
        icon.sizes = format!("{0}x{0}", size);

        Logger::saving_image(&name);
        resized_image
            .save(&name)
            .expect(format!("Failed to save {}", &name).as_str());
        icons.push(icon);
    }

    icons
}
