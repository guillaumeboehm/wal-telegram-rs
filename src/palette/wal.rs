use std::cell::RefCell;
use std::fs;
use std::path;
use std::collections::HashMap;
use std::rc::Rc;

use crate::color::Color;

pub fn import_wal_palette(wal_colors_path: Option<&str>) -> HashMap<String, Rc<RefCell<Color>>> {

    let filepath;
    if wal_colors_path.is_none() {
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
        if wal_colors_path.unwrap().is_empty() {
            panic!("wal color path can not be an empty string");
        }

        let mut path = path::PathBuf::new();
        path.push(wal_colors_path.unwrap());
        path.push("/colors");

        if !path.exists() {
            panic!("Wal 'colors' file not found in path: {}", path.parent().unwrap().display());
        }

        filepath = String::from(path.to_str().expect("Unexpected error, colors filename is not UTF-8"));
    }

    let palette_file = fs::read_to_string(filepath.clone())
        .expect(format!("Unexpected error, Couldn't read the file {}", filepath).as_str());

    let mut palette = HashMap::new();

    // Vector to store the colors before sorting them by luminance
    let mut colors_vec: Vec<(f32, Rc<RefCell<Color>>)> = Vec::new();
    
    let mut color_vec_count = 0;
    for (index, col) in palette_file.lines().into_iter().enumerate() {
        // If one of the main bg or fg colors
        if index == 0 || index == 7 || index == 8 || index == 15 {
            palette.insert(format!("color{index}"), Rc::new(RefCell::new(Color::from_hex(col).unwrap())));
        }
        else {
            let col = Rc::new(RefCell::new(Color::from_hex(col).unwrap()));
            colors_vec.push((col.borrow().get_relative_luminance(), col.clone()));
        }

        // Process the 8 (minus first and last) first colors
        if colors_vec.len() == 6 {
            colors_vec.sort_by_key(|(lum, _)| (lum * 1000000000.0) as u32);
            for (ind, col) in colors_vec.iter().enumerate() {
                palette.insert(format!("color{}", color_vec_count+ind+1), col.1.clone());
            }
            color_vec_count += colors_vec.len() + 2; // Plus first and last color
            colors_vec.clear();
        }
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
