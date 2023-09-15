use std::cell::RefCell;
use std::fs;
use std::collections::HashMap;
use std::rc::Rc;

use regex::Regex;

use crate::color::Color;

pub fn import_custom_palette(filepath: &str) -> HashMap<String, Rc<RefCell<Color>>> {

    if filepath.is_empty() {
        panic!("Palette filename is empty");
    }

    let palette_file = fs::read_to_string(filepath.clone())
        .expect(format!("Couldn't read the palette file: {}", filepath).as_str());

    let mut palette = HashMap::new();

    let re = Regex::new(r##"\s*color(?<ind>\d+)\s*=\s*['"]#(?<hex>(?:\d{6}|\d{8}))['"]\s*"##).unwrap(); // Needs ##" to allow # in the string

    let mut max_index = 0;
    for (num, line) in palette_file.lines().into_iter().enumerate() {

        if let Some(color) = re.captures(line) {
            max_index = max_index.max(color["ind"].parse().expect("Unexpected error, the color index is not an number for some reason."));
            palette.insert(format!("color{}", &color["ind"]), Rc::new(RefCell::new(Color::from_hex(&color["hex"]).unwrap())));
        };
    }

    if max_index != (palette.len() - 1) {
        panic!("The palette has a color count problem, make sure to start your colors at 'color0' and that there is no missing color.");
    }
    if palette.len() < 16 {
        panic!("The palette has {} colors but needs at least 16.", palette.len());
    }

    return palette;
}
