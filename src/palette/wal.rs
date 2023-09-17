use std::cell::RefCell;
use std::fs;
use std::path;
use std::collections::HashMap;
use std::rc::Rc;

use crate::color::Color;

pub fn import_wal_palette(wal_colors_path: &str) -> HashMap<String, Rc<RefCell<Color>>> {

    let filepath;
    if wal_colors_path.is_empty() {
        // Try to get wal from the default location
        if let Ok(xdg_dirs) = xdg::BaseDirectories::new() {
            filepath = String::from(xdg_dirs.find_cache_file("wal/colors")
                                            .expect(format!("Wal 'colors' file not found in default location: {}", xdg_dirs.get_cache_home().display()).as_str()).to_str()
                                            .expect("Unexpected error, colors filename is not UTF-8"));
        }
        else {
            // Problem initializing xdg_dirs, try manually
            let path = path::Path::new("~/.cache/colors");

            if !path.exists() {
                panic!("Wal 'colors' file not found in default location: {}", path.parent().unwrap().display());
            }

            filepath = String::from(path.to_str().expect("Unexpected error, colors filename is not UTF-8"));
        };
    }
    else
    {
        let mut path = path::PathBuf::new();
        path.push(wal_colors_path);
        path.push("/colors");

        if !path.exists() {
            panic!("Wal 'colors' file not found in path: {}", path.parent().unwrap().display());
        }

        filepath = String::from(path.to_str().expect("Unexpected error, colors filename is not UTF-8"));
    }

    let palette_file = fs::read_to_string(filepath.clone())
        .expect(format!("Unexpected error, Couldn't read the file {}", filepath).as_str());

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
