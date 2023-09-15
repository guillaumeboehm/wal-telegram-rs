use std::cell::RefCell;
use std::fs;
use std::collections::HashMap;
use std::rc::Rc;

use crate::color::Color;

pub fn import_wal_palette(wal_palette_filepath: &str) -> HashMap<String, Rc<RefCell<Color>>> {

    let filepath;
    if wal_palette_filepath.is_empty() {
        filepath = String::from(xdg::BaseDirectories::new().unwrap().find_cache_file("wal/colors").unwrap().to_str().unwrap());
    }
    else
    {
        filepath = String::from(wal_palette_filepath);
    }

    let palette_file = fs::read_to_string(filepath.clone())
        .expect(format!("Couldn't read the file {}", filepath).as_str());

    let mut palette = HashMap::new();

    for (index, col) in palette_file.lines().into_iter().enumerate() {
        palette.insert(format!("color{index}"), Rc::new(RefCell::new(Color::from_hex(col).unwrap())));
    }
    // Double colors if not wal16
    if palette.len() < 16 {
        let len = palette.len();
        for (index, (_, col)) in palette.clone().into_iter().enumerate() {
            let ind = len + index;
            palette.insert(format!("color{ind}"), col.clone());
        }
    }

    return palette;
}
