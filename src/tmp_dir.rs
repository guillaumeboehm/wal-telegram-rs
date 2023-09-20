use rand::RngCore;

pub fn create_tmp_dir() -> std::path::PathBuf {
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

    return tmp_dir;
}
