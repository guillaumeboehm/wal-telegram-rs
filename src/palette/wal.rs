use std::cell::RefCell;
use std::fs;
use std::collections::HashMap;
use std::rc::Rc;

use crate::color::Color;

pub fn import_wal_colors(wal_colors_filepath: Option<&str>) -> HashMap<String, Rc<RefCell<Color>>> {

    let filepath;
    if wal_colors_filepath.is_none() || wal_colors_filepath.unwrap().is_empty() {
        filepath = String::from(xdg::BaseDirectories::new().unwrap().find_cache_file("wal/colors").unwrap().to_str().unwrap());
    }
    else
    {
        filepath = String::from(wal_colors_filepath.unwrap());
    }

    let color_file = fs::read_to_string(filepath.clone())
        .expect(format!("Couldn't read the file {}", filepath).as_str());

    let mut colors = HashMap::new();

    for (index, col) in color_file.lines().into_iter().enumerate() {
        colors.insert(format!("color{index}"), Rc::new(RefCell::new(Color::from_hex(col).unwrap())));
    }
    // Double colors if not wal16
    if colors.len() < 16 {
        let len = colors.len();
        for (index, (_, col)) in colors.clone().into_iter().enumerate() {
            let ind = len + index;
            colors.insert(format!("color{ind}"), col.clone());
        }
    }

    return colors;
}
