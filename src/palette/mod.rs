mod wal;
mod custom;

use std::{collections::HashMap, rc::Rc, cell::RefCell};

use crate::color::Color;

pub static PALETTE_FILENAME: &str = "colors.tdesktop-theme";

pub enum Palette {
    Wal,
    Custom,
}

pub fn generate_palette(palette_type: Palette, filepath: Option<&str>) -> HashMap<String, Rc<RefCell<Color>>> {
    let palette;
    match palette_type {
        Palette::Wal => {
            palette = wal::import_wal_palette(filepath);
        }
        Palette::Custom => {
            if filepath.is_none() {
                panic!("Filepath with custom palette needs a path")
            }
            palette = custom::import_custom_palette(filepath.unwrap());
        }
    }

    // Generate contrasts
    super::color::compute_contrasts(&palette);

    return palette;
}
