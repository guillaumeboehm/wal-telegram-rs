use std::{cell::RefCell, rc::Rc, collections::HashMap};

use image::ImageBuffer;

use crate::color::Color;

pub mod wal;
pub mod custom;
pub mod blur;
pub mod plain;

pub static BG_FILENAME: &str = "background.jpg";

pub enum Background {
    Plain,
    Wal,
    Custom,
}

pub fn generate_background(tmp_dir: &std::path::PathBuf, background_type: Background, blur: f32, palette: Option<&HashMap<String, Rc<RefCell<Color>>>>, filepath: Option<&str>) {
    match background_type {
        Background::Plain => {
            if palette.is_none() {
                panic!("Palette needed for plain background")
            }
            plain::use_plain_image(tmp_dir, palette.unwrap()["color0"].clone())
        }
        Background::Wal => {
            wal::use_wal_background(tmp_dir, blur, filepath);
        }
        Background::Custom => {
            if filepath.is_none() {
                panic!("Filepath needed for custom background")
            }
            custom::use_custom_background(tmp_dir, blur, filepath.unwrap());
        }
    }

}

pub fn copy_background(tmp_dir: &std::path::PathBuf, blur:f32, bg_str: &str) {
    let mut tmp_bg = tmp_dir.clone();
    tmp_bg.push(&BG_FILENAME);

    // INFO: Trying to make it an image to verify it's validity
    let mut bg_img = image::open(&bg_str)
                            .expect(format!("Could not recognize background file '{}' as an image", bg_str).as_str());

    // Blur if needed
    if blur > 0.0 {
        blur::blur_image(&mut bg_img, blur).expect("Unexpected error, could not blur the image");
    }

    bg_img.save_with_format(&tmp_bg, image::ImageFormat::Jpeg).expect("Couldn't save the tmp background file to the tmp folder");
}

pub fn create_bg(tmp_dir: &std::path::PathBuf, color: Rc<RefCell<Color>>) {
    let mut tmp_bg = tmp_dir.clone();
    tmp_bg.push(&BG_FILENAME);

    let color_u8 = [
        color.borrow().u8_red(),
        color.borrow().u8_green(),
        color.borrow().u8_blue(),
    ];
    let img = ImageBuffer::from_fn(50, 50, move |_,_| {
        image::Rgb(color_u8)
    });

    // Save image
    img.save(&tmp_bg).expect(format!("Error writing plain background file to '{}'", tmp_bg.display()).as_str());
}
