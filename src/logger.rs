pub struct Logger;

impl Logger {
    pub fn reading_file(image_name: &str) {
        println!("{}", format!("Reading file: {}", image_name));
    }

    pub fn resizing_image(image_name: &str, size: u32) {
        println!("{}", format!("Resizing {0} to {1}x{1}", image_name, size));
    }

    pub fn saving_image(image_name: &str) {
        println!("{}", format!("Saving {}", image_name));
    }
}
