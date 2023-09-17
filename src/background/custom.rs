use std::path;

// Check if the background file exists
pub fn verify_custom_background(background_path: &str) -> String {

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

    return String::from(path.to_str().unwrap());
}
