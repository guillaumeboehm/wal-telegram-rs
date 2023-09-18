use std::collections::HashMap;
use std::io::Write;
use std::rc::Rc;

use core::cell::RefCell;
use rand::RngCore;

use crate::color::Color;

fn process_output(output_str: &Option<&str>) -> std::path::PathBuf {
    let default_filename = "wal_telegram.tdesktop-theme";
    let mut default_path = std::path::PathBuf::new();

    if let Ok(xdg_dirs) = xdg::BaseDirectories::new() {
        default_path.push(xdg_dirs.get_cache_home());
        default_path.push("wal-telegram");
    }
    else {
        // Problem initializing xdg_dirs, try manually
        default_path.push("~/.cache/wal-telegram");
    };

    default_path.push(default_filename);

    let mut output_path;

    if let Some(output_str) = output_str {
        output_path = std::path::PathBuf::new();
        output_path.push(output_str);

        // If it's a dir create the theme file in there with default filename
        if output_path.as_path().is_dir() {
            output_path.push(default_filename);
        }
    }
    else {
        // If it's None use default
        output_path = default_path;
    }

    // Will fail if permissions are denied
    output_path.try_exists().expect(format!("The output path {} cannot be probed, permissions are probably the issue.", output_path.display()).as_str());

    // Warning prompt
    if output_path.exists() {
        if output_path.is_dir() {
            panic!("The output path {} already exists as a directory", output_path.display());
        }
        else {
            if std::env::var("WAL_TELEGRAM_YES_ALL").is_ok_and(|var| { var == "TRUE" }) {
                // TODO: Panic if no
                println!("Prompt it's gonna delete you sure ?");
            }
        }
    }

    return output_path;
}

fn write_entire_file(writer: &mut zip::ZipWriter<std::fs::File>, data: &[u8]) -> std::io::Result<()> {
    let mut bytes_written = 0;

    while bytes_written < data.len() {
        let remaining_data = &data[bytes_written..];
        let written = writer.write(remaining_data)?;

        bytes_written += written;
    }

    Ok(())
}

// - output is the wanted output or None for default path
// - input_background is the path to the image, needs to be copied and renamed based on tiled (Can
// be assumed as existing)
// - input_palette is the HashMap precomputed with the palette colors from either wal or the custom
// palette
pub fn package_theme(output: Option<&str>, input_background: &str, tiled: bool, input_palette: &HashMap<String, Rc<RefCell<Color>>>) {

    let output_path = process_output(&output);

    // The directory where to manipulate the theme before final output
    let mut tmp_dir = std::env::temp_dir();
    tmp_dir.push("wal-telegram/");

    let mut rand_folder = String::new();
    if let Ok(epoch_time) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        rand_folder.push_str((String::from(epoch_time.as_secs().to_string()) + "_").as_str());
    }
    rand_folder.push_str(rand::thread_rng().next_u32().to_string().as_str());

    tmp_dir.push(rand_folder);
    match std::fs::create_dir_all(&tmp_dir) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error trying to create temporary directory '{}'", &tmp_dir.display());
            panic!("Error: {}", err);
        }
    };

    // Tmp files
    let bg_filename = String::from(if tiled { "tiled" } else { "background" }) + ".jpg";
    let mut tmp_bg = tmp_dir.clone();
    tmp_bg.push(&bg_filename);

    let mut tmp_palette = tmp_dir.clone();
    tmp_palette.push("colors.tdesktop-theme");

    let mut tmp_theme = tmp_dir.clone();
    tmp_theme.push("wal_telegram.tdesktop-theme");

    // Copy the background to the tmp folder
    // BUG: Try to read the byte slice directly with 'image'
    // Couldn't make it work for some reason
    let bg_contents;
    match std::fs::read(&input_background) {
        Ok(contents) => {
            bg_contents = contents;
        }
        Err(err) => {
            eprintln!("Error trying to read contents of bg file '{}'", &tmp_bg.display());
            panic!("Error: {}", err);
        }
    };
    // INFO: Trying to make it an image to verify it's validity
    let bg_img = image::load_from_memory(&bg_contents.as_slice())
                            .expect(format!("Could not recognize background file '{}' as an image", tmp_bg.display()).as_str());
    bg_img.save_with_format(&tmp_bg, image::ImageFormat::Jpeg).expect("Couldn't save the tmp background file to the tmp folder");

    // Generate the colors
    let telegram_colors = super::colors::get_telegram_colors(&input_palette);
    match std::fs::write(&tmp_palette, &telegram_colors) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error trying to write the temporary telegram palette to '{}'", &tmp_palette.display());
            panic!("Error: {}", err);
        }
    }

    // Zip it in theme
    match std::fs::File::create(&tmp_theme) {
        Ok(file) => {
            let mut zip_writer = zip::ZipWriter::new(file);

            let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated).unix_permissions(0o755);

            match zip_writer.start_file(tmp_bg.file_name().unwrap().to_str().unwrap(), options) {
                Ok(_) => { 
                    match write_entire_file(&mut zip_writer, bg_contents.as_slice()) {
                        Ok(_) => { }
                        Err(err) => {
                            eprintln!("Error trying to write '{}' to the temporary telegram theme zip '{}'", &tmp_bg.display(), &tmp_theme.display());
                            panic!("Error: {}", err);
                        }
                    };
                }
                Err(err) => {
                    eprintln!("Error trying to start file '{}' in the temporary telegram theme zip '{}'", &tmp_bg.display(), &tmp_theme.display());
                    panic!("Error: {}", err);
                }
            };

            match zip_writer.start_file(tmp_palette.file_name().unwrap().to_str().unwrap(), options) {
                Ok(_) => {
                    match write_entire_file(&mut zip_writer, telegram_colors.as_bytes()) {
                    // match zip_writer.write(telegram_colors.as_bytes()) {
                        Ok(_) => { }
                        Err(err) => {
                            eprintln!("Error trying to write '{}' to the temporary telegram theme zip '{}'", &tmp_palette.display(), &tmp_theme.display());
                            panic!("Error: {}", err);
                        }
                    };
                }
                Err(err) => {
                    eprintln!("Error trying to start file '{}' in the temporary telegram theme zip '{}'", &tmp_palette.display(), &tmp_theme.display());
                    panic!("Error: {}", err);
                }
            };

            match zip_writer.finish() {
                Ok(_) => { }
                Err(err) => {
                    eprintln!("Error finishing writing to the temporary telegram theme zip '{}'", &tmp_theme.display());
                    panic!("Error: {}", err);
                }
            };
        }

        Err(err) => {
            eprintln!("Error trying creating the temporary telegram theme zip file '{}'", &tmp_theme.display());
            eprintln!("Error: {}", err);
        }
    };

    // Copy the zip to output_path
    match std::fs::create_dir_all(&output_path.parent().unwrap()) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error trying to create directories to '{}'", &output_path.parent().unwrap().display());
            panic!("Error: {}", err);
        }
    };
    match std::fs::copy(&tmp_theme, &output_path) {
        Ok(_) => { }
        Err(err) => {
            eprintln!("Error trying to copy '{}' to '{}'", &tmp_theme.display(), &output_path.display());
            panic!("Error: {}", err);
        }
    };
}

