use std::path;

// Check if the background file exists
pub fn use_custom_background(tmp_dir: &std::path::PathBuf, blur: f32, background_path: &str) {

    let path;
    if background_path.is_empty() {
        panic!("The path to the background file can not be empty");
    }
    else
    {
        path = path::Path::new(background_path);

        if !path.is_file() {
            panic!("The file '{}' does not exist or is not a file", path.to_str().unwrap());
        }
    }

    super::copy_background(&tmp_dir, blur, path.to_str().unwrap());
}
