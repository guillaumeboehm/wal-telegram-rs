use std::fs;
use std::path;

// Try to fetch the background image from 'wal' file in wal cache
pub fn use_wal_background(tmp_dir: &std::path::PathBuf, blur: f32, wal_colors_path: Option<&str>) {

    let filepath;
    if wal_colors_path.is_none() || wal_colors_path.unwrap().is_empty() {
        // Try to get wal from the default location
        if let Ok(xdg_dirs) = xdg::BaseDirectories::new() {
            filepath = String::from(xdg_dirs.find_cache_file("wal/wal")
                                            .expect(format!("Wal's 'wal' file not found in default location: {}", xdg_dirs.get_cache_home().display()).as_str()).to_str()
                                            .expect("Unexpected error, colors filename is not UTF-8"));
        }
        else {
            // Problem initializing xdg_dirs, try manually
            let path = path::Path::new("~/.cache/wal/wal");

            if !path.exists() {
                panic!("Wal's 'wal' file not found in default location: {}", path.parent().unwrap().display());
            }

            filepath = String::from(path.to_str().expect("Unexpected error, colors filename is not UTF-8"));
        };
    }
    else
    {
        let mut path = path::PathBuf::new();
        path.push(wal_colors_path.unwrap());
        path.push("/wal");

        if !path.exists() {
            panic!("Wal's 'wal' file not found in path: {}", path.parent().unwrap().display());
        }

        filepath = String::from(path.to_str().expect("Unexpected error, colors filename is not UTF-8"));
    }

    let wal_str = fs::read_to_string(filepath.clone())
        .expect(format!("Unexpected error, Couldn't read the file {}", filepath).as_str());

    let path = path::Path::new(wal_str.as_str());

    if !path.is_file() {
        if !path.exists() {
            panic!("Couldn't find the wallpaper file at '{}'", filepath);
        }
        else {
            panic!("'{}' used as background image is a directory and not a file", filepath);
        }
    }

    super::copy_background(tmp_dir, blur, path.to_str().unwrap());
}

